from os import system
from pathlib import Path
from shutil import rmtree


def clean(project_root: Path, distclean: bool = True):
    manifest = project_root.joinpath("Cargo.toml")
    delete(project_root, "sysroot")

    system(f"cargo build --manifest-path {manifest}")
    if distclean:
        delete(project_root, "limine")
        delete(project_root, "ovmf")


def delete(project_root: Path, path: str):
    rmtree(project_root.joinpath(path), ignore_errors=True)
