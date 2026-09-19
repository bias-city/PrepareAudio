# Changelog

Alle nennenswerten Änderungen an PrepareAudio. Die GitHub-Release-Notizen einer Version
sind der entsprechende Abschnitt dieser Datei.

## 0.4.0 — noch nicht veröffentlicht

### Neu

- **MP3 oder WAV beim Mastern:** Neben dem Profil steht jetzt die Wahl des Ausgabeformats, sie
  bleibt gespeichert. **MP3** ist wie bisher 192 kbit/s. **WAV** schreibt 24 Bit in der
  Abtastrate der Quelle: nichts wird umgerechnet, nichts geht verloren — für Archiv und
  Weiterbearbeitung. Beide Formate erreichen dasselbe Ziel von −16 LUFS mit Spitzen bei
  −1,5 dBTP; beim WAV entfallen der Abschlag für den Codec und die Nachmessung, weil in der
  Datei genau das Signal steht, das gemessen wurde. Abtastraten, die MP3 nicht kennt, werden
  jetzt je Datei vermerkt und nur beim MP3 übergangen — als WAV gehen sie durch.
- Über eine Quelldatei schreibt das Mastern nie, auch wenn der gewählte Zielordner der
  Quellordner ist; in dem Fall entsteht ein Name mit `_2`.

## 0.3.0 — 2026-09-19

Aus dem Werkzeug für gestückelte Aufnahmen wird eine Kette für alles, was von einem
Gesprächstag übrig bleibt: hineingeben, synchronisieren, mastern. Erste Fassung, die auch für
den Mac App Store vorbereitet ist; derselbe Code erscheint hier als signiertes und
notarisiertes DMG. Pläne: `docs/PLAN-0.4-PIPELINE.md`, `docs/PLAN-APPSTORE.md`.

### Behoben

- Das Info-Feld beschrieb die ersten beiden Schritte noch in der alten Fassung: Es nennt jetzt
  alle Formate samt Video und die gemeinsame Datei mit einem Kanal je Sender.

- **Kein Ordner im Ordner mehr:** Heisst der gewählte Ordner schon `tracks`, `sync` oder
  `master` — gleich wie geschrieben —, schreibt die App direkt hinein statt einen weiteren
  Unterordner anzulegen.
- **Cloud-Platzhalter blockieren nicht mehr:** Dateien, die iCloud, Nextcloud oder Dropbox nur als
  Platzhalter zeigen, werden mit dem Hinweis «nicht lokal vorhanden» übersprungen. Vorher wartete
  die App beim Lesen auf den Download, und der Abbruch griff nicht.

### Geändert

- **Ablageflächen** in allen drei Schritten in derselben Form: Schrittnummer, ein Satz, der Button,
  das Kleingedruckte. Die gezeichneten Wellenformen als Illustration sind entfallen.
- Der Button unten links führt jetzt zum **Handbuch**; das Info-Feld mit den Lizenzen öffnet der
  Menüeintrag **About PrepareAudio**.

- Der Übersichtsbalken über der Timeline ist entfallen: seit die Leerzeiten zwischen Sessions
  herausfallen, zeigt die Timeline den Tag ohnehin am Stück.

### Neu

- **Mehrere Ordner auf einmal wählen**: Der Dialog **Ordner wählen…** nimmt jetzt eine
  Mehrfachauswahl, in allen drei Schritten. Hineinziehen ging das schon immer.

- **Handbuch in der App** (Menü Help, ⌘⇧/): durchsuchbar, mit Bildern aus der echten Oberfläche
  in hell und dunkel, in allen vier Sprachen. Erklärt die drei Schritte, die Timeline, die Profile
  beim Mastern, alle Tastenkürzel und was die App lokal speichert.
- **Menü auf das Nötige gekürzt** (kein File, kein View) mit eigenem Über-Eintrag, der das
  Info-Feld der App öffnet statt des kargen macOS-Panels. Im Fenster stehen Logo und Name nicht
  mehr: der Fenstertitel nennt die App.

- **Mastern mit zwei Profilen**, benannt nach ihrem Zweck: «Für Transkription» (Standard)
  arbeitet wie bisher mit fester Verstärkung und lässt die Dynamik unangetastet — an echten
  Interviews geprüft, Spracherkennung und Sprechertrennung arbeiten damit besser. «Fürs Hören»
  gleicht zusätzlich die Lautstärken der Sprechenden aus und senkt Übersprechen. Mehrkanalige
  Dateien werden in beiden Fällen nach den Positionen L/M/R aus Schritt 2 auf Stereo gemischt.
  Umschaltbar unten im Tab, die Wahl bleibt gespeichert.
- **Beliebig viele Mikrofone:** gemeinsame Abschnitte werden eine polyphone WAV mit einem Kanal
  je Sender, Timecode (bext) und Spurnamen (iXML). Jedes Segment hat eine Position L, M oder R
  für den Stereo-Mixdown beim Mastern; die Timeline zeigt eine Bahn je Sender.

- **Zusammenfügen nimmt alles:** neben WAV auch MP3, M4A, FLAC, ALAC, AIFF, CAF, OGG Vorbis und
  der Ton aus MP4- und MOV-Videos. Nicht-WAV-Dateien werden einmal dekodiert, als Teile geführt
  und bei passender Zeit und Naht verkettet; die Ausgabe ist immer WAV (Plan:
  `docs/PLAN-0.4-PIPELINE.md`).
- **Synchronisieren ab 60 Sekunden:** kurze Überlappungen werden in 20-s-Fenstern gemessen.
- **Timeline ohne Leerzeiten:** Zeit, in der kein Sender aufnahm, entfällt; eine Sessiongrenze
  nennt die entfernte Dauer, die Wiedergabe springt zur nächsten Session.
- **Layout nach Fenstergrösse:** Segment-Steuerelement, Symbol-Buttons, Statuszeile statt Kacheln,
  Listen als Zeilen über die ganze Breite.

- **Oberfläche an ResearchTranscript angeglichen:** Buttons und Tabs in Pillenform mit
  Haarlinie, flache Kopfzeile, «Info» als Button unten links, Grautöne der Slate-Skala,
  Zeiten in Festbreitenschrift. Die Werte stehen in `ui/tokens.css`. Aubergine und die
  Timeline bleiben, wie sie waren.
- **Store-Kanal:** Cargo-Feature `mas`, eigener Lizenzabsatz im Info-Feld, Links auf die
  Zusatzerlaubnis (`LICENSE-EXCEPTION`, AGPL §7) und die Datenschutzerklärung.
- **Erfundenes Demo-Material** (`scripts/demo-material.py`): zwei Sender, zwölf Minuten,
  je drei Aufnahme-Teile, gemeinsame und getrennte Gespräche. Dazu ein nachgebautes
  Backend und `scripts/screenshots.mjs` für Bildschirmfotos der echten Oberfläche.

### Geändert

- **LAME 3.100 ist dynamisch gelinkt** (`Contents/Frameworks/libmp3lame.dylib`, gebaut von
  `scripts/baue-lame.sh`, Quell-Tarball im App-Paket). Die Anbindung in
  `src-tauri/src/lame.rs` ist eigener Code; die Pakete `mp3lame-encoder` und `mp3lame-sys`
  (beide LGPL-3.0) sind entfallen. Die MP3-Dateien sind bitgleich zu 0.2.0, ein Test
  vergleicht die Bytes.
- **Finder und Links über das Opener-Plugin** statt über das Programm `open` als
  Kindprozess; das funktioniert auch in der App-Sandbox.
- **Bearbeitungsdatei beim Synchronisieren:** liegt weiter neben den Quellen. Ist der
  Ordner nicht beschreibbar (einzeln hineingezogene Dateien in der Sandbox), weicht die App
  nach `~/Library/Application Support/city.bias.prepareaudio/edits` aus.
- Beschreibungen in `tauri.conf.json` und `Cargo.toml` ohne Gerätemarke. Quell-Links zeigen
  auf `github.com/bias-city/PrepareAudio`.

## 0.2.0 — 2026-09-14

B/IAS, vier Sprachen, Teile jedes Recorders, Timeline (Plan: `docs/PLAN-0.2.md`).

## 0.1.0 — 2026-09-13

Erste Fassung: Aufnahme-Teile zusammenfügen, synchronisieren, mastern.
