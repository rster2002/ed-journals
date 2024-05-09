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

## Contributing

One of the best ways to contribute to this project is by contributing journal logs to the test-suite. Doing this will
allow the project to be more and more accurate. Note that this project currently only focuses on the live version of the
game, so game version 4 and up. 

You can contribute journal files by following these steps:

1. Locate your journal files. On Windows they are saved at `C:\users\<your username>\Saved Games\Frontier Developments\Elite Dangerous`
   On Linux it depends on a number of things. Check our this [forum post](https://forums.frontier.co.uk/threads/solved-linux-journal-files.507750/) for more information.
2. It's probably a good idea to anonymize your logs and remove some of the personal content. You can do so by using the
   [ED logs anonymizer tool](https://rster2002.github.io/ed-log-anonymizer/). Make sure to check some of the exported
   files to ensure everything has been processed correctly.
3. Create a fork of the project and add your journal files to the `test-jounals` directory in the root of the project.
4. Create a PR and wait for it to be merged.
5. Thank you so much!
