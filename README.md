# Rustey

A small, component-driven Discord bot written in Rust using the poise framework and SurrealDB for local storage.
The main focus for this project is to make it extendable.

## Features

The bot currently has the following commands:
- `/config` to configure the todo component (will be moved to `/todo`)
- `/moderation`
  - `ban` ban someone from the server (admin only)
  - `kick` kick someone from the server (admin only)
  - `timeout` timeout someone in the server (admin only)
  - `user` print info about a user
- `/ping` pong!
- `/todo`
  - `add` add something to your todo list
  - `list` show your todo list
  - `remove` remove something from your todo list (BROKEN RN BECAUSE THERE HASN'T BEEN A SURREALDB RELEASE)
- `/toggle_component` turn individual components on/off (owner only)
- `!register_commands` register the commands with discord. Only required one time. (owner only)

## Docker

I haven't published a docker image yet, sorry. You can build your own using the provided `Dockerfile`.

## Prerequisites

- Rust toolchain (stable) with `cargo` (install via rustup: https://rustup.rs)
- A Discord bot token (set as `DISCORD_TOKEN` environment variable)

This project uses SurrealDB in local mode (the database is stored in the `database/` directory by default).

## Build & Run

1. Clone the repository:

```bash
git clone <this-repo-url>
cd RustDiscordBot
```

2. Set the Discord token and any optional environment variables:

```bash
export DISCORD_TOKEN="your_discord_bot_token"
# Optional overrides (defaults shown):
export DB_PATH="./database"         # default: database/
export DB_NAMESPACE="rust_discord_bot" # default in code
export DB_DATABASE="main"          # default in code
export PREFIX="!"                  # default command prefix
```

3. Run in development mode:

```bash
cargo run
```

Or build a release binary:

```bash
cargo build --release
./target/release/Rustey
```

The bot will initialize a local SurrealDB file at the `DB_PATH` and will attempt to use the configured namespace and database.

## Environment variables

- DISCORD_TOKEN (required) — the bot token used to authenticate with Discord.
- DB_PATH (optional) — path to the local SurrealDB store (default: `database`).
- DB_NAMESPACE (optional) — SurrealDB namespace to use (default: `rust_discord_bot`).
- DB_DATABASE (optional) — SurrealDB database name (default: `main`).
- PREFIX (optional) — command prefix for the bot (default: `!`).

These settings are loaded by `src/environment.rs` (see `Environment::load_env`).
