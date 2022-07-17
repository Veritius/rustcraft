# Content Package Structure
```
packages
└── example_package_alpha
    ├── lib
    │   ├── client
    │   │   └── client.rs
    │   ├── server
    │   │   └── server.rs
    │   └── shared
    │       └── server.rs
    ├── assets
    │   ├── models
    │   │   └── cabbage.obj
    │   └── textures
    │       └── cabbage.png
    ├── locale
    │   └── en-US
    └── meta.yml
```

# Directories and Files
## meta.yml
The `meta.yml` file is always at the root of the package.
It describes information about the package, such as names.

The fields are as follows:
- `id`: Unique identifier for this package
- `name`: A localised package name (Fluent ID)
- `desc`: A localised package description (Fluent ID)
- `authors`: A list of authors (strings)
- `dependencies`: A list of packages that this package depends on.

`meta.yml` example
```yml
id: packagetitle.submodule
name: packagetitle-submodule-package-name
desc: packagetitle-submodule-package-desc
authors: ["Veritius"]
dependencies: ["packagetitle.submodule2"]
```

## lib
The `lib` directory contains libraries that the game will load in at runtime.
- Libraries in the `shared` directory will be loaded on both the server and client.
- Libraries in the `server` directory will be loaded on only the server.
- Libraries in the `client` directory will be loaded on only the client.

**Safety is not ensured.** Modders should ensure their own code is memory and thread safe, as the game cannot check at runtime.

***Players should verify the safety of the mods they're downloading before use.***

## assets
The `assets` directory stores any assets that the game will load.

Assets are accessed by giving a path such as `textures/cabbage.png`. The program will then find the highest priority package that has this asset, and use that. If the path is not found in any package, an error will be given.

## locale
The `locale` directory stores [Fluent](https://projectfluent.org/) translations, under the `.ftl` extension. The game automatically reads all files and places it in an easily accessible Bevy resource.