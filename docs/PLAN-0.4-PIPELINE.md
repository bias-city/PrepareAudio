# Plan 0.4: eine Pipeline für alles, was hereinkommt

Auftrag des Users vom 19.9.2026, wörtlich sinngemäss:

1. **Zusammenfügen.** Egal was hereinkommt (WAV mono oder stereo, OGG, M4A, MOV und anderes
   Video, MP3): den Ton herausziehen, Zusammenhängendes aneinanderhängen, soweit Teile erkannt
   werden, als WAV ausgeben.
2. **Synchronisieren.** Alles synchronisieren, was möglich ist, auch mehr als zwei parallele
   Mikrofone, ab 60 s Länge. Ausgabe als polyphone WAV; Mono, wenn nichts synchron ist.
3. **Mastern** nach neuer Ausrichtung (Sprach-Leveler, siehe unten) mit Stereo-Mixdown.

## Stand der Prüfung im Code

- Die Analyse vergleicht schon heute alle Sender paarweise (`analyze`, `cands`) und legt alle
  Spuren über `placements` auf eine gemeinsame Zeitachse, samt Drift. Das trägt für N Sender.
- Fest auf zwei Sender verdrahtet sind: `items_from_clips` (Seiten links/rechts, `lane < 2`),
  `plan_items` (eine Spurzeit gehört genau einem Paar), `write_stereo`, die Dateinamen
  (`_stereo_L-4_R-5`) und die Timeline in `ui/sync.js` (zwei Sender, je eine Mono- und eine
  Stereo-Bahn, «treffen sich in der Mitte»).
- Symphonia liest Ton aus MP4 und MOV (AAC, PCM 16/24 bit), geprüft mit dem Test
  `decode::probe::probe_files`. **Opus liest es nicht** (WhatsApp-Sprachnachrichten, WebM).
- Das Zusammenfügen nimmt heute nur WAV; Synchronisieren und Mastern dekodieren schon alles
  andere in den Cache (`decode.rs`).

## Arbeitspakete

### P0 — Synchronisieren ab 60 s ✔ (19.9.)
Überlappungen unter fünf Minuten werden in 20-s-Fenstern alle 10 s gemessen, die Drift bleibt
dort null. Tests: `short_recordings_from_sixty_seconds_are_synchronised`,
`unrelated_short_recordings_stay_apart`. Lange Aufnahmen unverändert.

### P1 — Zusammenfügen nimmt alles ✔ (19.9.)
- `AUDIO_EXT` um `mov`, `m4v` (Hinweis an Symphonia: `mp4`); später `mkv`/`webm`.
- `scan`: Nicht-WAV-Dateien einmal in den Cache dekodieren und als Teil führen (Name und Pfad
  des Originals, Audiodaten aus der Kopie). Startzeit: Name, dann Erstellungszeit im Container
  (`mvhd`), dann Dateidatum.
- Verkettung dekodierter Teile: gleiche Form, Anschluss ≤ 3 s aus Name oder Container (nicht
  aus dem Dateidatum), Naht bestätigt; sonst «niedrig» und nicht vorausgewählt. Kameras teilen
  bei 4 GB, Recorder-Apps selten.
- Ausgabe immer WAV (dekodierte Teile: 32-bit float). Fortschritt beim Dekodieren zeigen.
- Texte «Nur WAV» in vier Sprachen ersetzen.

### P2 — N Sender, polyphone WAV ✔ (19.9., Abnahme durch den User offen)

Entscheide des Users vom 19.9.: eine Bahn je Sender, auch bei zwei Sendern. Jedes SEGMENT hat
zwei Einstellungen: Ziel (gemeinsame Datei oder eigene Mono-Datei) und Position im Stereo-Mixdown
(L, M, R; Standard nach Reihenfolge der Sender: erster L, letzter R, sonst M). Die Positionen
stehen je Segment im iXML der gemeinsamen WAV (`<PREPAREAUDIO><PAN CH T0 T1 POS/>`), dazu
`FUNCTION` je Kanal; `wav::parse_pan_segments` liest sie für das Mastern. Mono-Dateien bleiben
beim Mastern in der Mitte.

- `items_from_clips`: eine gemeinsame Datei hat N Kanäle, einer je Sender in fester Reihenfolge
  (`label_order`); fehlt ein Sender, ist sein Kanal still. Zwei Sender ergeben wie bisher zwei
  Kanäle. `write_stereo` wird `write_poly`.
- WAV mit `WAVE_FORMAT_EXTENSIBLE` (Kanalmaske 0) ab drei Kanälen, 32-bit float, RF64 ab 4 GB.
  Dazu `bext` (Datum, Uhrzeit, TimeReference) und iXML mit den Spurnamen (`TRACK_LIST`).
  Name: `<stempel>_poly_<a>-<b>-<c>.wav`; bei zwei Sendern bleibt `_stereo_L-a_R-b`.
- Vorschlag der Analyse für N > 2: je Spur die Vereinigung aller «gemeinsam»-Phasen ihrer
  Paare; der Rest wird Mono, wenn er lang genug ist. Für zwei Sender bleibt `plan_items`.
- Timeline: je Sender EINE Bahn; gemeinsam = volle Farbe, Mono = hell. Das ersetzt die
  Anordnung «innen/aussen», die nur mit zwei Sendern aufgeht. **Entscheid des Users nötig.**
- Vorschau: alle Kanäle gleichmässig im Stereobild verteilt.

### P3 — Mastern als Sprach-Leveler mit Stereo-Mixdown ✔ (19.9., Hörtest durch den User offen)

Gebaut in `src-tauri/src/level.rs` (Voranalyse in 10-ms-Schritten, Gain-Kurven, Mixdown) und
angebunden in `master.rs` (`Profile`, ein zusätzlicher Lesedurchgang `W_PREP`). Der Leveler ist
Standard, «Dokumentarisch» bleibt bitgleich zu 0.2.0 (Test `MP3_GOLDEN`). VAD ist vorerst eine
Pegel-Schwelle mit Nachlauf, nicht `webrtc-vad`.

Je Kanal: DC weg, Hochpass 80 Hz → Sprache erkennen (zuerst Energie-Schwelle, später
`webrtc-vad`) → Gain-Riding auf Kurzzeitlautheit (400 ms bis 3 s, ±12 dB, in Pausen halten,
vorwärts und rückwärts geglättet) → Übersprechen: je Fenster die aktive Spur bestimmen,
inaktive sanft absenken → leichter Kompressor (höchstens 2:1) → Mixdown auf Stereo (Sender
gleichmässig von links nach rechts, Mono-Quellen mittig) → −16 LUFS → Limiter −1,5 dBTP → MP3,
nachmessen wie heute.
- Das heutige Mastern bleibt als Profil «Dokumentarisch» (feste Verstärkung, keine Eingriffe in
  die Dynamik) wählbar. **Entscheid des Users: welches Profil ist Standard?**
- Rauschunterdrückung (`nnnoiseless`) nur als ausdrückliche Option, nicht im ersten Wurf.
- Das gewählte Profil steht im ID3-Kommentar.

### Reihenfolge
P1 → P2 (Backend, dann Timeline) → P3. Jedes Paket mit Tests am erfundenen Material
(`scripts/demo-material.py`, neu: dritter Sender) und Bildschirmfotos in mehreren Grössen.

## Lücken und Risiken
- Opus braucht eine weitere Bibliothek (libopus, BSD) oder bleibt aussen vor.
- Falsche Verkettung bei Sprachmemos, die zufällig direkt aufeinander folgen: deshalb Naht-Test
  und «niedrig».
- N-Kanal-WAV lesen nicht alle Programme gleich gut; ResearchTranscript muss polyphone Dateien
  annehmen (dort prüfen).
- Der Store-Eintrag 0.3.0 kann vorher oder nachher eingereicht werden; die Texte und Bilder
  ändern sich mit P2 und P3 deutlich. Empfehlung: erst 0.4, dann einreichen.
