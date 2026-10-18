#!/bin/sh
set -e
KERNEL="$1"
LIMINE=target/limine-sdk
ROOT=target/iso_root

[ -d $LIMINE ] || git clone --depth=1 --branch=v11.x-binary \
    https://github.com/limine-bootloader/limine $LIMINE
[ -x $LIMINE/limine ] || make -C $LIMINE

rm -rf $ROOT
mkdir -p $ROOT/boot $ROOT/EFI/BOOT
cp "$KERNEL" $ROOT/boot/okernel
cp limine.conf $ROOT/boot/
cp $LIMINE/limine-bios.sys $LIMINE/limine-bios-cd.bin $LIMINE/limine-uefi-cd.bin $ROOT/boot/
cp $LIMINE/BOOTX64.EFI $ROOT/EFI/BOOT/

xorriso -as mkisofs -R -r -J \
    -b boot/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine-uefi-cd.bin -efi-boot-part --efi-boot-image \
    --protective-msdos-label $ROOT -o target/okernel.iso
$LIMINE/limine bios-install target/okernel.iso

exec qemu-system-x86_64 -cdrom target/okernel.iso -m 256M -serial stdio
