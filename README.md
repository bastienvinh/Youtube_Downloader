# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Prerequisites

Before running the application, ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable version)
- [Trunk](https://trunkrs.dev/) - Install with: `cargo install trunk`
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/) - Install with: `cargo install tauri-cli`
- System dependencies for Tauri (varies by OS - see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/))

## Running the Application

### Development Mode

To run the application in development mode:

```bash
cargo tauri dev
```

This will:
- Build the Leptos frontend using Trunk
- Run Tailwind CSS to generate styles
- Start the Tauri application with hot-reload enabled

### Building for Production

To build the application for production:

```bash
cargo tauri build
```

The compiled application will be available in `src-tauri/target/release/`.

### Frontend Only (Web View)

To run just the frontend in a browser for development:

```bash
trunk serve
```

Then open your browser to `http://localhost:1420`

## Project Structure

- `src/` - Leptos frontend code
- `src-tauri/` - Tauri backend code
- `index.html` - HTML entry point
- `input.css` - Tailwind CSS input file
- `Trunk.toml` - Trunk configuration
