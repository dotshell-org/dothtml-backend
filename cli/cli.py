
import typer

app = typer.Typer()

@app.callback()
def main():
    """Welcome message when CLI is executed."""
    typer.echo("Welcome to the dothtml-backend-cli.")

if __name__ == "__main__":
    app()