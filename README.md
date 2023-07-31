# RAI / personal local rust AI

- create `.env` file with `MODEL_PATH=<path to the model>` pointing to the model in `ggml` format.
- run `cargo build --release`
- run `cargo run --release` or the executable in `target/release`

Optionally you can call the command with an extra string, which will be the stated purpose of the conversation. Default is "A chat between a Human and an AI.". "Human" and "AI" are hard coded characters in this simulation and can be used as reference.
