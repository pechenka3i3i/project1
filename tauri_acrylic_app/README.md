# Tauri v2 Acrylic App

A simple Tauri v2 application demonstrating Windows Acrylic material effect.

## Prerequisites

- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs)
- **Node.js** (16+): Install from [nodejs.org](https://nodejs.org)
- For Windows development: Windows 10/11 with DWM support

## Project Structure

```
tauri_acrylic_app/
├── src/                      # Frontend source
│   └── index.html           # Main HTML file
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   └── commands.rs     # Tauri commands
│   ├── icons/              # App icons
│   ├── Cargo.toml          # Rust dependencies
│   ├── build.rs            # Build script
│   └── tauri.conf.json     # Tauri configuration
└── package.json            # Node.js dependencies
```

## Installation & Running

1. **Install Node.js dependencies:**
   ```bash
   cd tauri_acrylic_app
   npm install
   ```

2. **Run in development mode:**
   ```bash
   npm run tauri dev
   ```

3. **Build for production:**
   ```bash
   npm run tauri build
   ```

## Features

- **Acrylic Effect**: On Windows, the window uses DWM blur behind for an acrylic-like translucent effect
- **Transparent Background**: The frontend uses CSS transparency and backdrop-filter
- **Tauri v2**: Latest version with improved APIs
- **System Tray**: Includes a system tray icon with menu
- **Rust Commands**: Example greet command callable from JavaScript

## How the Acrylic Effect Works

The acrylic effect is implemented using Windows Desktop Window Manager (DWM):

1. In `main.rs`, the `apply_acrylic_effect()` function is called on Windows platforms
2. It uses the `DwmEnableBlurBehindWindow` API to enable blur behind the window
3. Combined with CSS `backdrop-filter: blur()` in the frontend, this creates the acrylic look

## Notes

- The acrylic effect only works on Windows 10/11
- On other platforms, the window will appear with standard transparency
- For best results, use a colorful desktop wallpaper to see the effect

## License

MIT
