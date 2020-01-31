# gn

This is a Rust implementation of grb's [gn](<https://github.com/garybernhardt/dotfiles/blob/master/bin/gn>) which prints a diff summary.

## Usage

```
$ git diff 'master~10..master' | gn

3559 lines of diff
432 lines (+1249, -817)
1542 words (+5155, -3613)
```

## Requirements

```
$ rustc --version
rustc 1.40.0 (73528e339 2019-12-16)
```

## How to build

```
cargo build --release
```

## Development

The root folder contains `example.diff` file which could be used as an example of git diff company. This way you can test any introduced changes.

```
$ cat example.diff | target/debug/gn
82 lines of diff
58 lines (+58, -0)
193 words (+193, -0)
```
