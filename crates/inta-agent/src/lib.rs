pub mod host;
pub mod install;
pub mod inventory;
pub mod managers;
pub mod models;
pub mod process;
pub mod server;
pub mod store;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PORT: u16 = 47821;

pub fn random_id(bytes: usize) -> String {
    use rand::{rngs::OsRng, RngCore};
    let mut value = vec![0u8; bytes];
    OsRng.fill_bytes(&mut value);
    value.iter().map(|b| format!("{b:02x}")).collect()
}
