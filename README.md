## Example of reproducibility in Rust and using shadow-rs

Build using the following command:

```bash
SOURCE_DATE_EPOCH=1309379017 cargo build
```

To test

```
cd /tmp
git clone https://github.com/kushaldas/reproducible
cd reproducible
SOURCE_DATE_EPOCH=1309379017 cargo build
sha256sum ./target/debug/reproducible
```

On my Fedora 34, the value is: `189280f7e14874474ac13c9c7a528d5286ff523aa6d37f9f96b1be241a745d93`.
