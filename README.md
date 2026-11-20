# Rust for Laravel, PHP, and JavaScript Developers

This edition starts with ownership, modeling valid states, and deciding what
failures a caller needs to handle. It assumes you already ship applications.
One short Cargo and syntax chapter replaces the introductory programming tour.

**[Start reading](php-laravel-js/src/start.md)** ·
[All chapters](php-laravel-js/src/SUMMARY.md) ·
[Job audit project](php-laravel-js/job-audit)

The examples draw on PHP DTOs, Laravel jobs and service contracts, and
JavaScript collection methods and promises. The capstone is a Rust CLI that
validates a job export and reports counts. It runs on stable Rust 1.90 or newer
and uses the standard library.

```sh
cargo +stable run --manifest-path php-laravel-js/job-audit/Cargo.toml -- php-laravel-js/job-audit/sample.jobs
```

You can read the chapters directly on GitHub or use [mdBook 0.5][mdbook-guide]
for a searchable local edition.

```sh
mdbook serve php-laravel-js --open
```

[mdbook-guide]: https://rust-lang.github.io/mdBook/

The chapter sequence draws on *The Rust Programming Language* by Steve Klabnik,
Carol Nichols, and the Rust community, and on the Cognitive Engineering Lab's
experimental edition. The original edition's build instructions follow.

## The Rust Programming Language (Experimental Edition)

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

The original experimental edition of *The Rust Programming Language* supports
interactive features such as quizzes.

For issues with the [Brown-hosted edition](https://rust-book.cs.brown.edu/), see
the [Cognitive Engineering Lab's issue tracker](https://github.com/cognitive-engineering-lab/rust-book/issues).

[The book is available in dead-tree form from No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

You can also read the book for free online. Please see the book as shipped with
the latest [stable], [beta], or [nightly] Rust releases. Be aware that issues
in those versions may have been fixed in this repository already, as those
releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

See the [releases] to download just the code of all the code listings that appear in the book.

[releases]: https://github.com/rust-lang/book/releases

## Requirements

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

This fork also requires a few mdBook preprocessors to support our experimental extensions. Follow the installation instructions at each link below.

* `mdbook-aquascope`: <https://github.com/cognitive-engineering-lab/aquascope#installation>
* `mdbook-quiz`: <https://github.com/cognitive-engineering-lab/mdbook-quiz#installation>

You should install the same version of each preprocessor [used in CI](https://github.com/cognitive-engineering-lab/rust-book/blob/main/.github/workflows/main.yml).

Finally, you need [pnpm](https://pnpm.io/installation).

The book also uses two mdbook plugins which are part of this repository. If you
do not install them, you will see warnings when building and the output will not
look right, but you *will* still be able to build the book. To use the plugins,
you should run:

```bash
$ cargo install --locked --path packages/mdbook-trpl-listing
$ cargo install --locked --path packages/mdbook-trpl-note
```

## Building

### With cargo-make

If you have [`cargo-make`] installed, then run:

```bash
$ cargo make build
```

### Without cargo-make

First, build the Javascript extensions.

```bash
$ cd js-extensions
$ pnpm init-repo
$ cd ..
```

Then to build the book, type:

```bash
$ mdbook build
```

### Output

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Because the book is [printed][nostarch], and because we want
to keep the online version of the book close to the print version when
possible, it may take longer than you're used to for us to address your issue
or pull request.

So far, we've been doing a larger revision to coincide with [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, we will only be correcting errors. If your issue or pull request
isn't strictly fixing an error, it might sit until the next time that we're
working on a large revision: expect on the order of months or years. Thank you
for your patience!

### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used the word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).

[`cargo-make`]: https://github.com/sagiegurari/cargo-make
