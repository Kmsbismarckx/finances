# Стек — Money Manager (Rust)

## Архитектура
Путь A: общее Rust-ядро + нативный UI на каждой платформе (UniFFI-биндинги).

## Платформы
- Android: min SDK 24 (Android 7), UI — Kotlin + Jetpack Compose
- iOS: позже, UI — Swift/SwiftUI (то же ядро)

## Rust-ядро
- БД: rusqlite (синхронный SQLite)
- Деньги: i64, минорные единицы (копейки/центы)
- FFI: uniffi, стиль proc-macros (`#[uniffi::export]`), без .udl

## Сборка под Android
- cargo-ndk, вызов скриптом (не gradle-плагин)

## Синхронизация
- Транспорт: собственный сервер (Rust backend, HTTPS)
- Модель конфликтов: last-write-wins по `updated_at` (время сервера)
- Удаление: soft delete (tombstone, `deleted_at`)
- Синк: инкрементальный, по монотонному `sync_version`/курсору на клиенте

## Открыто (не решено)
- Cargo workspace: структура crates (core / ffi / server)
- Схема БД (таблицы, поля синхронизации)
- Веб-фреймворк сервера (Axum / Actix)
- Аутентификация пользователя
