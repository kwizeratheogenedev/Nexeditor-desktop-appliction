# Nexeditor Desktop Application (Tauri)

This folder will contain the Tauri-based desktop version of the Nexteditor web app.

Purpose
- Convert the existing web MERN-based Nexteditor into a desktop application using Tauri.
- Implement all current backend/frontend features except the Editor export (deferred).

High-level plan
1. Initialize a Tauri + Vite (React) scaffold.
2. Copy and adapt the frontend from `Nexteditor/frontend` into Tauri's web app.
3. Integrate backend processing (FFmpeg flows, yt-dlp, Whisper) into a desktop-friendly runner:
   - Option A: Bundle Node backend as a child process launched by the Tauri Rust backend.
   - Option B: Reimplement critical endpoints as Rust commands invoked directly.
   (We'll start with option A for faster porting.)
4. Wire `socket.io` or use native Tauri events for progress reporting.
5. Adjust file storage to use an app data folder and expose downloads.
6. Bundle or detect native binaries (`ffmpeg`, `ffprobe`, `yt-dlp`) and provide install hints.
7. Implement persistence for jobs and stable download URLs.
8. Add packaging scripts for Windows/macOS/Linux.

Prerequisites
- Node.js (16+)
- Rust toolchain + `cargo` and `rustup`
- Tauri prerequisites (platform toolchains)

Quick setup (dev)
```bash
# from this folder
cd "Nexeditor desktop application"
# Create tauri app (example using create-tauri-app)
npm init tauri@latest
# or follow Tauri docs: https://tauri.app/v1/guides/getting-started/prerequisites
```

Next steps
- I will scaffold the Tauri app and copy the existing `frontend` build into the Tauri web folder, then wire a Node backend runner. Do you want to proceed with scaffolding now (Option A: bundle Node backend as a child process)?