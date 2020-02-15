# entry

[![Build Status](https://travis-ci.org/battesonb/entry.svg?branch=master)](https://travis-ci.org/battesonb/entry)
[![](http://meritbadge.herokuapp.com/entry)](https://crates.io/crates/entry)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A simple command-line note-taking tool.

### Installation

Simply run the following:

```sh
cargo install entry
```

Make sure that you have cargo installed and that cargo is in your path:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

### Examples

The entry command-line tool currently has three main subcommands.

```sh
$ entry find # find entries that contain the given text.
```

```sh
$ entry new # create a new note under the default entry name.
```

```sh
$ entry config # configure the entry tool with get, set and list subcommands.
```

Each of these subcommands has optional arguments and can be viewed with -h (or
--help).

### Contributing

Feel free to make pull requests. I've been using conventional commits, but I
have not yet set up formal contributing guidelines. This is my first time
working with Rust, so I imagine there are some issues with it.

