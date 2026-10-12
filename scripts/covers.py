#!/usr/bin/env python3
"""The catalog's cover pictures, from the user's cover art: each
SRC/NAME-cover.jpg resized to 320 by 320 into images/covers/TARGET.jpg,
where TARGET is the game (or the group) the cover is for. The covers
keep their "E / Everyone" rating (the user's); the ESRB logo and
acronym under it, on the covers that have them, are blurred.

    just covers DIR      # uv pip installs Pillow into .venv first
"""
import sys
from pathlib import Path

from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parent.parent
# Cover file -> what it is for: a game's slug, or a group (group-NAME).
COVERS = {
    "dungeons": "dungeon",
    "maze-escape": "maze",
    "race": "racer",
    "scroll-right": "scroller",
    "stargazer": "stargazer",
    "towerd-defense": "tower-defense",
    "world-capitals": "capitals",
    "trek": "trek-adventure",
    "space-rocks": "asteroids",
    "falling-blocks": "blocks",
    "ray-caster": "raycaster",
    "horse-race": "horse-race",
    "puzzles": "group-puzzles",
}
# The covers whose badge has the ESRB logo and acronym, and where it is
# (left, top, right, bottom in the 1024-pixel original).
ESRB = {"maze-escape", "race", "towerd-defense", "ray-caster"}
ESRB_BOX = (24, 948, 142, 1006)
SIZE = 320


def main():
    src = Path(sys.argv[1]).expanduser()
    out = ROOT / "images" / "covers"
    out.mkdir(parents=True, exist_ok=True)
    for name, target in COVERS.items():
        im = Image.open(src / f"{name}-cover.jpg").convert("RGB")
        if name in ESRB:
            box = im.crop(ESRB_BOX).filter(ImageFilter.GaussianBlur(9))
            im.paste(box, ESRB_BOX[:2])
        im.resize((SIZE, SIZE), Image.LANCZOS).save(out / f"{target}.jpg", quality=84, optimize=True, progressive=True)
        print(f"covers: {name}-cover.jpg -> images/covers/{target}.jpg")


if __name__ == "__main__":
    main()
