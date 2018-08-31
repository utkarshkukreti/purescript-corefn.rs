# purescript-corefn.rs

A parser for [PureScript](https://github.com/purescript/purescript)'s corefn
JSON representation.

## Usage

The library exposes one function:

    pub fn from_str(str: &str) -> Result<Module, serde_json::Error>

and a bunch of structs and enums representing the corefn structure.

See [docs](https://docs.rs/purescript-corefn) for more details.
