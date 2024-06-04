from os import environ, system
from pathlib import Path
from shutil import rmtree

from .external_resources import fetch_extenal_resources
from .build import build_buterscotch
from .clean import clean
from .img import makeimg

sysroot = Path(environ.get("SYSROOT", "sysroot")).resolve()
profile = environ.get("RUST_PROFILE", "dev")


def build(project_root: Path):
    fetch_extenal_resources(project_root)
    build_buterscotch(project_root, sysroot, profile)
    makeimg(project_root, sysroot)

    print("Build completed successfully")

def run(project_root: Path, uefi: bool = False):
    img = project_root.joinpath("butterscotch.img")

    args = ""
    if uefi:
        args += f"-bios {project_root.joinpath("ovmf/OVMF.fd")}"

    system(f"qemu-system-x86_64 -M q35 -m 2G -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 {args} {img}")
