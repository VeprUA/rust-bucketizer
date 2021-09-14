# Rust Bucketizer

Create for slotting numerical values into buckets.
To do this create a `Bucketizer` and add your buckets to it,
then use the `.bucketize()` method to get back the bucket a value fits into.

# Example
```
use rust_bucketize::Bucketizer;

let b = Bucketizer::new()
    .bucket(Some(10.0), Some(20.0), 15.0)
    .bucket(Some(5.0), Some(10.0), 7.5)
    .bucket(None, Some(4.0), 0.0);

assert_eq!(b.bucketize(12.34), Some(15.0));
assert_eq!(b.bucketize(999.99), None);
```

### Premise
This is small project created from a tutorial to keep as a reference for building well documented and tested rust crates. 


### Library Documentation
To see the documentation, run the following command 

```
cargo doc --open
```

### Takeaways
- Test driven development works! Writing tests before implementation allows the developer to think about the edge cases first and test out how the API will work. We often think of a solution first and hack it away which can lead to badly written or hard to understand API. 
- Rust comes with impressive documentation tools. All the documentation gets written by the developer next to the code, so write once and accessible everywhere.
