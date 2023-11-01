# Rust and Lua

Rustcraft is written in Rust (surprise), which has some important differences with Lua in implementations of things like strings that must be considered.

## Strings
Strings in Lua can contain null characters, whereas Rust strings cannot. If your string has a null anywhere but the last character, it will be replaced with the Unicode replacement character `U+FFFD`, which looks like this: `�` When Rust compares strings, `AB` is different to `A�B`. Keep this in mind, since content identifiers use strings.

This behavior may change in future. It's best to avoid having nulls in the first place. See [here](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy) for more on this behavior within Rust.