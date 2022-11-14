use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

mod ui;

use crate::ui::EspBackend;

slint::include_modules!();


fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world from Rust!");

    slint::platform::set_platform(Box::new(EspBackend::default()))
    .unwrap();

    // Setup the UI.
    let ui = MyUI::new();
    // ... setup callback and properties on `ui` ...

    ui.run();
}
