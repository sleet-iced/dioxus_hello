# iOS set up

Note: You'll need to have Xcode installed and configured properly on your Mac. Make sure you have:

- Xcode installed from the App Store
- An active Apple Developer account
- A valid provisioning profile set up

> and that is why i probably won't build for ios.


```sh
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

```sh
xcode-select --install
```

```sh
brew install ios-deploy
```

```toml
[lib]
crate-type = ["staticlib", "cdylib", "rlib"]

[target.'cfg(target_os = "ios")'.dependencies]
objc = "0.2.7"
```

``rs
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn start_app() {
    dioxus::launch(App);
}
```


```sh
cargo build --release --target aarch64-apple-ios
```