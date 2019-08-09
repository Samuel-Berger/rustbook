# Rustbook

Learn Rust via [the Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html) book.
Everything will be done via *NIX operating systems.
See the book to adopt the notes for a Windows enviroment.

[Learn Rust in Y minutes](https://learnxinyminutes.com/)
is also a nice source for notes.

## Installation and Maintainance of Rust

Install and update path: `curl https://sh.rustup.rs -sSf | sh`
and `source $HOME/.cargo/env`. Keep up to date by running `rustup update`.

Install the automatic formatter (same as linter?) via `rustup`:
`rustup component add rustfmt`. The formatter is invoked by

## Basic

Format the code, compile and run:

```rust
rustfmt main.rs && rustc main.rs && ./main
```

`fn main()` is always needed.

A macros i denoted by !. So `println!` calls a macro,
and `println` calls a function.
