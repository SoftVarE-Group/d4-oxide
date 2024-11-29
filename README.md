# d4-oxide

> A Rust wrapper around [d4][d4].

## Usage

Add it as a dependency to your Cargo.toml:

```toml
[dependencies]
d4-oxide = "0.5"
```

## Requirements

- [Mt-KaHyPar][mtkahypar]
- [Boost][boost]
- [GMP][gmp] (with C++ bindings)
- [zlib][zlib]

These dependencies can be made available in a [Nix][nix] development shell coming with this repository:

```
nix develop
```

Alternatively, to point to the directories containing Mt-KaHyPar, the following environment variables can be used:

| Variable                | Meaning                                                                                |
|-------------------------|----------------------------------------------------------------------------------------|
| `MTKAHYPAR_ROOT`        | Contains `include` and `lib` or `lib64` subdirectories with the corresponding content. |
| `MTKAHYPAR_INCLUDE` | Contains the header files.                                                             |
| `MTKAHYPAR_LIB`     | Contains library files.                                                                |

[d4]: https://github.com/crillab/d4v2
[mtkahypar]: https://github.com/kahypar/mt-kahypar
[boost]: https://boost.org
[gmp]: https://gmplib.org
[zlib]: https://zlib.net
[nix]: https://nixos.org
