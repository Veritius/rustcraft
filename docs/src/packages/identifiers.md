# Content Identifiers

Each piece of content accessed by the [Lua API](../lua/introduction.md) is marked with a "content identifier". Content identifiers are composed of two or three strings separated by colons (`:`) or forward slashes (`/`).

## Anatomy of a content identifier
A content identifier takes the form of `namespace:identifier:variant`.

The `namespace` is the name of the content package the content originates from. For example, blocks added by the engine have the `engine` namespace (such as air/empty space). If you wanted to add a mod named "Dave's Mod" you'd use the namespace `daves_mod`.

The `identifier` is the name of the content being referred to. If you were adding a creature named "Dave", you'd use the identifier `dave`, making the content identifier `daves_mod:dave` or `daves_mod/dave`.

The `variant` is an optional field that is used to identify variations of the same content. If our `dave` creature had two variants, one with a hat and now without a hat, we would have the content identifiers `daves_mod:dave:no_hat` and `daves_mod:dave:hat`.

No fields in a content identifier can have a length of zero (no characters before/after a separator). The `namespace` and `identifier` fields are mandatory, and not having them will cause an error.

## In Lua
Content identifiers can be strings:
```lua
"daves_mod:dave:no_hat"
```
or tables:
```lua
{
    "namespace": "daves_mod",
    "identifier": "dave",
    "variant": "no_hat"
}
```