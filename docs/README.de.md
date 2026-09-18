# AutoStarter

<!-- language-switcher -->
[English](../README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · [Español](README.es.md) · [العربية](README.ar.md) · [Português](README.pt.md) · [Français](README.fr.md) · **Deutsch** · [日本語](README.ja.md) · [हिन्दी](README.hi.md)

![AutoStarter — Ihre Programme, Ihre Startreihenfolge. Ein Windows-Manager auf Basis von Tauri und Rust.](images/hero.svg)

**Startmanager für Programme unter Windows mit Einstellung des Fensterverhaltens und Unterstützung des Infobereichs.**

AutoStarter (in der Oberfläche AutoStart Manager) startet die ausgewählten Anwendungen beim eigenen Start. Fügen Sie die benötigten Programme hinzu, konfigurieren Sie ihr Verhalten und aktivieren Sie den Autostart des Managers selbst — dann wird Ihr Anwendungsset bei der Anmeldung an Windows gestartet.

Das Projekt basiert auf **Tauri 1, Rust, TypeScript und Vite 5**.

> **Nur Windows.** Die Anwendung nutzt die Registrierung und die Win32-API; das Ausführen und Bauen der Desktop-Version unter Linux und macOS wird in der aktuellen Implementierung nicht unterstützt.

## Inhalt

- [Funktionen](#funktionen)
- [Schnellstart](#schnellstart)
- [Startmodi der Programme](#startmodi-der-programme)
- [Einstellungen des Managers](#einstellungen-des-managers)
- [Installation und Bauen aus den Quellen](#installation-und-bauen-aus-den-quellen)
- [Datenspeicherung und Windows-Autostart](#datenspeicherung-und-windows-autostart)
- [Häufige Fragen](#häufige-fragen)
- [Projektstruktur](#projektstruktur)
- [Mitwirken](#mitwirken)
- [Lizenz](#lizenz)

## Funktionen

- **Eigene Programmliste:** Hinzufügen von `.exe`-Dateien, `.lnk`-Verknüpfungen und `.bat`- / `.cmd`-Skripten über den Dateiauswahldialog.
- **Unabhängiges Aktivieren:** Schließen Sie ein Programm vorübergehend vom Start aus, ohne es aus der Liste zu löschen.
- **Fenstersteuerung:** normaler, minimierter oder verborgener Start sowie das Senden eines Schließbefehls, sobald der Prozess erkannt wurde.
- **Start bei der Windows-Anmeldung:** Autostart des Managers für den aktuellen Benutzer.
- **Infobereich-Unterstützung:** Hauptfenster ausblenden, wiederherstellen und über das Kontextmenü des Symbols beenden.
- **Modus ohne Oberfläche:** startet die aktivierten Programme und beendet den Manager nach 30 Sekunden.
- **Lokale Einstellungen:** Programmliste und Parameter werden in JSON-Dateien gespeichert.
- **10 Oberflächensprachen:** Russisch, Englisch, Chinesisch, Spanisch, Arabisch, Portugiesisch, Französisch, Deutsch, Japanisch und Hindi.

## Schnellstart

![Drei Schritte: Programme hinzufügen und ihre Karten aktivieren, Modi und den Autostart des Managers einstellen, sich bei Windows anmelden. Ein manueller Start des Managers startet ebenfalls die aktivierten Einträge.](images/workflow.svg)

Wenn die Anwendung noch nicht gebaut wurde, folgen Sie zuerst der [Bauanleitung](#installation-und-bauen-aus-den-quellen).

1. Starten Sie `tauri-app.exe` — so heißt die ausführbare Datei des Projekts derzeit.
2. Öffnen Sie die Einstellungen über die Schaltfläche unten rechts und wählen Sie **Language → Deutsch**.
3. Klicken Sie auf **„+ Hinzufügen“** und wählen Sie ein Programm oder Skript aus.
4. Markieren Sie bei Bedarf **„Minimiert“**, **„Verborgen“** oder **„Schließen“** auf der Programmkarte.
5. Stellen Sie den Schalter der Karte auf **„On“**. Neue Einträge sind standardmäßig deaktiviert.
6. Aktivieren Sie in den Einstellungen **„Autostart“**, wenn der Manager zusammen mit den ausgewählten Programmen bei der Windows-Anmeldung starten soll.

Änderungen werden automatisch gespeichert. Die Schaltfläche zum Löschen entfernt nur den Eintrag aus der Liste — die Programmdatei bleibt auf der Festplatte.

> **Wichtig:** Aktivierte Programme werden bei jedem neuen Start des AutoStarter-Prozesses ausgeführt, auch beim manuellen Start. Das Hinzufügen eines Eintrags oder das Umschalten auf „On“ startet ihn nicht sofort. Um die Einstellung zu prüfen, beenden Sie den Manager vollständig über **Quit** im Infobereich und öffnen Sie ihn erneut. Bereits laufende Anwendungen können ein zweites Mal gestartet werden.

## Startmodi der Programme

| Modus auf der Karte | Verhalten |
| --- | --- |
| Ohne zusätzliche Markierungen | Normaler Start über `cmd /c start`. |
| **Minimiert** | Start über `cmd /c start /min` mit der Anforderung eines minimierten Fensters. |
| **Verborgen** | Start über PowerShell `Start-Process -WindowStyle Hidden`. Das ist eine Anforderung, das Fenster auszublenden, keine Garantie, dass das Programm im Infobereich erscheint. |
| **Schließen** | Normaler Start, danach Suche nach dem Prozess anhand des Dateinamens für etwa 30 Sekunden. Nach der Erkennung wartet der Manager weitere 0,5 Sekunden und sendet den System-Schließbefehl an die sichtbaren Fenster des Prozesses. |
| **Off / On** | Schließt den Eintrag beim nächsten Start des Managers vom Start aus oder nimmt ihn auf. |

**Besonderheiten und Einschränkungen:**

- „Minimiert“ und „Verborgen“ schließen sich in der Oberfläche gegenseitig aus.
- „Schließen“ hat Vorrang vor beiden Modi und bedeutet kein erzwungenes Beenden des Prozesses. Die Zielanwendung kann sich schließen, in den eigenen Infobereich minimieren, einen Dialog anzeigen oder die Anforderung ignorieren.
- Die Suche für „Schließen“ erfolgt anhand des Dateinamens, nicht anhand des vollständigen Pfads oder der Kennung der gestarteten Instanz. Daher kann der Befehl einen bereits laufenden Prozess mit demselben Namen betreffen. Bei Verknüpfungen und Skripten stimmt der Name der ausgewählten Datei in der Regel nicht mit dem echten Prozessnamen überein.
- Manche Anwendungen verwalten ihre Fenster selbst und können einen verborgenen oder minimierten Start ignorieren.
- Das Einstellen von Befehlszeilenargumenten, Arbeitsverzeichnis und individuellen Verzögerungen ist in der Oberfläche nicht vorgesehen.

Fügen Sie nur vertrauenswürdige Programme und Skripte hinzu. Verwenden Sie „Schließen“ nicht ungeprüft für Anwendungen mit ungespeicherten Daten.

## Einstellungen des Managers

| Einstellung | Wirkung |
| --- | --- |
| **Sprache** | Ändert die Sprache der Oberfläche; standardmäßig ist Englisch ausgewählt. |
| **Im Infobereich starten** | Blendet beim Start der normalen Version das Hauptfenster aus und zeigt ein Symbol im Infobereich an. |
| **Autostart** | Fügt AutoStarter dem Windows-Autostart für den aktuellen Benutzer hinzu. Standardmäßig deaktiviert. |
| **Nach Ausführung schließen** | Fügt beim Windows-Autostart den Schalter `--close-after-30` hinzu: Der Manager startet die aktivierten Programme, wartet 30 Sekunden und beendet sich, ohne Oberfläche und Infobereichssymbol zu erzeugen. Nur bei aktiviertem „Autostart“ verfügbar. |

**„Schließen“ auf der Karte und „Nach Ausführung schließen“ in den Einstellungen sind unterschiedliche Funktionen:** Die erste bezieht sich auf die Fenster des ausgewählten Programms, die zweite auf den Manager selbst. Das Intervall von 30 Sekunden ist fest und bedeutet nicht, dass auf das Ende aller gestarteten Anwendungen gewartet wird.

Ein normaler manueller Start ohne den Schalter öffnet den Manager gemäß der Einstellung „Im Infobereich starten“, auch wenn „Nach Ausführung schließen“ aktiviert ist.

### Steuerung über den Infobereich

- **Das Schließen-Kreuz des Fensters** blendet den Manager in den Infobereich aus, statt ihn zu beenden.
- **Ein Klick auf das Symbol** oder der Eintrag **Show** holt das Hauptfenster zurück.
- **Rechtsklick → Quit** beendet den Manager vollständig.

Die Namen der Menüeinträge im Infobereich werden derzeit unabhängig von der Oberflächensprache auf Englisch angezeigt.

## Installation und Bauen aus den Quellen

### Voraussetzungen

Für die grafische Version wird die **Microsoft Edge WebView2 Runtime** benötigt. Eine praktikable Bauumgebung ist Windows 10/11 mit folgender Installation:

- [Node.js](https://nodejs.org/) **20 oder neuer** und npm; die aktuelle LTS-Version wird empfohlen.
- [Rust](https://rustup.rs/) — aktuelle Stable-Toolchain für Windows MSVC. In `Cargo.toml` ist mindestens 1.70 angegeben, Abhängigkeiten aus der Lock-Datei können jedoch eine neuere Version erfordern.
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) mit den Komponenten **Desktop development with C++**, MSVC und Windows SDK.
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).
- [Git](https://git-scm.com/) zum Klonen des Repositorys.

Weitere Informationen: [Einrichten der Tauri-1-Umgebung für Windows](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows).

### Quellen beziehen

Führen Sie in PowerShell aus:

```powershell
git clone https://github.com/BORGERone/AutoStarter.git
cd AutoStarter
npm ci
```

`npm ci` installiert die Abhängigkeiten gemäß `package-lock.json`. Der erste Build benötigt außerdem Netzwerkzugriff, um Rust-Abhängigkeiten und Paketierwerkzeuge herunterzuladen.

### Entwicklungsmodus

```powershell
npm run tauri-dev
```

Oder starten Sie `dev.bat` aus dem Explorer. Tauri startet Vite automatisch auf Port **5174** und öffnet das Desktop-Fenster.

> `npm run dev` startet nur die Weboberfläche. In einem normalen Browser sind die Tauri-Funktionen nicht verfügbar: Dateiauswahl über den nativen Dialog, Speichern der Einstellungen, Steuerung von Infobereich und Registrierung. Verwenden Sie `npm run tauri-dev` für eine vollständige Prüfung der Anwendung.

### Build mit Installationsprogrammen

```powershell
npm run tauri-build
```

Der Befehl baut die Oberfläche, die Rust-Anwendung und die von der Tauri-Konfiguration festgelegten Pakete. Die Ergebnisse liegen unter:

- `src-tauri/target/release/tauri-app.exe` — die ausführbare Datei;
- `src-tauri/target/release/bundle/` — die vom Bundler erstellten Installationspakete.

### Build ohne Installationsprogramm

```powershell
.\build.bat
```

Das Skript führt nacheinander `npm run build` und `cargo build --release` im Verzeichnis `src-tauri` aus. Das Ergebnis ist `src-tauri/target/release/tauri-app.exe`. Für die grafische Oberfläche wird weiterhin die WebView2 Runtime benötigt.

Manueller Start des Modus ohne Oberfläche:

```powershell
.\src-tauri\target\release\tauri-app.exe --close-after-30
```

### Prüfung der Oberfläche

```powershell
npm run build
```

Der Befehl führt die TypeScript-Prüfung und den Vite-Produktionsbuild aus. Er prüft weder den Rust-Code noch das Verhalten der Win32-API. Ein separates Skript für automatische Tests gibt es in `package.json` derzeit nicht.

## Datenspeicherung und Windows-Autostart

Die Einstellungen des aktuellen Benutzers werden im Verzeichnis gespeichert:

```text
%APPDATA%\tauri-launcher\
├── programs.json    # Programmliste, Pfade und Startmodi
└── settings.json    # Sprache und Einstellungen des Managers
```

Beenden Sie für eine Sicherung den Manager vollständig und kopieren Sie beide Dateien. Prüfen Sie beim Übertragen auf einen anderen Rechner die Programmpfade. Die Autostart-Parameter in der Registrierung gehören nicht zu dieser Kopie: Aktivieren Sie den Autostart nach der Übertragung erneut aus der gewünschten Instanz der Anwendung.

Beim Aktivieren von „Autostart“ wird der Wert **`AutoStartManager`** im Schlüssel erstellt:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
```

Darin werden der Pfad zur aktuellen ausführbaren Datei des Managers und bei Bedarf der Schalter `--close-after-30` gespeichert. Das ist ein Start **bei der Anmeldung des aktuellen Benutzers**, kein Systemdienst. Für jedes hinzugefügte Programm werden keine separaten Einträge erstellt; den vorhandenen Autostart anderer Anwendungen verwaltet der Manager nicht.

**Deaktivieren Sie „Autostart“, bevor Sie die ausführbare Datei verschieben oder löschen.** Öffnen Sie den Manager nach dem Verschieben vom neuen Speicherort und aktivieren Sie ihn erneut. Für ein vollständiges Zurücksetzen deaktivieren Sie zuerst den Autostart, beenden die Anwendung über Quit und löschen anschließend das Verzeichnis `%APPDATA%\tauri-launcher` — damit werden die gespeicherte Liste und die Einstellungen entfernt.

## Häufige Fragen

**Ein Programm wurde hinzugefügt, startet aber nicht.**

Prüfen Sie den Schalter „On“ auf seiner Karte und ob die Datei unter dem angegebenen Pfad existiert. Starten Sie den Manager vollständig neu: Das Ändern der Liste startet für sich genommen nichts. Für den Start bei der Windows-Anmeldung muss außerdem „Autostart“ in den Einstellungen des Managers aktiviert sein.

**Anwendungen werden doppelt geöffnet.**

AutoStarter prüft nicht, ob ein Programm bereits läuft. Prüfen Sie dessen eigene Autostart-Einstellung und die Windows-Autostartliste. Ein Neustart des Managers selbst startet ebenfalls alle aktivierten Einträge erneut.

**Nach dem Klick auf das Kreuz läuft der Manager weiter.**

Das ist das erwartete Verhalten. Suchen Sie das Symbol AutoStart Manager im Infobereich, blenden Sie bei Bedarf die ausgeblendeten Symbole ein und wählen Sie Quit.

**Die Anwendung erscheint nach der Windows-Anmeldung nicht.**

Prüfen Sie „Im Infobereich starten“ und „Nach Ausführung schließen“: Im ersten Fall ist das Fenster ausgeblendet, im zweiten wird überhaupt keine Oberfläche erzeugt. Starten Sie zum Ändern der Einstellungen die ausführbare Datei manuell ohne den Schalter `--close-after-30`.

**Der Build findet den Linker oder das Windows SDK nicht.**

Prüfen Sie die Installation der C++ Build Tools und der MSVC-Toolchain von Rust und starten Sie dann das Terminal neu. Bei Problemen mit der Fensteranzeige prüfen Sie, ob die WebView2 Runtime vorhanden ist.

**Der Entwicklungsmodus meldet, dass der Port belegt ist.**

Geben Sie Port 5174 frei, zum Beispiel indem Sie die vorherige Vite-Instanz beenden. Der Port wird sowohl in der Vite- als auch in der Tauri-Konfiguration verwendet.

## Projektstruktur

```text
AutoStarter/
├── index.html                 # Markup von Hauptfenster und Einstellungen
├── src/
│   ├── main.ts                # Oberfläche, Übersetzungen und Tauri-Aufrufe
│   └── styles.css             # Stile der Anwendung
├── src-tauri/
│   ├── src/main.rs            # Programmstart, JSON, Registrierung und Win32-Infobereich
│   ├── Cargo.toml             # Rust-Abhängigkeiten und -Parameter
│   ├── Cargo.lock             # Festgeschriebene Rust-Abhängigkeiten
│   ├── build.rs               # Tauri-Buildskript
│   └── tauri.conf.json        # Fenster, Berechtigungen und Paketierung
├── icon.ico                   # Symbol der Anwendung und des Infobereichs
├── package.json               # npm-Befehle und Abhängigkeiten der Oberfläche
├── package-lock.json          # Festgeschriebene npm-Abhängigkeiten
├── vite.config.ts             # Vite-Einstellungen
├── build.bat                  # Release-Build ohne Installationsprogramm
└── dev.bat                    # Start des Entwicklungsmodus
```

## Mitwirken

Fehlerberichte und Vorschläge können in den [Issues](https://github.com/BORGERone/AutoStarter/issues) hinterlassen werden. Geben Sie für einen reproduzierbaren Bericht Ihre Windows-Version, die Art des Starts oder Builds, die Schritte und das erwartete Verhalten an. Entfernen Sie vor dem Veröffentlichen von Protokollen oder JSON-Dateien persönliche Daten und private Pfade.

Führen Sie bei Änderungen an der Oberfläche `npm run build` aus; Änderungen an Start, Registrierung und Infobereich sollten zusätzlich in der Desktop-Version unter Windows geprüft werden. Ein Pull Request sollte möglichst eine Beschreibung der Änderungen und der durchgeführten Prüfungen enthalten.

## Lizenz

In `src-tauri/Cargo.toml` ist die Lizenz **MIT** angegeben. Eine separate Datei `LICENSE` mit dem vollständigen Lizenztext ist im Repository bislang nicht vorhanden.
