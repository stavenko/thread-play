mod options;
#[cfg(all(not(feature="with_tokio"), not(feature="with_rayon")))]
mod std;
#[cfg(all(not(feature="with_tokio"), not(feature="with_rayon")))]
pub use self::std::parallel_compute;

#[cfg(all(feature="with_rayon", not(feature="with_tokio")))]
mod rayon;
#[cfg(all(feature="with_rayon", not(feature="with_tokio")))]
pub use self::rayon::parallel_compute;
#[cfg(all(feature="with_tokio", not(feature="with_rayon")))]
mod tokio;
#[cfg(all(feature="with_tokio", not(feature="with_rayon")))]
pub use self::tokio::parallel_compute;

pub use options::ParallelComputeOptions;

