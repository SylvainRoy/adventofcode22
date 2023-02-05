#!/usr/bin/env python

import os
import typer
from pathlib import Path

app = typer.Typer()

@app.command()
def build():
    dirs = [d for d in Path(__file__).parent.iterdir() if d.is_dir()]
    dirs = sorted(dirs)
    for p in dirs:
        if not p.is_dir():
            continue
        print(f"\n## {p.name}:")
        os.system(f"cd {p}; cargo build --release")

@app.command()
def run():
    dirs = [d for d in Path(__file__).parent.iterdir() if d.is_dir()]
    dirs = sorted(dirs)
    for p in dirs:
        if not p.is_dir():
            continue
        print(f"\n## {p.name}:")
        os.system(f"cd {Path(__file__).parent.joinpath(p)}; ./target/release/{p.name}")

if __name__ == '__main__':
    app()

