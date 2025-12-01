# smol_str

[![CI](https://github.com/rust-analyzer/smol_str/workflows/CI/badge.svg)](https://github.com/rust-analyzer/smol_str/actions?query=branch%3Amaster+workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/smol_str.svg)](https://crates.io/crates/smol_str)
[![API reference](https://docs.rs/smol_str/badge.svg)](https://docs.rs/smol_str/)


A `SmolStr` is a string type that has the following properties:

* `size_of::<SmolStr>() == 24` (therefore `== size_of::<String>()` on 64 bit platforms)
* `Clone` is `O(1)`
* Strings are stack-allocated if they are:
    * Up to 23 bytes long
    * Longer than 23 bytes, but substrings of `WS` (see `src/lib.rs`). Such strings consist
    solely of consecutive newlines, followed by consecutive spaces
* If a string does not satisfy the aforementioned conditions, it is heap-allocated
* Additionally, a `SmolStr` can be explicitly created from a `&'static str` without allocation

Unlike `String`, however, `SmolStr` is immutable. The primary use case for
`SmolStr` is a good enough default storage for tokens of typical programming
languages. Strings consisting of a series of newlines, followed by a series of
whitespace are a typical pattern in computer programs because of indentation.
Note that a specialized interner might be a better solution for some use cases.

## Benchmarks

The following benchmarks illustrate the performance characteristics of `SmolStr` for various operations. All benchmarks were run on `Monday, December 1, 2025`.

### `from_utf8_lossy` Comparison: `SmolStr` vs `String`

This section compares `SmolStr::from_utf8_lossy` against `String::from_utf8_lossy` for different string lengths and validity scenarios. The percentage difference indicates how much slower (+) or faster (-) `SmolStr` is compared to `String`.

| Length | Scenario           | SmolStr Time (ns) | String Time (ns) | SmolStr vs String |
|--------|--------------------|-------------------|------------------|-------------------|
| 12     | Valid              | 13.068            | 10.154           | +28.69% slower    |
| 12     | Invalid (single)   | 13.432            | 23.700           | -43.32% faster    |
| 12     | Invalid (many)     | 25.038            | 35.990           | -30.43% faster    |
| 50     | Valid              | 43.395            | 28.802           | +50.66% slower    |
| 50     | Invalid (single)   | 73.348            | 57.962           | +26.54% slower    |
| 50     | Invalid (many)     | 107.27            | 91.219           | +17.59% slower    |
| 1000   | Valid              | 268.77            | 240.27           | +11.86% slower    |
| 1000   | Invalid (single)   | 354.94            | 322.10           | +10.19% slower    |
| 1000   | Invalid (many)     | 1424.3            | 1328.6           | +7.20% slower     |

_Note: Negative percentage indicates SmolStr is faster, positive indicates SmolStr is slower._

### Other `SmolStr` Operations

Here are the detailed benchmark results for other `SmolStr` operations, organized by string length:

#### Length: 12 bytes

| Benchmark                                | Time (ns) |
|------------------------------------------|-----------|
| `format_smolstr!`                        | 26.389    |
| `SmolStr::from`                          | 14.389    |
| `SmolStr::clone`                         | 4.3895    |
| `SmolStr::eq`                            | 2.2177    |
| `to_lowercase_smolstr`                   | 23.447    |
| `to_ascii_lowercase_smolstr`             | 7.4287    |
| `replace_smolstr`                        | 8.0733    |

#### Length: 50 bytes

| Benchmark                                | Time (ns) |
|------------------------------------------|-----------|
| `format_smolstr!`                        | 59.060    |
| `SmolStr::from`                          | 12.080    |
| `SmolStr::clone`                         | 3.6731    |
| `SmolStr::eq`                            | 2.3987    |
| `to_lowercase_smolstr`                   | 51.157    |
| `to_ascii_lowercase_smolstr`             | 26.337    |
| `replace_smolstr`                        | 33.498    |

#### Length: 1000 bytes

| Benchmark                                | Time (ns) |
|------------------------------------------|-----------|
| `format_smolstr!`                        | 101.73    |
| `SmolStr::from`                          | 19.590    |
| `SmolStr::clone`                         | 3.2027    |
| `SmolStr::eq`                            | 11.466    |
| `to_lowercase_smolstr`                   | 146.04    |
| `to_ascii_lowercase_smolstr`             | 64.224    |
| `replace_smolstr`                        | 212.61    |

## MSRV Policy

Minimal Supported Rust Version: latest stable.

Bumping MSRV is not considered a semver-breaking change.
