docu is combined junk of cursor, copilot, mistreal, gemini

# Anode

Local-first novelist writing app — **Tauri 2**, **Svelte 5**, **TipTap**, **Rust** (`anode-core`).

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install) (stable)
2. [Node.js](https://nodejs.org/) 20+
3. [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS (WebView2 on Windows, etc.)

## Development

```bash
npm install
npm run tauri dev
```

## Project layout

```
Anode/
├── crates/anode-core/   # Library, books, pages, SQLite, atomic saves
├── src/                 # SvelteKit frontend
├── src-tauri/           # Tauri shell + commands
└── LICENSE              # MIT
```

## v0.1 foundation (implemented)

- First-run library path wizard
- Home: book list, create book
- Book home: plan / write / read tabs, page sidebar, search bar placeholder
- TipTap editor with Word-like **pages mode** (paper rectangle)
- Autosave to `pages/*.body.json` (TipTap JSON)
- Compile order panel (include/exclude write pages)
- Status bar: mode, word counts, save state, clock
- Themes: system / light / dark / sepia
- i18n string table (`src/lib/i18n/en.json`)

## Library on disk

```
{library}/
├── library.db
└── books/{uuid}/
    ├── book.meta.json
    ├── book.db
    ├── pages/
    │   ├── {page}.meta.json
    │   └── {page}.body.json
    └── snapshots/
```

## Icons

If `src-tauri/icons/` is missing, add a 1024×1024 PNG and run:

```bash
npm run tauri icon path/to/icon.png
```

## License

MIT — see [LICENSE](LICENSE).
