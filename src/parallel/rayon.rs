use super::options::ParallelComputeOptions;

pub fn parallel_compute<T, R, F>(
  input: Vec<T>,
  compute: F,
  options: ParallelComputeOptions,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + Sync + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  parallel_compute_rayon_impl(&input, compute, options)
}

fn parallel_compute_rayon_impl<T, R, F>(
  input: &[T],
  compute: F,
  options: ParallelComputeOptions,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + Sync + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone + Copy,
{
  use rayon::prelude::*;
  let max_size = options.max_single_thread_size.unwrap_or(8);
  if input.len() > max_size {
    input.par_iter().map(|item| compute(*item)).collect()
  } else {
    input.iter().map(|item| compute(*item)).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn preserve_order_rayon_default() {
    let arr = vec![1, 2, 3, 4];
    let result = parallel_compute(arr, |n| n + 1, ParallelComputeOptions::default());
    assert_eq!(result, vec![2, 3, 4, 5]);
  }

  #[test]
  fn preserve_order_rayon() {
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

}
