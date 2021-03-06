# The (Rust) super tiny compiler

This project is a rust version of the super tiny compiler ([the original one (JS) was created by Jamie Kyle](https://github.com/jamiebuilds/the-super-tiny-compiler/blob/master/the-super-tiny-compiler.js)).

The output (C-like) and input (Lisp-like) syntax are the same. The Lexer and Parser created here were heavily inspired by the super-tiny-compiler.js.

## Examples

```sh
cargo run --example tokens
cargo run --example ast
cargo run --example compiler
```

## Test

```sh
cargo test
```

## License

[MIT](./LICENSE)
