# Realtime programming for Rust

**BuRSt** is a framework for writing realtime software in the Rust
programming language. It enforces static and dynamic guarantees about
the latency of operations. It runs in tiny environments and does not
depend on the Rust standard library.

*NOTE: BuRSt is a proof-of-concept. It is not ready for use.*

## About BuRSt

BuRSt enforces a two-phase programming model, wherein work with
unpredicatble latency is performed in the initial setup phase,
followed by a realtime phase where operations must have known
latencies. All allocation is performed in the setup phase.

It replaces the Rust standard library with a minimal set of
platform-independent components optimized for realtime programming,
including realtime-suitable replacements for the common collection
and synchronization types.

It further offers a selection of platform-specific, and
platform-independent, components built on `burst-core` that are
designed to encourage best practices. It runs on Linux, but is
intended to be compatible with other real-time environments.

[See the example](examples/example1.rs).

No BuRSt crates depend on `std`. The BuRSt framework is a
replacement for `std` for embedded systems.

BuRSt only runs on Rust nightly.

## BuRSt core

`burst-core` defines the BuRSt programming model along with a
number of platform-independent data structures suitable for realtime
programming.

A burst program begins by calling `burst::begin_setup` to retrieve the
singleton *setup token*, `St`. The setup token is passed to all
functions that might allocate, have unpredicatble latency, or might
otherwise cause unpredictable system behavior.

`burst-core` defines the standard box and collection types, including
`Box`, `Rc`, `Arc`, `Vec`, and `HashMap`. These are generally identical
to the `std` types of the same name, except that all operations which
might allocate take a reference to `&St`.

`Vec` and `HashMap`, though their capacities are frozen after the
setup phase, may still be mutated during the realtime phase.

The core has few platform dependencies, mainly the allocator, the
random number generator, and platform-specific facilities for ensuring
real-time operation such as `mlockall`. All platform-specific code is
abstracted into the `burst-core-pal-linux` crate for future
portability.

## BuRSt framework

TODO

## License

MIT/Apache-2.0
