# escape-simd

Shared SIMD kernels for JSON and HTML string escaping.

This crate is the implementation. Depend on [`json-escape-simd`](https://crates.io/crates/json-escape-simd)
or [`html-escape-simd`](https://crates.io/crates/html-escape-simd) unless you need both behind feature flags.

```toml
escape-simd = { version = "0.1", features = ["json"] }
escape-simd = { version = "0.1", features = ["html"] }
```

- `json` — `escape_simd::json::{escape, escape_into}`
- `html` — `escape_simd::html::{escape_html, escape_html_into, InputTooLarge}`
- `avx512` — enable the AVX-512 kernel (runtime-detected)
- `asan` — force the bounded tail-copy path for sanitizers

## License

See [LICENSE](LICENSE), [LICENSE-APACHE](LICENSE-APACHE), and [NOTICE](NOTICE).
Original project contributions are MIT-licensed. The shared `escape-simd`
kernels also contain Apache-2.0 code derived from sonic-rs and sonic-simd;
both licenses apply to those combined kernels. The facade source is MIT-licensed.
