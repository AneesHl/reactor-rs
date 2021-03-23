# Rust reactors prototype

## Setup

* Download [rustup](https://rustup.rs/) (Rust toolchain manager).
* Install the Rust nightly toolchain:
```shell
rustup install nightly
rustup default nightly
```
This is the unstable rust compiler + stdlib. We use some features that haven't been entirely stabilized yet ([function traits](https://doc.rust-lang.org/nightly/unstable-book/library-features/fn-traits.html#fn_traits))

* Run a program:
```shell
cargo run --bin reflex_game
```

## Benchmarking

```shell
cargo bench --feature benchmarking
```
You can then find plots in `target/criterion/$BENCHMARK_NAME/report` (see [Criterion report structure](https://bheisler.github.io/criterion.rs/book/user_guide/plots_and_graphs.html)).

Note: The `--feature` flag is used by conditional compilation directives (eg to remove some log statements).


## Tour

* See `src/runtime/scheduler.rs` for the scheduler implementation.
* See `benches` for some benchmarks.
* See `src/bin` for some example programs.

The runtime assumes we have a code generator that works as described in the header of `src/bin/reflex_game.rs`. It assumes most of the structural checks have been performed by the code generator and hence elides them.

> **Note:** the crate used to contain a module that does not assume a code generator.
This was scrapped in 11a3ad5. Check it out for ideas about how to implement eg runtime checks.

## Status

* The scheduler is single-threaded and really dumb.
* There's too much contention for the priority queue
* Reactors and reactions needs to be wrapped in Arc (atomic reference counted).
  This smart pointer performs synchronization on access to their value. This is probably mostly useless, because synchronization can in theory be limited to just the event queue.
* Maybe using `async/await` would be nice?
* There are no tests yet... but to test is to doubt right
