mod parallel;
mod routine;

fn main() {
  #[cfg(all(feature="with_tokio", not(feature="with_rayon")))]
  println!("Using with tokio runtime and some futures library staff");
  #[cfg(all(feature="with_rayon", not(feature="with_tokio")))]
  println!("Using with rayon super easy implementation");
  #[cfg(all(not(feature="with_tokio"), not(feature="with_rayon")))]
  println!("Use manual implementation on standart threads and some copying");

  let sample_data = vec![1, 2, 3, 100];
  let expected_result = vec![0, 1, 7, 88];

  let actual_result = routine::routine(sample_data, 8);

  println!("Expected: {:?}", expected_result);
  println!("Actual: {:?}", actual_result);
}
