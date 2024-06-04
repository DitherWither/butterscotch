from pathlib import Path
from os import system


def makeimg(project_root: Path, sysroot: Path):
    print(" * Building Boot Image")
    img = project_root.joinpath("butterscotch.img")
    limine = project_root.joinpath("limine/limine")
    if img.exists():
        img.unlink()

    system(f"dd if=/dev/zero bs=1M count=0 seek=128 of={img}")
    system(f"sgdisk {img} -n 1:2048 -t 1:ef00")
    system(f"{limine} bios-install {img}")
    system(f"mformat -i {img}@@1M")
    system(f"mcopy -s -i {img}@@1M {sysroot}/* ::")
