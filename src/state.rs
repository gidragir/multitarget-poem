use std::sync::atomic::AtomicUsize;

#[derive(Default)]
pub struct AppState {
    pub requests_total: AtomicUsize,
}