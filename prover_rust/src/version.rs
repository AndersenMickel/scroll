use std::cell::OnceCell;

static DEFAULT_COMMIT: &str = "unknown";
static mut VERSION: OnceCell<String> = OnceCell::new();

pub const TAG: &str = "v4.4.3";
pub const DEFAULT_ZK_VERSION: &str = "000000-000000";

fn init_version() -> String {
    let commit = option_env!("GIT_REV").unwrap_or(DEFAULT_COMMIT);
    let zk_version = option_env!("ZK_VERSION").unwrap_or(DEFAULT_ZK_VERSION);
    format!("{TAG}-{commit}-{zk_version}")
}

pub fn get_version() -> String {
    unsafe { VERSION.get_or_init(init_version).clone() }
}