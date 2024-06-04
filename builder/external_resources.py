from os import system
from pathlib import Path
from urllib.request import urlretrieve


def fetch_extenal_resources(project_root: Path):
    print(" --- Fetching and Building External Resources ---")
    fetch_ovmf(project_root)
    build_limine(project_root)


def build_limine(project_root: Path):
    limine = project_root.joinpath("limine")
    if limine.exists():
        print(" * Limine already fetched, skipping")
        return

    print(" * Fetching Limine")
    system(
        f"git clone https://github.com/limine-bootloader/limine.git --branch=v7.5.3-binary --depth=1 --quiet {limine} > /dev/null"
    )
    print(" * Building Limine")
    system(f"make -C {limine} > /dev/null")


def fetch_ovmf(project_root: Path):
    ovmf = project_root.joinpath("ovmf")
    if ovmf.exists():
        print(" * OVMF.FD already fetched, skipping")
        return

    ovmf.mkdir()

    print(" * Fetching OVMF.fd")
    ovmf_file = ovmf.joinpath("OVMF.fd")
    urlretrieve(
        "https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd", ovmf_file
    )
