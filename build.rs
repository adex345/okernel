use std::process::Command;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=target/x86_64-unknown-none/debug/okernel");
    println!("cargo:rerun-if-changed=limine.conf");

    let limine_sdk = "target/limine-sdk";

    // KROK 1: Automatyczne pobieranie Limine (Zawsze sprawdzamy to na początku)
    if !Path::new(limine_sdk).exists() {
        let git_status = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                "--branch=v11.x-binary",
                "https://github.com/limine-bootloader/limine",
                limine_sdk
            ])
            .status()
            .expect("Błąd: Nie można uruchomić gita!");

        assert!(git_status.success(), "Błąd: Klonowanie repozytorium Limine nie powiodło się.");
    }

    // KROK 2: Sprawdzenie czy binarka kernela już powstała. 
    // Jeśli nie (np. faza przed-kompilacyjna), kończymy skrypt sukcesem i czekamy na linker.
    let kernel_path = "target/x86_64-unknown-none/debug/okernel";
    if !Path::new(kernel_path).exists() {
        return;
    }

    // KROK 3: Przygotowanie struktury katalogów pod obraz ISO
    let _ = std::fs::create_dir_all("target/iso_root/boot/");
    let _ = std::fs::create_dir_all("target/iso_root/EFI/BOOT/");

    // KROK 4: Kopiowanie Twojego jądra i konfiguracji
    std::fs::copy(kernel_path, "target/iso_root/boot/okernel").unwrap();
    std::fs::copy("limine.conf", "target/iso_root/boot/limine.conf").unwrap();

// KROK 5: assety Limine
for f in ["limine-bios-cd.bin", "limine-uefi-cd.bin", "limine-bios.sys"] {
    std::fs::copy(format!("{limine_sdk}/{f}"), format!("target/iso_root/boot/{f}")).unwrap();
}
std::fs::copy(format!("{limine_sdk}/BOOTX64.EFI"), "target/iso_root/EFI/BOOT/BOOTX64.EFI").unwrap();

// KROK 5b: zbuduj narzędzie limine (raz)
let limine_bin = format!("{limine_sdk}/limine");
if !Path::new(&limine_bin).exists() {
    let ok = Command::new("make").args(["-C", limine_sdk]).status().unwrap().success();
    assert!(ok, "Nie udało się zbudować narzędzia limine");
}

// KROK 6: ISO
let ok = Command::new("xorriso")
    .args([
        "-as", "mkisofs", "-R", "-r", "-J",
        "-b", "boot/limine-bios-cd.bin",
        "-no-emul-boot", "-boot-load-size", "4", "-boot-info-table",
        "--efi-boot", "boot/limine-uefi-cd.bin",
        "-efi-boot-part", "--efi-boot-image", "--protective-msdos-label",
        "target/iso_root", "-o", "okernel.iso",
    ])
    .status().expect("xorriso nie zainstalowane!").success();
assert!(ok, "xorriso zawiodło");

// KROK 7: instalacja stage 1/2 w ISO (BIOS)
let ok = Command::new(&limine_bin).args(["bios-install", "okernel.iso"]).status().unwrap().success();
assert!(ok, "limine bios-install zawiodło");
}
