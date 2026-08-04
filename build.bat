@echo off
echo ========================================
echo PLANILORE 2025 - Build Script
echo ========================================
echo.

REM Controlla se Node.js è installato
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js non trovato. Installa Node.js da https://nodejs.org/
    pause
    exit /b 1
)

REM Controlla se Rust è installato
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust non trovato. Installa Rust da https://rustup.rs/
    pause
    exit /b 1
)

REM Installa dipendenze npm se necessario
if not exist node_modules (
    echo 📦 Installazione dipendenze npm...
    npm install
    if errorlevel 1 (
        echo ❌ Errore nell'installazione delle dipendenze npm
        pause
        exit /b 1
    )
)

echo ✅ Dipendenze installate correttamente

REM Costruisci l'app per Windows
echo 🔨 Costruzione PLANILORE 2025...
npm run tauri:build:release

if errorlevel 1 (
    echo ❌ Errore durante la costruzione
    pause
    exit /b 1
)

echo.
echo ✅ Build completato con successo!
echo.
echo 📁 Il file installabile si trova in:
echo    src-tauri\target\release\bundle\msi\PLANILORE2025_1.0.0_x64_en-US.msi
echo.
echo 🚀 Puoi distribuire il file .msi agli utenti Windows
echo.
echo 📋 Per installare:
echo    1. Esegui il file .msi
    2. Segui le istruzioni di installazione
    3. Trova PLANILORE 2025 nel menu Start
echo.
pause
