//! SigmaOS-fallback-boot — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_fallback_boot_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-fallback-boot: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-fallback-boot: ok");
}
