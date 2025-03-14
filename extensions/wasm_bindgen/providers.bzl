"""Rust WASM bindgen providers"""

RustWasmBindgenInfo = provider(
    doc = "Info about wasm-bindgen outputs.",
    fields = {
        "js": "Depset[File]: The Javascript files produced by `wasm-bindgen`.",
        "root": "str: The path to the root of the `wasm-bindgen --out-dir` directory.",
        "snippets": "File: The snippets directory produced by `wasm-bindgen`.",
        "ts": "Depset[File]: The Typescript files produced by `wasm-bindgen`.",
        "wasm": "File: The `.wasm` file generated by `wasm-bindgen`.",
    },
)
