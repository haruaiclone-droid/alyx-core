# Web Hosting

`alyx-host` provides:

- `build_web` to export static files
- `build_static_dist` for `index.html` + manifest
- `serve_http` for local preview serving
- `serve_http_with_runtime` for runtime-backed local preview with event bridge
- CLI flow is exposed from the `alyx` binary in the `alyx-cli` crate:
  - `cargo run --package alyx-cli -- --help`
  - `cargo run --package alyx-cli -- build-web dist`
  - `cargo run --package alyx-cli -- serve 3000 dist`
  - `cargo run --package alyx-cli -- serve dist`

## Deploying a static dist

`build-web` writes a deployable static bundle to the destination directory:

- `index.html`
- `manifest.json`
- `alyx-manifest.json` (contains `entry` and `renderer` metadata)
- `app.wasm`
- `alyx-loader.js`

Note:
- `build-web` attempts to compile the configured example into `app.wasm` for browser delivery (default: `web_counter`).
- If wasm build support is unavailable in the environment, the command falls back to a placeholder `app.wasm` for compatibility.

Deployment output must include:

- `index.html`
- `manifest.json` and `alyx-manifest.json` (`entry` and `renderer` metadata are in `alyx-manifest.json`)
- `app.wasm`
- `alyx-loader.js`

To deploy:

- Vercel
  - Configure build output directory as `dist` in the project settings.
  - Use a build step that runs `cargo run --package alyx-cli -- build-web dist`.
  - Set the project output/public directory to `dist`.
- Netlify
  - Build: `cargo run --package alyx-cli -- build-web dist`
  - Publish directory: `dist`
- GitHub Pages / custom static host
  - Run `cargo run --package alyx-cli -- build-web dist`
  - Upload the `dist` directory.

For local preview of runtime-integrated pages, start `serve` and open `http://127.0.0.1:<port>/index.html`.
