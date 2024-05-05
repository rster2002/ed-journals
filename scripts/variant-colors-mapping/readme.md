# Variant color mapping

Quick script to turn columns from
[Canonn exobio](https://docs.google.com/spreadsheets/d/15lqZtqJk7B2qUV5Jb4tlnst6i1B7pXlAUzQnacX64Kc/edit#gid=0) into
match statements.

## Usage

Create a `input.txt` file and paste the 'Name' and 'Journal Name' columns in there. Run the script using:

```shell
bun index.js
```

This will generate a new file called `output.txt` with the match statements.
