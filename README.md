# mykernel-arm64

ARM64/AArch64 port of the original tiny Rust `no_std` project for testing in UTM SE.

## Important distinction

The original code writes directly to x86 VGA memory (`0xb8000`) and uses the x86 `hlt`
instruction. Neither works on ARM64. This port uses the UEFI text-console protocol instead,
so it is a bootable ARM64 UEFI application that demonstrates the same output. It is not yet
a standalone bare-metal kernel: UEFI remains in control and provides the console.

## Build on a computer with Rust installed

Install Rust from https://rustup.rs, then in this folder run:

```sh
rustup target add aarch64-unknown-uefi
cargo build --release
```

Expected output:
`target/aarch64-unknown-uefi/release/mykernel-arm64.efi`

If the dependency API has changed by the time you build, run `cargo update` and check the
compiler error against the `uefi` crate documentation.

## Make a bootable FAT image

On Linux, install `dosfstools` and `mtools`, then:

```sh
mkdir -p esp/EFI/BOOT
cp target/aarch64-unknown-uefi/release/mykernel-arm64.efi esp/EFI/BOOT/BOOTAA64.EFI
dd if=/dev/zero of=arm64-uefi.img bs=1M count=64
mkfs.vfat arm64-uefi.img
mcopy -i arm64-uefi.img -s esp/EFI ::
```

The standard ARM64 UEFI removable-media path is `EFI/BOOT/BOOTAA64.EFI`.

## Run in UTM SE on iPhone/iPad

1. Build the `.efi` and create `arm64-uefi.img` on a computer (or another environment that
   can run Rust and the image tools). UTM SE is a VM runner, not a convenient Rust build host.
2. Transfer `arm64-uefi.img` to Files on the iPhone/iPad.
3. In UTM SE, create a new VM with **ARM64/AArch64** architecture and UEFI boot firmware.
   Do not select x86_64.
4. Add `arm64-uefi.img` as a removable/USB or disk drive, depending on the options exposed by
   your UTM SE version. Use the firmware boot menu to boot that drive. If it does not appear
   as a boot option, use a UEFI Shell (if available) and run:
   `FS0:\EFI\BOOT\BOOTAA64.EFI`
   If the volume is mounted as `FS1:`, try that instead.
5. You should see the two greeting lines. The app returns to UEFI immediately, so the display
   may return to the firmware menu; take a screenshot or use a build that waits for a key if
   you need more time to inspect it.

## Limitations

- UTM SE uses emulation and can be slow.
- UTM SE's available machine/firmware/drive options vary by version. If it cannot boot a custom
  ARM64 UEFI app directly, boot an ARM64 Linux ISO and use its UEFI shell/boot manager if available.
- This project has not been compiled or tested in this environment.
