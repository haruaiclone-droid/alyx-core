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

`build-web` writes a deployable static bundle to the destination directory (`index.html`, `manifest.json`, `alyx-manifest.json`, and `script` tags for event handling).

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
