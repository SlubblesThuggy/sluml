# sluml
>A rust math library

Sluml is a library for the rust programming language. The goal is to provide dependency-free math-related tools to ease the production of, primarily, simulations.

## How to use

The project is currently not uploaded to crates.io. Instead you can specify the github link in your ``Cargo.lock`` file:
```toml
[dependencies]
sluml = { git = "https://github.com/SlubblesThuggy/sluml" }
```
Or you can clone the directory to your own device and specify a path, eg.
```toml
[dependencies]
sluml = { path = "../sluml" }
```

## Development setup

This project aims to use as few dependencies as possible, but in case any are used they should be preconfigured in the ``Cargo.lock`` file and cargo should install them automatically for you.

## License

Distributed under the MIT license. See ``LICENSE`` for more information.

## Contributing

Contributing is currently not supported, but is planned to be once the groundwork is layed out.
