from .system import system
from pathlib import Path
from shutil import copy, rmtree


def build_buterscotch(project_root: Path, sysroot: Path, profile: str):
    print(" --- Building Butterscotch ---")
    build_cargo(project_root, profile)

    print(" * Building Sysroot")
    make_sysroot(sysroot, project_root, profile)


def build_cargo(project_root: Path, profile: str):
    manifest = project_root.joinpath("Cargo.toml")
    system(f"cargo build --profile {profile} --manifest-path {manifest}")


def make_sysroot(
    sysroot: Path,
    project_root: Path,
    profile: str,
):
    # Clear previous sysroot
    if sysroot.exists():
        rmtree(sysroot)
    sysroot.mkdir()

    boot = sysroot.joinpath("boot")
    boot.mkdir()

    install_kernel(boot, get_build_dir(project_root, profile))
    install_limine(project_root, boot.joinpath("limine"))

    efi = sysroot.joinpath("EFI", "BOOT")
    efi.mkdir(parents=True)

    install_efi(project_root, efi)


def install_limine(project_root: Path, dir: Path):
    limine_files = [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
    ]
    dir.mkdir()
    copy(project_root.joinpath("limine.cfg"), dir.joinpath("limine.cfg"))
    for file in limine_files:
        copy(project_root.joinpath("limine", file), dir.joinpath(file))


def install_kernel(boot: Path, build_dir: Path):
    copy(
        build_dir.joinpath("butterscotch_kernel"), boot.joinpath("butterscotch.kernel")
    )


def install_efi(project_root: Path, efi: Path):
    files = ["BOOTX64.EFI", "BOOTIA32.EFI"]
    for file in files:
        copy(project_root.joinpath("limine", file), efi.joinpath(file))


def get_build_dir(project_root: Path, profile: str) -> Path:
    profile_dirname = profile
    if profile_dirname == "dev":
        profile_dirname = "debug"

    build_dir = project_root.joinpath(
        "target/x86_64-butterscotch_kernel/", profile_dirname
    )

    return build_dir
