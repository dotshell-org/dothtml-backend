import os
from pathlib import Path
import typer
from dotenv import load_dotenv
from rich.console import Console
from rich.table import Table

app = typer.Typer()
console = Console()

@app.callback(invoke_without_command=True)
def main(ctx: typer.Context):
    # Load the .env file
    env_path = Path(__file__).parent.parent / '.env'
    load_dotenv(env_path)

    # If no command is provided, display help
    if ctx.invoked_subcommand is None:
        typer.echo(ctx.get_help())
        raise typer.Exit()

@app.command("list")
def list_items(
    what: str = typer.Argument(..., help="What to list ('accounts' or 'keys')"),
    uuid: str = typer.Argument(None, help="UUID for which to list keys")
):
    """List items from the system"""
    if what == "accounts":
        list_accounts()
    elif what == "keys":
        if uuid is None:
            console.print("[red]Error: UUID is required for listing keys[/]", style="bold red")
            raise typer.Exit(1)
        list_keys(uuid)
    else:
        console.print("[red]Error: Only 'accounts' and 'keys' listings are supported[/]", style="bold red")
        raise typer.Exit(1)

def list_accounts():
    """List accounts from the system"""
    import psycopg2
    from psycopg2.extras import DictCursor

    sql_query = """
        SELECT
            a.id as uuid, a.identifier,
            COALESCE(COUNT(pk.id), 0) as key
        FROM accounts a
        LEFT JOIN public_keys pk ON a.identifier = pk.identifier
        GROUP BY a.id, a.identifier
    """

    database_url = os.getenv('DATABASE_URL')
    if not database_url:
        console.print("[red]Error: DATABASE_URL not found in .env file[/]", style="bold red")
        raise typer.Exit(1)

    try:
        conn = psycopg2.connect(database_url)
        cur = conn.cursor(cursor_factory=DictCursor)
        cur.execute(sql_query)
        results = cur.fetchall()

        if results:
            table = Table(
                title="Accounts List",
                title_style="bold magenta",
                border_style="white",
                header_style="bold white",
                row_styles=["", "dim"],
                show_lines=True
            )
            for column in [desc[0] for desc in cur.description]:
                table.add_column(column, style="#808080")

            for row in results:
                row_values = list(row)
                key_count = int(row_values[-1])
                if key_count == 0:
                    row_values[-1] = "[red]NOT CONNECTED[/red]"
                else:
                    row_values[-1] = f"[green]CONNECTED[/green]"
                table.add_row(*[str(val) for val in row_values])

            console.print("\n")
            console.print(table)
            console.print(f"\n[bold green]Total results:[/] {len(results)} \n")
        else:
            console.print("[yellow]No account found[/yellow]")

    except psycopg2.Error as e:
        console.print(f"[red]Database error:[/red] {e}", style="bold red")
        raise typer.Exit(1)
    finally:
        if 'cur' in locals():
            cur.close()
        if 'conn' in locals():
            conn.close()

def list_keys(uuid: str):
    """List public keys associated with a given UUID"""
    import psycopg2
    from psycopg2.extras import DictCursor

    sql_query = """
        SELECT pk.id as key_id, pk.public_key
        FROM public_keys pk
        JOIN accounts a ON pk.identifier = a.identifier
        WHERE a.id = %s
    """

    database_url = os.getenv('DATABASE_URL')
    if not database_url:
        console.print("[red]Error: DATABASE_URL not found in .env file[/]", style="bold red")
        raise typer.Exit(1)

    try:
        conn = psycopg2.connect(database_url)
        cur = conn.cursor(cursor_factory=DictCursor)
        cur.execute(sql_query, (uuid,))
        results = cur.fetchall()

        if results:
            table = Table(
                title=f"Public Keys for UUID: {uuid}",
                title_style="bold magenta",
                border_style="white",
                header_style="bold white",
                row_styles=["", "dim"],
                show_lines=True
            )
            table.add_column("uuid", style="#808080")
            table.add_column("public_key", style="#808080")

            for row in results:
                table.add_row(row[0], row[1])

            console.print("\n")
            console.print(table)
            console.print(f"\n[bold green]Total keys found:[/] {len(results)} \n")
        else:
            console.print("[yellow]No keys found for the given UUID[/yellow]")

    except psycopg2.Error as e:
        console.print(f"[red]No account found with uuid \"{uuid}\"[/red]", style="bold red")
        raise typer.Exit(1)
    finally:
        if 'cur' in locals():
            cur.close()
        if 'conn' in locals():
            conn.close()

@app.command("add")
def add_item(
    what: str = typer.Argument(..., help="What to add ('account')"),
    identifier: str = typer.Argument(..., help="Identifier for the item to add (e.g., account identifier)")
):
    """Add items to the system"""
    if what == "account":
        add_account(identifier)
    else:
        console.print("[red]Error: Only 'account' addition is supported[/]", style="bold red")
        raise typer.Exit(1)

def add_account(identifier: str):
    """Add a new account to the system."""
    import psycopg2
    import uuid

    # Retrieve the database URL from the .env file
    database_url = os.getenv("DATABASE_URL")
    if not database_url:
        console.print("[red]Error: DATABASE_URL not found in .env file[/]", style="bold red")
        raise typer.Exit(1)

    connection = None
    cursor = None
    try:
        connection = psycopg2.connect(database_url)
        cursor = connection.cursor()

        # Generate a new UUID for the account
        new_uuid = str(uuid.uuid4())

        # Insert the new account into the accounts table
        cursor.execute(
            "INSERT INTO accounts (id, identifier) VALUES (%s, %s) RETURNING id",
            (new_uuid, identifier),
        )
        result = cursor.fetchone()
        connection.commit()

        # Provide feedback about the added account
        if result:
            console.print(
                f"[green]Account '{identifier}' created successfully[/green]"
            )
        else:
            console.print("[red]Account creation failed[/red]", style="bold red")

    except psycopg2.errors.UniqueViolation:
        if connection:
            connection.rollback()
        console.print(
            f"[red]Error: An account with identifier '{identifier}' already exists[/red]",
            style="bold red",
        )
        raise typer.Exit(1)

    except psycopg2.Error as db_error:
        if connection:
            connection.rollback()
        console.print(f"[red]Database error: {db_error}[/red]", style="bold red")
        raise typer.Exit(1)

    finally:
        # Ensure resources are properly cleaned up
        if cursor:
            cursor.close()
        if connection:
            connection.close()

@app.command("remove")
def remove_item(
    what: str = typer.Argument(..., help="What to remove ('account')"),
    uuid: str = typer.Argument(..., help="UUID of the item to remove")
):
    """Remove items from the system"""
    if what == "account":
        remove_account(uuid)
    else:
        console.print("[red]Error: Only 'account' removal is supported[/]", style="bold red")
        raise typer.Exit(1)

def remove_account(uuid: str):
    """Remove an account and all associated keys by account UUID"""
    import psycopg2

    database_url = os.getenv('DATABASE_URL')
    if not database_url:
        console.print("[red]Error: DATABASE_URL not found in .env file[/]", style="bold red")
        raise typer.Exit(1)

    conn = None
    cur = None
    try:
        conn = psycopg2.connect(database_url)
        cur = conn.cursor()
        conn.autocommit = False  # Start a transaction

        # Get the identifier for the account
        cur.execute("SELECT identifier FROM accounts WHERE id = %s", (uuid,))
        account_data = cur.fetchone()

        if not account_data:
            console.print(f"[yellow]No account found with UUID '{uuid}'[/yellow]")
            conn.rollback()
            return

        identifier = account_data[0]

        # Delete associated public keys
        cur.execute("DELETE FROM public_keys WHERE identifier = %s", (identifier,))
        keys_deleted_count = cur.rowcount
        console.print(f"[green]{keys_deleted_count} key(s) associated with account '{uuid}' removed[/green]")

        # Delete the account
        cur.execute("DELETE FROM accounts WHERE id = %s RETURNING id", (uuid,))
        account_deleted_id = cur.fetchone()

        if account_deleted_id:
            console.print(f"[green]Account with UUID '{uuid}' removed successfully[/green]")
            conn.commit()
        else:
            # This case should ideally not happen if account_data was found, but good for robustness
            console.print(f"[red]Failed to remove account with UUID '{uuid}'. Rolling back[/red]")
            conn.rollback()

    except psycopg2.Error as e:
        if conn:
            conn.rollback()
        console.print(f"[red]Database error:[/red] {e}", style="bold red")
        raise typer.Exit(1)
    finally:
        if cur:
            cur.close()
        if conn:
            conn.close()

@app.command("disconnect")
def disconnect(uuid: str = typer.Argument(..., help="UUID of the account to disconnect public key")):
    """Remove the connected public_key from the account with given UUID"""
    import psycopg2

    database_url = os.getenv('DATABASE_URL')
    if not database_url:
        console.print("[red]Error: DATABASE_URL not found in .env file[/]", style="bold red")
        raise typer.Exit(1)

    conn = None
    cur = None
    try:
        conn = psycopg2.connect(database_url)
        cur = conn.cursor()

        # Get the identifier of the account by UUID
        cur.execute("SELECT identifier FROM accounts WHERE id = %s", (uuid,))
        account_data = cur.fetchone()

        if not account_data:
            console.print(f"[yellow]No account found with UUID '{uuid}'[/yellow]")
            raise typer.Exit(1)

        identifier = account_data[0]

        # Delete the public_key linked to this identifier
        cur.execute("DELETE FROM public_keys WHERE identifier = %s RETURNING id", (identifier,))
        deleted_keys = cur.fetchall()
        conn.commit()

        if deleted_keys:
            count = len(deleted_keys)
            console.print(f"[green]{count} public key(s) disconnected from account '{uuid}' successfully[/green]")
        else:
            console.print(f"[yellow]No public key connected to account '{uuid}'[/yellow]")

    except psycopg2.Error as e:
        if conn:
            conn.rollback()
        console.print(f"[red]Database error:[/red] {e}", style="bold red")
        raise typer.Exit(1)
    finally:
        if cur:
            cur.close()
        if conn:
            conn.close()

if __name__ == "__main__":
    app()
