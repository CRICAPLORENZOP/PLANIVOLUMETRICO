# PLANILORE 2025 - Calcolo Planivolumetrico

**PLANILORE 2025** è un'applicazione desktop Windows professionale per il calcolo planivolumetrico, creata con Tauri.

## 🚀 Caratteristiche Principali

- ✅ **Calcolo completo planivolumetrico** con 13 sezioni specializzate
- ✅ **Interfaccia moderna** con sidebar navigabile
- ✅ **Esportazione dati** in JSON, Excel e HTML
- ✅ **Importazione/backup** dati da file JSON
- ✅ **Modalità singola e multi-sezione**
- ✅ **Design responsive** ottimizzato per Windows
- ✅ **Funziona offline** dopo il primo caricamento

## 📋 Sezioni Disponibili

1. **Superficie Intervento** - Calcolo superficie totale
2. **Sup Coperta Massima** - Determinazione superficie coperta massima
3. **Volumetria Massima** - Calcolo volumetria realizzabile
4. **Sup Drenante Minima** - Superficie drenante necessaria
5. **Sup Coperta in Progetto** - Superficie coperta nel progetto
6. **Sup Lorda in Progetto** - Determinazione S.L.
7. **Volume Urbanistico** - Calcolo volume urbanistico
8. **Sup Parcheggio Richiesto** - Parcheggi richiesti
9. **Sup Accessoria in Progetto** - Superficie accessoria S.A.
10. **Volumetria Complessiva** - Volume totale progetto
11. **Sup Parcheggio in Progetto** - Parcheggi nel progetto
12. **Sup Drenante in Progetto** - Superficie drenante progetto
13. **Verifiche Finali** - Controllo conformità

## 🛠️ Sviluppo

### Prerequisiti

- [Node.js](https://nodejs.org/) (versione LTS)
- [Rust](https://rustup.rs/) (installa tramite rustup)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Installazione

1. Installa le dipendenze:
```bash
npm install
```

2. Avvia in modalità sviluppo:
```bash
npm run tauri:dev
```

3. Costruisci per produzione:
```bash
npm run tauri:build:release
```

### Build Rapido

Esegui semplicemente:
```bash
build.bat
```

Il file installabile `.msi` sarà nella cartella `src-tauri/target/release/bundle/msi/`.

## 📦 Distribuzione

Il file `PLANILORE2025_1.0.0_x64_en-US.msi` può essere distribuito agli utenti Windows.

### Installazione per l'utente finale:
1. Esegui il file `.msi`
2. Segui le istruzioni di installazione
3. Trova **PLANILORE 2025** nel menu Start

## 🏗️ Struttura del Progetto

- `src/` - Applicazione web (HTML, CSS, JS)
- `src-tauri/` - Codice Rust per Tauri
- `src/Index.html` - File principale dell'applicazione
- `build.bat` - Script di build automatico

## 📝 Note Tecniche

- L'app funziona completamente offline
- I dati possono essere esportati e importati per backup
- Interfaccia ottimizzata per Windows con dimensioni minime 1200x800
- Supporta esportazione in Excel con fogli separati per sezione

## 🎯 Versione 1.0.0

- Prima versione stabile di PLANILORE 2025
- Tutte le funzionalità di calcolo planivolumetrico
- Interfaccia moderna e intuitiva
- Sistema di esportazione completo

## 🔄 Aggiornamenti automatici

L'app controlla le [GitHub Releases](https://github.com/CRICAPLORENZOP/PLANIVOLUMETRICO/releases) all'avvio.
Se c'è una versione più nuova, chiede all'utente se vuole aggiornare.

### Come pubblicare una nuova versione

1. Aumenta il numero in `src-tauri/tauri.conf.json` → `package.version` (es. `0.1.1`)
2. Crea e pubblica un tag:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```
3. Oppure da GitHub: **Actions → Release PLANILORE → Run workflow**
4. GitHub Actions costruisce l'installer Windows e crea la Release (con `latest.json` per l'updater)

Repository: https://github.com/CRICAPLORENZOP/PLANIVOLUMETRICO
