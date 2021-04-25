# entry

[![Build Status](https://travis-ci.org/battesonb/entry.svg?branch=master)](https://travis-ci.org/battesonb/entry)
[![Crates.io](https://img.shields.io/crates/v/entry)](https://crates.io/crates/entry)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

An interactive CLI for generating JSON-formatted data from a local schema.

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
entry schema # has various subcommands for schemas

$ entry schema new # Interactively generates a new schema
Enter a name for the schema
...

$ entry schema list # lists all saved schemas
migraine

$ entry schema show migraine | jq # displays the given schema
{
  "shape": {
    "activities": {
      "count": "many",
      "data_type": "string"
    },
    "date": {
      "count": "one",
      "data_type": "date"
    },
    "foods": {
      "count": "many",
      "data_type": "string"
    },
    "painScale": {
      "count": "one",
      "data_type": "number"
    }
  }
}

$ entry schema remove migraine # removes a given schema
Successfully removed schema `migraine`
```

```sh
$ entry for migraine # interactively generate a new entry for a given schema
Please provide the activities (array of strings)
...

{date:"2020/04/02",painScale:7,foods:["tomato"],activities:["run"]}
```

```sh
$ entry last # shows the last JSON-formatted entry in case of error
{date:"2020/04/02",painScale:7,foods:["tomato"],activities:["run"]}
```

### Previous versions

If you're looking for the version which simply allows quick note creation:

1. You can build from source for v0.1.2.
2. Use a shell alias/function the `date` command and your preferred editor.
3. Fork from v0.1.2 if you'd like to improve upon that functionality.

I'm no longer supporting the note-taking feature, since I've started preferring
recording notes and data following some schema so that I can graph out the data
over time.

### Contributing

Feel free to make pull requests. I've been using conventional commits, but I
have not yet set up formal contributing guidelines. This is my first time
working with Rust, so I imagine there are some issues with it.
