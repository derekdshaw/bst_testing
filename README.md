# Binary Search Tree Example

This code implements a simple binary search tree using C++, Rust, and Go. I wrote this code out of curiosity, and for edification. 

## Implementations

Each implementation is a separate directory. The C++ implementations use ref counted pointers for children. This was done so that cleanup of the tree after being deleted would be automatic. Where the Go implementation does not need ref counted pointers, the children are stored in a slice. The rust version stores the nodes in a Vector. This stores the nodes in a contiguous block of memory and cuts down on memory allocations. The nodes are released as part of the vector on drop.

Tests are used in order to validate the implementations. As well as test some performance benchmarks were also added where possible.

To run the tests/benchmarks

Rust
```powershell
cargo test
cargo bench --profile release-performance
```

Go
```powershell
go test
go test -bench=BenchmarkDeleteFromLargeBST -benchmem
go test -bench=BenchmarkBuildLargeBST -benchmem
```

C++

Just build the release version and run it. The tests will ouput timings. Not quite as good as the benchmarks but gets the point across.

## Conclusion

This was a fun project. I learned a bit more about the performance and usage of the Rust borrow checker. In the Rust version the performace of the Insert method was really bad when not using a RefCell container around the child nodes. I tried various approaches, see merges in git, I landed on a vector to store the nodes and node references a indexes. This cut down on the number of memory allocations and seems to be the fastest version I could create.

Feel free to reach out if you have any questions or observations.
