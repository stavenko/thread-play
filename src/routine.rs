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

      if item & 1 == 0 {
        item /= 2;
      } else if item > (u64::MAX-1) / 3 {
        println!("Warning: Overloading of max int. But I actually don't have instructions how to handle this");
        println!("Warning: I will not try to create correct overload, without panic");
        let bigger_item: u128 = item as u128; // Fast and dirty. 
        let max = u64::MAX as u128 + 1 ;
        let mut bigger_item = bigger_item * 3 + 1;
        while bigger_item > max {
          bigger_item %= max
        }

        item = bigger_item as u64;
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
  fn some_more_iters() {
    let arr = vec![100];
    let result = routine(arr, 26);
    assert_eq!(result, vec![25])
  }

  #[test]
  fn test_overload() {
    let arr = vec![u64::MAX - 1];
    let result = routine(arr, 619);
    assert_eq!(result, vec![618])
  }

  #[test]
  fn bigger_check() {
    let arr = vec![1, 2, 3, 100, 1, 2, 3, 100, 1, 2, 3, 100, 1, 2, 3, 100];
    let result = routine(arr, 8);
    assert_eq!(
      result,
      vec![0, 1, 7, 88, 0, 1, 7, 88, 0, 1, 7, 88, 0, 1, 7, 88,]
    )
  }
}
