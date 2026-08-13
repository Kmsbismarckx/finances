#[uniffi::export]
pub fn greet(name: String) -> String {
    format!("Привет, {name}! Это сообщение из Rust.")
}
