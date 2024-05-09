# ED Journals

> **Warn** this project is currently in beta, which means that it is very much work in progress. Breaking
> changes are likely to happen.

Work in progress parsing and utilities for working with journal files from Elite Dangerous, with specific emphasis on
creating structs and enums to make working with journal entries a lot easier and predictable.

## Features and plans

This is a list of some of the current features and some of the features I really want to have added before a 1.0
release.

- [x] Models for journal directory and files.
- [x] Journal reader to read entries from journal files.
- [x] Serde models for journal entries.
- [ ] Parsing dynamic files like `Market.json`, `Status.json` etc.
- [ ] Async variants of models, especially the `JournalReader`.
- [ ] Automatic journal detection based on platform etc.
- [ ] Exobiology utilities for things like predicting available species based on planet scan etc.
- [ ] Exploration utilities for pointing out special or unusual discoveries.
