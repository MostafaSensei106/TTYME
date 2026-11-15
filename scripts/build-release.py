#!/usr/bin/env python3
"""Build and package both TTYME programs for Windows/Linux x86, x86_64, ARM64."""

import argparse
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tomllib
import zipfile


ROOT = Path(__file__).resolve().parents[1]
TARGETS = {
    "x86_64-unknown-linux-musl": ("linux", "x86_64"),
    "i686-unknown-linux-musl": ("linux", "x86"),
    "aarch64-unknown-linux-musl": ("linux", "arm64"),
    "x86_64-pc-windows-gnu": ("windows", "x86_64"),
    "i686-pc-windows-gnu": ("windows", "x86"),
    "aarch64-pc-windows-gnullvm": ("windows", "arm64"),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", action="append", choices=TARGETS,
                        help="Build only this target; repeat for several targets")
    parser.add_argument("--llvm-mingw", type=Path,
                        help="LLVM-MinGW directory (needed for Windows ARM64)")
    args = parser.parse_args()
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"]
    output = ROOT / "release"
    binaries = output / "bin"
    binaries.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    if args.llvm_mingw:
        compiler = args.llvm_mingw.resolve() / "bin" / "aarch64-w64-mingw32-clang"
        if not compiler.is_file():
            parser.error(f"LLVM-MinGW compiler not found: {compiler}")
        environment["CARGO_TARGET_AARCH64_PC_WINDOWS_GNULLVM_LINKER"] = str(compiler)
    # Keep build artifacts at a predictable location even with CARGO_TARGET_DIR set.
    environment["CARGO_TARGET_DIR"] = str(ROOT / "target")

    for target in args.target or TARGETS:
        system, arch = TARGETS[target]
        subprocess.run(
            ["cargo", "build", "--locked", "--offline", "--release", "--bins", "--target", target],
            cwd=ROOT, env=environment, check=True,
        )
        for binary, program in [("app", "client"), ("backend", "server")]:
            name = f"ttyme-{program}-v{version}-{system}-{arch}"
            extension = ".exe" if system == "windows" else ""
            executable = binaries / f"{name}{extension}"
            shutil.copy2(ROOT / "target" / target / "release" / f"{binary}{extension}", executable)
            if system == "windows":
                archive = output / f"{name}.zip"
                with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as package:
                    package.write(executable, executable.name)
                    package.write(ROOT / "README.md", "README.md")
            else:
                archive = output / f"{name}.tar.gz"
                with tarfile.open(archive, "w:gz") as package:
                    package.add(executable, arcname=executable.name)
                    package.add(ROOT / "README.md", arcname="README.md")
            print(f"Packaged {archive.name}", flush=True)

    archives = sorted(output.glob(f"ttyme-*-v{version}-*.zip"))
    archives += sorted(output.glob(f"ttyme-*-v{version}-*.tar.gz"))
    checksums = "".join(
        f"{hashlib.sha256(archive.read_bytes()).hexdigest()}  {archive.name}\n"
        for archive in sorted(archives)
    )
    (output / "SHA256SUMS.txt").write_text(checksums)
    print(f"Release files: {output}")


if __name__ == "__main__":
    main()
