`
cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/wit_sample.wasm -o wit_sample_com.wasm --adapt ./wasi_snapshot_preview1.wasm
`