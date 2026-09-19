# Changelog

Alle nennenswerten Änderungen an PrepareAudio. Die GitHub-Release-Notizen einer Version
sind der entsprechende Abschnitt dieser Datei.

## Unveröffentlicht (Ziel 0.3.0)

Erste Fassung für den Mac App Store; derselbe Code erscheint als signiertes und
notarisiertes DMG. Plan: `docs/PLAN-APPSTORE.md`.

### Neu

- **Zusammenfügen nimmt alles:** neben WAV auch MP3, M4A, FLAC, ALAC, AIFF, CAF, OGG Vorbis und
  der Ton aus MP4- und MOV-Videos. Nicht-WAV-Dateien werden einmal dekodiert, als Teile geführt
  und bei passender Zeit und Naht verkettet; die Ausgabe ist immer WAV (Plan:
  `docs/PLAN-0.4-PIPELINE.md`).
- **Synchronisieren ab 60 Sekunden:** kurze Überlappungen werden in 20-s-Fenstern gemessen.
- **Timeline ohne Leerzeiten:** Zeit, in der kein Sender aufnahm, entfällt; eine Sessiongrenze
  nennt die entfernte Dauer, die Wiedergabe springt zur nächsten Session.
- **Layout nach Fenstergrösse:** Segment-Steuerelement, Symbolknöpfe, Statuszeile statt Kacheln,
  Listen als Zeilen über die ganze Breite.

- **Oberfläche an ResearchTranscript angeglichen:** Knöpfe und Reiter als Pillen mit
  Haarlinie, flache Kopfzeile, «Info» als Pille unten links, Grautöne der Slate-Skala,
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
