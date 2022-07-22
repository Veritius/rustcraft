**Loading external libraries is *not* a complete or even properly working feature! I don't recommend using it in any way.**

# Official Packages
This directory contains some official packages for the game. Feel free to use them as a template for your own packages.

# Example Content Package Structure
```
packages
└── example_package_alpha
    ├── assets
    │   ├── models
    │   │   └── cabbage.obj
    │   ├── sound
    │   │   └── cabbage.ogg
    │   └── textures
    │       └── cabbage.png
    ├── locale
    │   ├── en-US
    │   │   ├── package.ftl
    │   │   ├── ui
    │   │   │   └── mainmenu.ftl
    │   │   └── blocks
    │   │       └── stone.ftl
    │   └── nl-NL 
    │       ├── package.ftl
    │       ├── ui
    │       └── blocks
    └── package.toml
```

# Directories and Files
## package.toml
The `package.toml` file is always at the root of the package.
It describes information about the package, such as names.

Mandatory fields:
- `id`: Unique identifier for this package
- `name`: A localised package name (Fluent ID)
- `desc`: A localised package description (Fluent ID)
- `authors`: A list of authors (strings)
- `version`: The version of the package (SemVer compliant)
- `gameversionrange`: Restricts the package to running within a specific range of game versions

Optional fields:
- `dependencies`: A list of packages that this package depends on.
- `incompatibilities`: A list of packages that this package will refuse to function with.

`package.toml` example
```toml
id = "jimmysmod.decorations"
name = "jimmysmod-decorations-package-name"
desc = "jimmysmod-decorations-package-desc"
authors = ["Jimmy"]
version = "1.7.2"
gameversionreq = "1.7.*"
dependencies = ["jimmysmod.core"]
incompatibilities = ["davesmod.decorations"]
```

**Further reading:** [Semantic Versioning](https://semver.org/), [Version Requirement](https://docs.rs/semver/latest/semver/struct.VersionReq.html)

## assets
The `assets` directory stores any assets that the game will load.

Assets are accessed by giving a path such as `textures/cabbage.png`. The program will then find the highest priority package that has this asset, and use that. If the path is not found in any package, an error will be given.

## locale
The `locale` directory stores [Fluent](https://projectfluent.org/) translations, under the `.ftl` extension. The game automatically reads all files and places it in an easily accessible Bevy resource.

It's recommended to prefix your localisation string keys with the name of your package, like this: `rustcraft-core-debug-hello`. This prevents overlap with other localisation strings from other packages, as they are all collated during runtime.
