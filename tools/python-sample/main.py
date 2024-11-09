import sys
from pathlib import Path

import click


def tee(
    outputs: tuple[Path],
):
    for line in sys.stdin:
        sys.stdout.write(line)
        for output in outputs:
            print(line, file=output.open("a"), end="")


@click.command()
@click.argument("paths", type=click.Path(path_type=Path), nargs=-1)
def main(paths: tuple[Path]) -> None:
    tee(outputs=paths)


if __name__ == "__main__":
    main()
