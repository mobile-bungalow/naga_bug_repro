
Reproduction steps

```bash
cargo run
```

the resulting panic should be:

```bash
byte index 45 is out of bounds of ``
```

The source string is set to the empty string [here](https://github.com/gfx-rs/wgpu/blob/6db097694ce121f9cd927ef6dbb5608a69906760/wgpu-core/src/device/resource.rs#L1493)
