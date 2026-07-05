# ED State

Models for collective incremental (live) state into a single source of truth which makes it easier to build
reason about the data emitted by the game. You don't _need_ this crate though, and you can easily create your own
version, this just provides _one_ implementation for doing this. Feel free to look at the code and just copy the parts
that you like! Builds on top of the [ed-journals](https://crates.io/crates/ed-journals) crate.

## Features and plans

This crate is currently being reworked from the original implementation and not all aspects of the game are covered
yet and is being worked on in the future.

- [x] Shared trait for accepting input from different sources, be it logs of live JSON updates.
- [x] Models for working state on different levels, like commander-, system-, and body-state with more being added in
      the future.
- [ ] Ability to serialize and deserialize state in order to preserve any data sources from live updates.

## Contributing

This crate is intended to cover most aspects of the game and various different gameplay areas like exploration and
combat. Currently, this crate mostly focuses on exploration and exobiology (that's just my gameplay style), but feel
free to [open an issue](https://github.com/rster2002/ed-journals/issues/new) to describe the kind of state that would
be useful to track. We can see if it's a good fit for this implementation or if it would make more sense as a custom
implementation instead.
