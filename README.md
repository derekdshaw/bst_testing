# Binary Search Tree Example

This code implements a simple binary search tree using C++, Rust, and Go. I wrote this code out of curiosity, and edification. 

## Implementations

Each implementation is a separate directory. The Rust and Go implementations use ref counted pointers for children. This was done so that cleanup of the tree after being deleted would be automatic. Where the Go implementation does not need ref counted pointers, the children are stored in a slice.

Tests are used in order to validate the implementations. As well as test some performance. Benchmarks were also added where possible. 

## Conclusion

This was a fun project. I learned a bit more about the performance and usage of the Rust borrow checker. As the performace of the Insert method was really bad when not use a RefCell container around the child nodes. 

Feel free to reach out if you have any questions or observations.