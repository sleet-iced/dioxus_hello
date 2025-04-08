# android setuo

For Android development with Dioxus, here's what you need to set up:

1. Install Android SDK and NDK:
```bash
brew install --cask android-sdk
brew install --cask android-ndk
```

2. Set up Android targets for Rust:
```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

3. Install Android tools:
```bash
brew install android-platform-tools
```

4. Update your Cargo.toml:
```toml
[lib]
crate-type = ["cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
android_logger = "0.11.0"
jni = "0.21.1"
```

5. Create an Android-specific main file:
```rust
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: AndroidApp) {
    dioxus::launch(App);
}
```

6. Build for Android:
```bash
cargo build --release --target aarch64-linux-android
```

7. To serve to an Android device:
```bash
dx serve --platform android
```

Additional setup:
1. Install Android Studio
2. Create an Android Virtual Device (AVD) for testing
3. Enable USB debugging on your Android device

Android development is generally easier to set up than iOS since it doesn't require an active developer account or provisioning profiles. You can test on both physical devices and emulators.
