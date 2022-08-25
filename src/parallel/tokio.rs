use std::sync::Arc;

use futures::{stream::FuturesOrdered, StreamExt};

use super::options::ParallelComputeOptions;

fn compute_tokio_single<T, R, F>(input: Arc<Vec<T>>, start: usize, end: usize, compute: F) -> Vec<R>
where
  F: Fn(T) -> R + Send + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  input[start..end]
    .iter()
    .map(|item| compute(*item))
    .collect()
}
async fn compute_tokio_single_async<T, R, F>(
  input: Arc<Vec<T>>,
  start: usize,
  end: usize,
  compute: F,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  compute_tokio_single(input, start, end, compute)
}

async fn parallel_compute_tokio_impl<T, R, F>(
  input: Arc<Vec<T>>,
  compute: F,
  max_size: usize,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  let mut futures = FuturesOrdered::new();
  let mut iters = input.len() / max_size;

  for i in 0..iters {
    let start = i * max_size;
    let end = start + max_size;
    futures.push(compute_tokio_single_async(
      input.clone(),
      start,
      end,
      compute.clone(),
    ))
  }

  if iters * max_size < input.len() {
    futures.push(compute_tokio_single_async(
      input.clone(),
      iters * max_size,
      input.len(),
      compute.clone(),
    ))
  }

  let mut result = Vec::new();
  while let Some(fut_result) = futures.next().await {
    result.extend(fut_result.into_iter()); // If it is expected faster implementation, then I would
                                           // like to talk you into implement stream here, instead
                                           // of returning filled container. 
  }
  result
}

pub fn parallel_compute<T, R, F>(
  input: Vec<T>,
  compute: F,
  options: ParallelComputeOptions,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  let max_size = options.max_single_thread_size.unwrap_or(8);
  let input = Arc::new(input);

  if input.len() < max_size {
    let len = input.len();
    compute_tokio_single(input, 0, len, compute)
  } else {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(parallel_compute_tokio_impl(input, compute, max_size))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn preserve_order_tokio_default() {
    let arr = vec![1, 2, 3, 4];
    let result = parallel_compute(arr, |n| n + 1, ParallelComputeOptions::default());
    assert_eq!(result, vec![2, 3, 4, 5]);
  }

  #[test]
  fn preserve_order_tokio() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let result = parallel_compute(
      arr,
      |n| n + 1,
      ParallelComputeOptions {
        max_single_thread_size: Some(2),
      },
    );
    assert_eq!(result, vec![2, 3, 4, 5, 6, 7]);
  }
  #[test]
  fn check_chunks() {
    let arr = vec![1, 2, 3];
    let result = parallel_compute(
      arr,
      |n| n + 1,
      ParallelComputeOptions {
        max_single_thread_size: Some(2),
      },
    );
    assert_eq!(result, vec![2, 3, 4]);
  }
}
