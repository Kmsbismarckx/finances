#!/usr/bin/env bash
set -euo pipefail

# Собирает Rust-ядро под Android через cargo-ndk и генерирует
# Kotlin-биндинги через uniffi-bindgen.
# Использование: ./scripts/build-android.sh [--release]

if [[ ! -f Cargo.toml ]]; then
    echo "Запускай из корня репозитория (там, где Cargo.toml)" >&2
    exit 1
fi

PROFILE="debug"
RELEASE_FLAG=""
if [[ "${1:-}" == "--release" ]]; then
    PROFILE="release"
    RELEASE_FLAG="--release"
fi

JNI_LIBS_DIR="android/app/src/main/jniLibs"
KOTLIN_OUT_DIR="android/app/src/main/java"
SO_PATH="target/aarch64-linux-android/$PROFILE/libfinances.so"

echo "==> Сборка Rust-ядра под Android ($PROFILE)"
cargo ndk -o "$JNI_LIBS_DIR" -t armeabi-v7a -t arm64-v8a -t x86_64 build $RELEASE_FLAG

echo "==> Генерация Kotlin-биндингов"
cargo run --bin uniffi-bindgen generate --library "$SO_PATH" --language kotlin --out-dir "$KOTLIN_OUT_DIR"

echo "==> Готово. Пересобери Android-проект в Android Studio, чтобы подхватить изменения."
