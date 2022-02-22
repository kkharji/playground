# What does that tell you `Arc<Mutex<T>>`?

When ever you see `Arc<Mutex<T>>`, You right away think of `T` as shared mutably across mulitple
threads, and that cloning it will be light operation because T is wrapped in an **Atomically Reference Counted**.

[Arc] is **Atomically Reference Counted** is a threa-safe reference-counting pointer.

[Mutex] is a mutual exclusion primitive useful for protecting shared data. Mutex and other similar
things are safe abstractions over the unsafe behaviour of shared mutable state across tasks.

See
- [How Arc works in Rust?]
- [Arc]
- [Mutex]


<!-- Links -->
[Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[Mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[How Arc works in Rust?]: https://medium.com/@DylanKerler1/how-arc-works-in-rust-b06192acd0a6
