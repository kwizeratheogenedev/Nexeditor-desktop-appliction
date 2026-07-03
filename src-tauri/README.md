Node backend runner

This Tauri app spawns the existing Node backend during development by running `node ../Nexteditor/backend/server.js` from the Tauri runtime.

Notes:
- Development: ensure `node` is available in PATH and the `Nexteditor/backend` folder is present alongside this project. Run `npm run tauri:dev` from the desktop project root.
- Note: The local backend will attempt port 3000 first; if occupied it falls back to 3001. The Tauri runner injects `VITE_API_URL` and `window.__API_URL` set to `http://127.0.0.1:3001` by default.
- Production: you should bundle or package the Node backend with the app, or provide a native Rust reimplementation. Adjust `src-tauri/src/main.rs` to point to the packaged backend binary or script.
- The webview reads `VITE_API_URL` at runtime; currently the runner sets it to `http://127.0.0.1:3000`.
