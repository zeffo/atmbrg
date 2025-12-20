
pub mod app;
pub mod api;

fn main() {
    loop {
        api::scan_devices();
    }
}
