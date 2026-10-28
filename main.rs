#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::prelude::*;
use uefi::proto::console::text::Color;

#[entry]
fn main() -> Status {
    // UEFI gives us a portable console; VGA memory at 0xb8000 is x86-specific.
    uefi::helpers::init().unwrap();

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::LightCyan, Color::Black);
        let _ = writeln!(stdout, "Hello from my own Rust kernel!");
        let _ = stdout.set_color(Color::LightGreen, Color::Black);
        let _ = writeln!(stdout, "ARM64 / AArch64 + no_std + UEFI");
        let _ = writeln!(stdout, "");
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = writeln!(stdout, "This is an ARM64 UEFI application.");
        let _ = writeln!(stdout, "Press a key or reset the VM to exit.");
    });

    // Return to the UEFI firmware instead of executing x86 HLT.
    Status::SUCCESS
}
