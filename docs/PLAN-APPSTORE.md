# Plan: PrepareAudio in den Mac App Store

Stand 2026-09-18. Grundlage: Code-Prüfung von 0.2.0 (`9f124ca`) und die
Erfahrungen aus ResearchTranscript 0.6.0 (eingereicht am 18.9.2026,
`~/Claude/enrich-transcript-spike/docs/appstore-plan.md`). Zielversion: **0.3.0**,
derselbe Code als Store-Paket und als notarisiertes DMG.

## Stand 2026-09-18 abends (Zweig `appstore`, nicht gepusht)

- **Phase 0 umgesetzt**, Abnahme durch den User offen. Vorschaubilder: `docs/entwurf-phase0/`.
- **Phase 1 umgesetzt:** Opener-Plugin, Ausweichort der Bearbeitungsdatei (mit Test), LAME
  3.100 dynamisch mit eigener Anbindung `src/lame.rs` (MP3 bitgleich, Test), Feature `mas`,
  `LICENSE-EXCEPTION`, Entitlements, `scripts/release-mas.mjs` mit `--probe`. Neu gefunden:
  auch `mp3lame-encoder` und `mp3lame-sys` standen unter LGPL-3.0 und sind entfallen.
- **Phase 2 begonnen:** Die Sandbox-Probe (`npm run probe:sandbox`) baut, besteht alle
  Prüfungen, startet und legt ihren Container an. Offen ist der Handtest am Fenster
  (Liste in Phase 2), darunter der Start ohne `network.client`.
- **Phase 3 begonnen:** `site/privacy.html` (vier Sprachen, nicht hochgeladen),
  Demo-Material, Demo-Backend und Bildschirmfoto-Skript liegen im Repo. Das Demo-Material
  verkettet über exakte TimeReference, zeigt aber den Hinweis «kürzer als ein voller
  Teil»; für Store-Bilder entweder hinnehmen oder Teile über 100 MiB erzeugen.
- Im Schlüsselbund liegen zwei «Developer ID Application»; die Probe nimmt die aus
  `tauri.conf.json`. Die andere liess `codesign` ohne Rückmeldung warten.

## Befund in Kürze

PrepareAudio ist für den Store ein leichter Fall: reines Rust in einem Prozess,
keine Hilfsprogramme, kein Python, kein Netz, keine Modelle, 8 MB. Es merkt sich
keine Pfade über den Neustart (nur die Sprache im `localStorage`), also braucht es
keine Security-Scoped Bookmarks. Fünf Stellen brechen in der Sandbox oder sind
für die Prüfung heikel:

| # | Stelle | Problem | Lösung |
|---|---|---|---|
| 1 | `lib.rs` `reveal`, `open_link` | starten `/usr/bin/open` als Kindprozess; in der Sandbox unzuverlässig, bei RT ausdrücklich verworfen | `tauri-plugin-opener` (`reveal_item_in_dir`, `open_path`, `open_url`), läuft über NSWorkspace |
| 2 | `sync.rs:2304` Bearbeitungsdatei | `.prepareaudio-sync.json` liegt in `roots[0]`. Bei einzeln hineingezogenen **Dateien** ist das der Elternordner, und der ist in der Sandbox nicht beschreibbar | Schreiben versuchen, bei Fehler nach `Application Support/edits/<hash>.json` ausweichen; beim Lesen beide Orte prüfen; Hinweistext «wird neben den Quellen gespeichert» je Fall |
| 3 | LAME 3.100 statisch (`mp3lame-sys`) | LGPL im Store: bei RT bewusst dynamisch gelinkt, Quelle gehostet. Statisch wäre hier die Abweichung | `libmp3lame.dylib` **3.100** nach `Contents/Frameworks`, Tarball auf bias.city/prepareaudio/quellen/, eigener kleiner `-sys`-Ersatz über `[patch.crates-io]`. 3.100 behalten, weil Limiter und Nachmessung darauf kalibriert sind |
| 4 | WebKit in der Sandbox | RT-Befund F: ohne `network.client` rendert WebKit die eigenen Seiten nicht | Entitlement setzen, Begründung in die Prüfnotiz (Text von RT übernehmen); vorher einmal ohne testen |
| 5 | Markenname | `tauri.conf.json` `shortDescription` und `Cargo.toml` `description` nennen «DJI Mic 2» (Richtlinie 5.2.1) | neutral formulieren wie README und Website; der Test in `i18n.rs` wacht schon über die UI-Texte |

Unkritisch, aber zu belegen (Phase 2): Ordner hineinziehen gibt rekursiven
Zugriff; Ausgabe nur in per Dialog gewählte Ordner; Cache über `HOME` landet im
Container; `cpal` spielt ohne Mikrofon-Berechtigung; keine Verschlüsselung.
Der Name «PrepareAudio» hat im Store keinen gleichnamigen Treffer (Suche 18.9.).

## Phase 0 — Oberfläche an ResearchTranscript angleichen (User 18.9.2026)

Zuerst, weil danach alle Bildschirmfotos (Website, Store) neu entstehen. Nur CSS
und wenig HTML in `ui/`, kein Framework-Wechsel. Massgeblich sind die Regeln aus
`enrich-transcript-spike/frontend/src/styles.css` («Pillenform», User 17.9.).

1. **Knöpfe als Pillen.** `border-radius: 9999px`, Höhe 24 px (klein) bzw. 32 px
   (Startknopf), Fläche = Panel, Rand als `inset 0 0 0 1.5px currentColor`,
   Schrift in Textfarbe, Hover graue Fläche. Gilt für `.btn`, `.btn.small`,
   `.icon-btn`, die Tag-Knöpfe und die Sprachwahl.
2. **Startknopf** (Zusammenfügen, Erzeugen, Mastern): gefüllte Pille in Aubergine,
   ohne Verlauf. Aubergine bleibt die Farbe von PrepareAudio, so wie Indigo die
   von RT ist.
3. **Reiter als Pillengruppe** wie die Modulwahl von RT oben rechts (aktiv = graue
   Fläche, Ziffern bleiben). «Neu analysieren» und «Anderer Ordner…» als Pillen
   daneben.
4. **Kopf und Fuss.** Kopfzeile flach und grau wie bei RT, 40 px, Logo und Name
   links; die Unterzeile wandert in den Leerzustand. Fusszeile mit Haarlinie.
   Der «i»-Knopf wird zur Pille **«Info» unten links** (wie «Hilfe» in RT).
5. **Raster und Masse.** Abstände auf 4/8/12/16/24 px, Schriftgrössen 12/14 px
   (Radix 1 und 2), Haarlinien statt weicher Kartenränder, Radius der Flächen
   8 px, Zeitangaben überall in Monospace mit Tabellenziffern. Grautöne auf die
   Slate-Skala von RT, hell und dunkel.
6. **Menüs:** markierter Eintrag dunkelgrau auf hell (Kontextmenü der Timeline).
7. Werte als `ui/tokens.css` mit Verweis auf die Quelle, damit beide Apps später
   gemeinsam nachgezogen werden können.

Nicht angleichen: die Timeline selbst (Clips, Wellenformen, Senderfarben) und die
Kennzahl-Kacheln, sie sind fachlich eigen. Aufwand: 1 bis 1,5 Tage, Abnahme durch
den User am laufenden Fenster.

## Phase 1 — Sandbox und Store-Kanal im Code

1. `entitlements.mas.plist`: `app-sandbox`, `files.user-selected.read-write`,
   `network.client` (Befund 4), `application-identifier`
   `CCRJ4A42D3.city.bias.prepareaudio`, `team-identifier`. Kein Mikrofon, keine
   Bookmarks, kein JIT.
2. Befunde 1, 2, 3 und 5 umsetzen. Für LAME `scripts/baue-lame.sh` aus RT
   übernehmen (Version 3.100, Prüfsumme, `--disable-decoder --disable-frontend`),
   Einbindung über `bundle.macOS.frameworks`, rpath in `build.rs`.
3. Cargo-Feature `mas` und Befehl `kanal` wie in RT. Im Info-Feld je Kanal: Store
   = ein Absatz mit Zusatzerlaubnis, DMG = AGPL-Text wie heute.
4. `LICENSE-EXCEPTION` (AGPL §7, Apple App Store) ins Repo, README und Info-Feld.
   Aller eigene Code gehört B/IAS; Fremdcode steht unter MIT, Apache, BSD, MPL
   und LGPL, nichts davon braucht die Ausnahme.
5. Quell-Link auf `github.com/bias-city/PrepareAudio` (heute noch `BenPohlBasel`
   in `ui/info.js`, README und `site/README.md`).
6. `Info.plist`: `LSApplicationCategoryType`, `ITSAppUsesNonExemptEncryption =
   false`. Mindestsystem 12.0 bleibt (für reine Apple-Silicon-Apps verlangt der
   Store mindestens 12.0).
7. `tauri.mas.conf.json` und `scripts/release-mas.mjs` aus RT, stark gekürzt:
   Profil einbetten, mit «Apple Distribution» signieren, Wächter «jedes
   Mach-O-Programm trägt app-sandbox», `productbuild` mit «3rd Party Mac Developer
   Installer». `CFBundleVersion` hochzählbar.
8. Neu im Repo: `CHANGELOG.md`, `.gitattributes` (BACKLOG und Pläne nicht ins
   Quellarchiv), Abschnitt «Authorship and AI assistance» wie in RT.

Aufwand: 1,5 bis 2 Tage.

## Phase 2 — Prüfen im gebauten Sandbox-Bundle

Nie nur im Entwicklungsmodus. Testlauf mit den Aufnahmen vom 6. bis 8.9.2026:

- Ordner hineinziehen, einzelne Dateien hineinziehen, «Ordner wählen…», Ordner
  auf externem Laufwerk.
- Zusammenfügen über 4 GB (RF64), Abbrechen, «schon vorhanden».
- Synchronisieren: Analyse, Bearbeiten, Neustart, Bearbeitung wieder da (beide
  Speicherorte aus Befund 2), Vorschau-Wiedergabe, Erzeugen.
- Mastern aus WAV, M4A und MP3; Cache im Container; Aufräumen nach 30 Tagen.
- «Ordner öffnen», Links im Info-Feld, Sprachwechsel, Systemdialoge viersprachig.
- Einmal ohne `network.client` starten und das Ergebnis festhalten.
- `cargo test --lib`, dazu `real_data`, `real_sync`, `real_master`: MP3-Ergebnis
  mit dynamischer LAME bitgleich zur statischen.

Danach TestFlight beim User. Aufwand: 0,5 Tage plus Testzeit.

## Phase 3 — Store-Material

1. **Datenschutzseite** `bias.city/prepareaudio/privacy.html`, viersprachig, Du-Form
   (heute 404). Upload nur auf ausdrückliches Go, vorher Live-Stand sichern.
2. **Texte** in de, en-GB, fr, it, Du-Form: Name, Untertitel (30), Werbetext (170),
   Beschreibung, Schlagwörter (100), Neuerungen. Erzeugt durch ein Skript wie
   `appstore-anleitung.py`, mit Längenprüfung. Keine Gerätemarken.
3. **Bildschirmfotos** 2880 × 1800, sechs bis acht Motive × vier Sprachen. Das
   nachgebaute Backend der Website-Bilder liegt **nicht im Repo**: als
   `scripts/demo-backend.js` samt Aufnahmeskript einchecken, Daten erfunden oder
   neutralisiert (`/Users/demo/…`).
4. **Material für die Prüfer.** Ohne Aufnahme-Teile lässt sich die App nicht
   prüfen. Ein erfundenes Demo-Paket auf bias.city: zwei Spuren mit gemeinsamen
   Ereignissen (Synchronisieren, Mastern) und eine geteilte Aufnahme. Offen: Ein
   «voller Teil» braucht heute mindestens 100 MiB oder genau eine Stunde
   (`scan.rs:90`). Entweder ein stündiges Paket mit niedriger Abtastrate oder ein
   dokumentierter Prüfschalter. In Phase 3 entscheiden und messen.
5. **Prüfnotiz** auf Englisch: arbeitet offline, kein Konto, Grund für
   `network.client`, Demo-Link, Schritt-für-Schritt für die drei Reiter.
6. Website auf 0.3.0 und neue Bilder; **kein Store-Emblem vor der Freigabe**.

Aufwand: 1 Tag.

## Phase 4 — Apple-Portal (Schritte des Users, je mit Anleitung)

1. Identifier `city.bias.prepareaudio` (explizit, macOS) anlegen.
2. Profil «PrepareAudio MAS» mit dem vorhandenen Zertifikat «Apple Distribution»;
   Datei nach `src-tauri/profiles/` (gitignored).
3. App-Eintrag: Name PrepareAudio, SKU `prepareaudio-mac`, Hauptsprache wie RT.
4. Kategorien: Vorschlag Dienstprogramme primär, Musik sekundär. Altersfreigabe
   4+, Verschlüsselung keine, Datenschutz «keine Daten erfasst», DSA wie RT,
   kostenlos, alle Länder, Standard-Lizenzvertrag.
5. Upload über Transporter, TestFlight-Probe, Einreichen.

## Phase 5 — Release rundherum

DMG 0.3.0 notarisiert (Profil `localtranscript`), GitHub-Release, Website, und
eine **Zenodo-DOI in der Community `bias-city`**: `scripts/zenodo.py` und
`.zenodo.json` aus RT übernehmen, `CITATION.cff` mit ORCID. Vor jedem Push die
Commits auf Adressen, Passwörter und echte Aufnahmen prüfen. Aufwand: 0,5 Tage.

## Entscheide für den User

| # | Frage | Empfehlung |
|---|---|---|
| E1 | LAME dynamisch (wie RT) oder statisch lassen | dynamisch, Version 3.100 |
| E2 | Startknopf flach in Aubergine oder mit Verlauf wie heute | flach |
| E3 | Nur Apple Silicon oder Universal (auch Intel) | zunächst nur Apple Silicon; Universal später, wenn Nachfrage |
| E4 | Kategorien | Dienstprogramme, Musik |
| E5 | Einreichen sofort oder nach Apples erster Rückmeldung zu RT | Bauen parallel, einreichen nach der Rückmeldung: dieselben Fragen (Netz-Entitlement, AGPL) kämen sonst doppelt |

## Risiken

| Risiko | Einschätzung | Gegenmittel |
|---|---|---|
| Hineinziehen von Ordnern gibt in der Sandbox keinen rekursiven Zugriff | unwahrscheinlich, aber ungeprüft | Phase 2 zuerst; Rückfall: nur Ordnerdialog |
| Prüfer können die Funktion nicht nachvollziehen | mittel | Demo-Paket und genaue Prüfnotiz |
| Rückfrage zu `network.client` | wahrscheinlich | Begründung steht, RT liefert das Muster |
| Angleichung der Oberfläche zieht sich | mittel | Umfang auf Phase 0 begrenzt, Timeline ausgenommen |
| MP3 mit dynamischer LAME weicht ab | gering bei gleicher Version | Bitvergleich in Phase 2 |

Gesamtaufwand ohne Wartezeiten: etwa 5 bis 6 Arbeitstage.
