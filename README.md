# d4-oxide

> A Rust wrapper around [d4][d4].

## Usage

Add it as a dependency to your Cargo.toml:

```toml
[dependencies]
d4-oxide = "0.2"
```

## Requirements

- [Mt-KaHyPar][mtkahypar]
- [GMP][gmp] (with C++ bindings)

To point to the directories containing Mt-KaHyPar, the following environment variables can be used:

| Variable                | Meaning                                                                                |
|-------------------------|----------------------------------------------------------------------------------------|
| `MTKAHYPAR_ROOT`        | Contains `include` and `lib` or `lib64` subdirectories with the corresponding content. |
| `MTKAHYPAR_INCLUDE_DIR` | Contains the header files.                                                             |
| `MTKAHYPAR_LIB_DIR`     | Contains library files.                                                                |

[d4]: https://github.com/crillab/d4v2
[mtkahypar]: https://github.com/kahypar/mt-kahypar
[gmp]: https://gmplib.org
