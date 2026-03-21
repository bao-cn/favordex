pub mod ai;
pub mod browser;
pub mod checker;

pub(crate) fn is_local_address(url: &str) -> bool {
    url.starts_with("file://")
        || url.contains("localhost")
        || url.contains("127.0.0.1")
        || url.starts_with("http://192.168.")
        || url.starts_with("http://10.")
}
