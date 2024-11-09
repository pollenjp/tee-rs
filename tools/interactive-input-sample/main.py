import click


@click.command()
def main():
    print("Japanese")
    print("ニホンゴ")
    print("日本語")
    for i in range(10):
        click.echo(f"{i:>5}\tHello, World!")
    click.confirm("Do you want to say hello?", abort=True)
    print("hello")


if __name__ == "__main__":
    main()
