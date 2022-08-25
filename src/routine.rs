use crate::parallel::ParallelComputeOptions;

pub fn routine(data: Vec<u64>, iterations: u64) -> Vec<u64>
where
{
  let compute = move |mut item: u64| {
    let mut iter = 0;
    loop {
      if iter == iterations {
        return item;
      }
      if item == 1 {
        return iter;
      }

      if item % 2 == 0 {
        item /= 2;
      } else {
        item = item * 3 + 1;
      }
      iter += 1;
    }
  };

  crate::parallel::parallel_compute(data, compute, ParallelComputeOptions::default())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_check() {
    let arr = vec![1, 2, 3, 100];
    let result = routine(arr, 8);
    assert_eq!(result, vec![0, 1, 7, 88])
  }

  #[test]
  fn zero() {
    let arr = vec![];
    let result = routine(arr, 8);
    assert_eq!(result, vec![])
  }

  #[test]
  fn zero_el() {
    let arr = vec![0];
    let result = routine(arr, 8);
    assert_eq!(result, vec![0])
  }

  #[test]
  fn bigger_check() {
    let arr = vec![1, 2, 3, 100, 1, 2, 3, 100, 1, 2, 3, 100, 1, 2, 3, 100];
    let result = routine(arr, 8);
    assert_eq!(result, vec![0, 1, 7, 88, 0, 1, 7, 88,0, 1, 7, 88, 0, 1, 7, 88,])
  }
}
