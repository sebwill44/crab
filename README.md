# Crab🦀

## Guide

### Install

Everything should work if Rust is installed. Run `cargo install cargo-run-script` to get the script manager. Then, run `cargo run-script dev_run` to clean and run the project.

### Wav Files

Allow for storage of directly sampled analog sounds without further processing. .WAV is a wrapper for PCM encoding, which is raw uncompressed digital audio data. See <http://soundfile.sapp.org/doc/WaveFormat/>

#### PCM Sample Format Cheat Sheet</h3>

| Syntax      | Description |
| ----------- | ----------- |
| alaw      | A-Law       |
| f32be   | 32-bit floating-point big-endian        |
| f32le      | 32-bit floating-point little-endian       |
| f64be   | 64-bit floating-point big-endian        |
| f64le      | 64-bit floating-point little-endian       |
| mulaw   | mu-law        |
| s16be      | signed 16-bit big-endian       |
| s16le   | signed 16-bit little-endian        |
| s24be      | signed 24-bit big-endian       |
| s24le   | signed 24-bit little-endian        |
| s32be      |  signed 32-bit big-endian       |
| s32le   |  signed 32-bit little-endian        |
| s8      |  signed 8-bit      |
| u16be   |  unsigned 16-bit big-endian        |
| u16le      |  unsigned 16-bit little-endian       |
| u24be   |  unsigned 24-bit big-endian        |
| u24le      |  unsigned 24-bit little-endian       |
| u32be   |  unsigned 32-bit big-endian        |
| u32le    | PCM unsigned 32-bit little-endian |
| u8       | PCM unsigned 8-bit|
