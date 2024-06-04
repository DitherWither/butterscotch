#!/usr/bin/env python3

from pathlib import Path
import builder
from sys import argv

project_root = Path(__file__).resolve().parent

if len(argv) != 2:
    print(f"Usage: {argv[0]} [command]")
    exit(-1)

match argv[1]:
    case "build":
        builder.build(project_root)

    case "clean":
        builder.clean(project_root)

    case "distclean":
        builder.clean(project_root, distclean=True)

    case "run":
        builder.build(project_root)
        builder.run(project_root)

    case "run-uefi":
        builder.build(project_root)
        builder.run(project_root, uefi=True)

    case _:
        print(f"Usage: {argv[0]} [command]")