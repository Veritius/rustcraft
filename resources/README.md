# Content Package Structure
```
resources
└── example_package_alpha
    ├── assets
    │   ├── models
    │   └── textures
    ├── locale
    │   └── en-US
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

`meta.yml` example
```yml
id: rustcraft.core
name: rustcraft-core-plugin-name
desc: rustcraft-core-plugin-desc
authors: ["Veritius"]
```

## assets
The `assets` directory stores any assets that the game will load.

Assets are accessed by giving a path such as `models/cabbage.png`. The program will then find the highest priority package that has this asset, and use that. If the path is not found in any package, an error will be given.

## locale
The `locale` directory stores [Fluent](https://projectfluent.org/) translations, under the `.ftl` extension. The game automatically reads all files and places it in an easily accessible Bevy resource.