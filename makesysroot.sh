# Shell script to build sysroot
# Should be called by the makefile, do not call directly

rm -rf sysroot

mkdir -p sysroot

mkdir -p sysroot/boot/limine
cp -v limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin sysroot/boot/limine
cp -v kernel/kernel sysroot/boot/butterscotch.kernel
mkdir -p sysroot/EFI/BOOT
cp -v limine/BOOTX64.EFI sysroot/EFI/BOOT/
cp -v limine/BOOTIA32.EFI sysroot/EFI/BOOT/
