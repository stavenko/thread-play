use super::options::ParallelComputeOptions;

pub fn parallel_compute<T, R, F>(
  input: Vec<T>,
  compute: F,
  options: ParallelComputeOptions,
) -> Vec<R>
where
  F: Fn(T) -> R + Send + 'static + Clone,
  R: Send + 'static,
  T: Send + 'static + Sync + Clone,
{
  let max_size = options.max_single_thread_size.unwrap_or(8);
  if input.len() > max_size {
    let mut children = Vec::new();
    for chunk in input.chunks(max_size) {
      let options = options.clone();
      let compute = compute.clone();
      let chunk: Vec<T> = chunk.into();
      children.push(std::thread::spawn(move || {
        parallel_compute(chunk, compute, options)
      }));
    }
    children
      .into_iter()
      .flat_map(|handle| handle.join().unwrap_or_default())
      .collect()
  } else {
    input.into_iter().map(compute).collect()
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn preserve_order_default() {
    let arr = vec![1, 2, 3, 4];
    let result = parallel_compute(arr, |n| n + 1, ParallelComputeOptions::default());
    assert_eq!(result, vec![2, 3, 4, 5]);
  }

  #[test]
  fn preserve_order() {
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
