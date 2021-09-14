//! `rust-bucketize` is a create for slotting numerical values into buckets.
//! To do this create a `Bucketizer` and add your buckets to it,
//! then use the `.bucketize()` method to get back the bucket a value fits into.
//! 
//! # Example
//! ```
//! use rust_bucketize::Bucketizer;
//! 
//! let b = Bucketizer::new()
//!     .bucket(Some(10.0), Some(20.0), 15.0)
//!     .bucket(Some(5.0), Some(10.0), 7.5)
//!     .bucket(None, Some(4.0), 0.0);
//! 
//! assert_eq!(b.bucketize(12.34), Some(15.0));
//! assert_eq!(b.bucketize(999.99), None);
//! ```
//! 

/// A `Bucketizer` holds the list of buckets you want to slot values into, and does
/// the bucketization operation.
/// 
/// You can create one with `new()` and add buckets with a chained `.bucket()` calls.
/// these calls add buckets which are aevaluated in order. For instance, if you add
/// a bucket from 9 to 100 and then add a bucket from 2 to 50, nothing will ever 
/// get put in that second bucket.
/// 
/// Buckets are min-inclusive and max-exclusive. If a given value matches no bucket,
/// `bucketize` returns `None`.
/// 
/// # Example
/// ```
/// use rust_bucketize::Bucketizer;
/// 
/// let b = Bucketizer::new()
///     .bucket(Some(10.0), Some(20.0), 15.0)
///     .bucket(Some(5.0), Some(10.0), 7.5)
///     .bucket(None, Some(4.0), 0.0);
/// 
/// assert_eq!(b.bucketize(12.34), Some(15.0));
/// assert_eq!(b.bucketize(999.99), None);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>,
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    /// Create a new `Bucketizer` with no buckets configured
    pub fn new() -> Self {
        Bucketizer {
            buckets: Vec::new(),
        }
    }

    pub fn bucket(self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        let mut new = self;
        new.buckets.push((min, max, value));
        new
    }

    pub fn bucketize(&self, input: f64) -> Option<f64> {
        for buck in &self.buckets {
            match *buck {
                (None, None, val) => return Some(val),
                (Some(min), None, val) => {
                    if input >= min { return Some(val)}
                },
                (None, Some(max), val) => {
                    if input < max { return Some(val)}
                },
                (Some(min), Some(max), val) => {
                    if input >= min && input < max { return Some(val) }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let bucketizer = Bucketizer::new().bucket(Some(0.0), Some(1.0), 0.5);

        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(999.999), None);
    }

    #[test]
    fn single_bucket_end_values() {
        let bucketizer = Bucketizer::new().bucket(Some(0.0), Some(1.0), 0.5);

        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }

    #[test]
    fn multiple_buckets_closed_ends() {
        let b = Bucketizer::new()
            .bucket(Some(-1.0), Some(0.0), -0.5)
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(b.bucketize(0.0), Some(0.5));
        assert_eq!(b.bucketize(-0.7), Some(-0.5));
        assert_eq!(b.bucketize(999.99), None);
    }

    #[test]
    fn multiple_buckets_opened_ends() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5)
            .bucket(Some(1.0), None, 1.5);

        assert_eq!(b.bucketize(0.0), Some(0.5));
        assert_eq!(b.bucketize(-0.7), None);
        assert_eq!(b.bucketize(999.99), Some(1.5));
    }
}
