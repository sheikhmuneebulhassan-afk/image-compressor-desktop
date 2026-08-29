@echo off
setlocal
where node >nul 2>nul || (echo Node.js is required. & exit /b 1)
where cargo >nul 2>nul || (echo Rust is required. Install from https://rustup.rs & exit /b 1)
call npm install || exit /b 1
call npm run build:web || exit /b 1
call npx tauri build --bundles nsis,msi
endlocal
