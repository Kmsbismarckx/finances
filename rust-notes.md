# Заметки по Rust — механизмы из нашего кода

Не про архитектуру (см. architecture.md) — про сами языковые механизмы Rust, разобранные на реальных кусках кода из проекта.

## Приватные поля + публичные методы

`Email(String)` — поле не `pub`, доступ только через методы (`as_str()`). Обычная инкапсуляция, но именно она делает Always-Valid Model возможной: если единственный способ получить `Email` — `Email::new()` с проверкой формата, невалидный `Email` не может возникнуть нигде.

## Newtype — обёртка в одно поле

`UserId(String)`, `Email(String)`, `PasswordHash(String)` — обёртки вокруг `String`. Компилятор не даст перепутать `UserId` и `DeviceId`, даже если внутри у обоих `String` — это разные типы. Ошибка ловится на этапе компиляции.

## `Result<T, E>` и `?`

В Rust нет исключений. Функция, которая может не получиться, возвращает `Result<Успех, Ошибка>`:
```rust
let user = User::register(id, email, password_hash)?;
```
`?` прерывает функцию и возвращает ошибку наружу при `Err(...)` — как ранний `return Err(...)` в одну строку.

## Владение — `self` / `&self` / `&mut self`

```rust
fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
fn save(&mut self, user: User) -> Result<(), RepositoryError>;
```
`&self` — только посмотреть. `&mut self` — можно менять внутреннее состояние. `user: User` (без `&`) — функция забирает объект себе полностью. Поэтому в `register_user` пришлось писать `repo.save(user.clone())` — раз `save` забирает `User`, а он ещё нужен для `Ok(user)`, пришлось клонировать.

## `String` vs `&str`

`String` — своя строка, можно менять, живёт пока не удалишь. `&str` — "взгляд" на чужую строку без владения. `as_str(&self) -> &str` — одалживает посмотреть, не отдаёт владение.

## `Option<T>` вместо `null`

В Rust нет `null`. Если что-то может отсутствовать, это в типе: `Option<User>` — либо `Some(user)`, либо `None`. Компилятор заставляет обработать оба случая.

## Трейты — контракты

```rust
pub trait UserRepository {
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
    fn save(&mut self, user: User) -> Result<(), RepositoryError>;
}
```
Описывает не устройство, а что объект должен уметь. `InMemoryUserRepository` и `SqliteUserRepository` — два разных способа выполнить один контракт.
```rust
pub fn register_user(repo: &mut impl UserRepository, ...)
```
`impl UserRepository` = "подойдёт что угодно, что это реализует". Именно это физически не даёт `domain` знать про rusqlite — у функции на руках только то, что описано в трейте.

## `#[derive(...)]`

Макросы, которые дописывают код за тебя. `#[derive(Debug, Clone, PartialEq, Eq)]` — сгенерируй вывод на печать, копирование, сравнение на равенство. Вручную это писалось бы отдельными `impl`-блоками.

## `#[cfg(test)]`

Условная компиляция — код внутри попадает только в тестовую сборку (`cargo test`), в обычный `cargo build` не включается вообще. Поэтому `fakes.rs` помечен так — иначе тестовые заглушки попали бы в релизный бинарник.

## Замыкания (closures)

`|row| { ... }` в `query_row(..., |row| {...})` — анонимная функция "на лету", передаётся как аргумент. Похоже на лямбды в других языках.

## `thiserror`

`#[derive(Error)]` + `#[error("...")]` над `enum DomainError` генерирует текстовое описание ошибки и реализацию `std::error::Error`. `#[from]` (`Repository(#[from] RepositoryError)`) позволяет `?` автоматически конвертировать `RepositoryError` в `DomainError` без ручного `.map_err(...)`.

## Axum-экстракторы: `State(state)` и `Json(payload)`

`State<S>` и `Json<T>` — обычные tuple-структуры с одним полем, как и наши newtype (`pub struct Json<T>(pub T)`). Синтаксис в аргументах функции:
```rust
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ...
```
это pattern matching прямо в сигнатуре — то же самое, что `let State(state) = value;`, только без отдельной строки. `state` внутри функции — уже готовый `AppState`, `payload` — готовый `RegisterRequest`.

Как эти значения берутся из запроса — специфика Axum: у каждого такого типа есть реализация трейта-экстрактора. `State<AppState>` достаёт объект, once положенный через `.with_state(state)` при сборке `Router` — запрос не парсится, просто подсовывается готовый объект. `Json<RegisterRequest>` парсит тело запроса как JSON через `serde` (`#[derive(Deserialize)]` на `RegisterRequest`); если JSON битый — Axum сам вернёт `400` ещё до вызова кода внутри `register`.

Практическая деталь: тело запроса читается только один раз (поток байт), поэтому экстракторы, читающие тело (`Json`), должны идти в сигнатуре последними. `State` тело не трогает — может быть раньше.

По сути тот же принцип, что и с портами `UserRepository`/`PasswordHasher`: через типы в сигнатуре Axum понимает, что нужно, и сам это подключает — только для типовых вещей (JSON-тело, общее состояние) это уже сделано за тебя фреймворком.
