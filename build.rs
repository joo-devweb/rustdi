// build.rs
fn main() {
    // Tidak ada konfigurasi build khusus saat ini
    println!("cargo:rerun-if-changed=build.rs");
}