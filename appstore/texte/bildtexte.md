# Store-Bilder: Haltung, Gestaltung, Aufnahme

Stand 11.10.2026. Gezogen aus den Website-Texten (`site/index.html`) und der
Store-Beschreibung. Zielgruppe: Feldforschung, Interviews, Podcast.

> **Die Beschriftungen stehen in `bildtexte.json`, nicht hier.** Dort liest sie das
> Montageskript, in allen vier Sprachen. Hier steht nur, warum sie so aussehen und
> was auf den Aufnahmen zu sehen sein soll. Zwei Dateien mit denselben Texten laufen
> sonst auseinander.

## Haltung

Die App ist Werkzeug, nicht Unterhaltung. Die Bilder sollen wie eine Methodenseite
wirken, nicht wie eine Anzeige: ruhiger Grund, eine Aussage je Bild, das Fenster
gross und **ganz** sichtbar. Keine Pfeile, keine Sticker, keine Gerätemontagen.
Keine Markennamen, auch nicht in den Beschriftungen.

Vier Botschaften tragen die Reihenfolge:

1. **Mehrere Quellen werden eine Zeitachse.** Das ist der Kern.
2. **Zerteilte Aufnahmen werden wieder ganz.** Verlustfrei, erkannt an der Zeit.
3. **Bearbeiten verschiebt nichts.** Die gemessene Lage bleibt, die Sender bleiben synchron.
4. **Es bleibt auf dem Rechner.** Kein Konto, keine Cloud, offener Quellcode.

## Gestaltung (2880 × 1800, 16:10)

| Element | Wert |
|---|---|
| Grund, alle Motive | `#f3eaf6` (Website `--aub-050`) |
| Aussenrand | 100 px |
| Akzentlinie über der Schlagzeile | 110 × 4 px, `#56107a` (Website `--aub-600`) |
| Schlagzeile | Recursive, 76 px, Gewicht 640, Zeilenabstand 1.1 |
| Unterzeile | Recursive, 38 px, Gewicht 400, Schwarz bei 70 % Deckung |
| Kopfhöhe | 300 px, danach 44 px Luft |
| Fenster | ganz sichtbar, mittig in der Restfläche, Radius 18 px, weicher Schatten in Aubergine |

Schrift ist **Recursive** aus `site/fonts/Recursive_VF.woff2` (SIL OFL 1.1, Lizenztext
in `OFL.txt`), dieselbe wie auf der Website, mit `CASL 0` und `MONO 0`.

Zwei Entscheide:

- **Das Fenster wird nicht angeschnitten.** In der Timeline sitzt die Legende unten,
  in den Listen die Fusszeile mit Dateizahl und Knopf — genau das wäre sonst weg.
  Deshalb ist der Kopf knapp gehalten und das Bild wird eingepasst.
- **Der Grund ist der Markenton, nicht das neutrale Grau der Website.** `--wash` liegt
  zu nah an der Fensterleiste von macOS, das Fenster verschwände im Hintergrund.

## Motive, in dieser Reihenfolge

Fünf Bilder, die zusammen den Ablauf erzählen. Hell und dunkel wechseln, damit der
Satz im Store nicht monoton wirkt und beide Erscheinungsbilder belegt sind.

| # | Grund | Rohaufnahme | Was zu sehen ist |
|---|---|---|---|
| 1 | hell | `sync` | Timeline mit zwei Sendern, gemeinsamen und einzelnen Abschnitten, darunter Track-Paare mit Versatz, Drift und Trefferanteil |
| 2 | hell | `merge` | gefundene Aufnahmen mit ihren Teilen, aufgeklappt |
| 3 | dunkel | `edit-dunkel` | Timeline im dunklen Erscheinungsbild, Menü an einem Abschnitt offen: Ziel und Position beim Mastern |
| 4 | hell | `master` | Dateien mit Format, gemessener Lautheit und Zielwert |
| 5 | hell | `info` | Info-Feld mit Version, Herausgeber, Links und Lizenzen |

## Aufnahme und Montage

```bash
export PLAYWRIGHT_FROM=/Users/benpohl/Claude/enrich-transcript/frontend/package.json
node scripts/screenshots.mjs appstore/roh --breite 1440 --hoehe 720 --faktor 2
node scripts/screenshots.mjs appstore/roh --breite 1440 --hoehe 720 --faktor 2 --dunkel --motive sync,edit
node scripts/appstore-montage.mjs
```

Die Rohaufnahmen sind 2880 × 1440 (2:1), damit das Fenster die Restfläche unter dem
Kopf füllt, ohne angeschnitten zu werden. Sie liegen in `appstore/roh/`, die fertigen
Bilder in `appstore/store/<sprache>/`. Hochgeladen wird `appstore/store/`.

`appstore/upload/` ist der alte Satz ohne Beschriftung (sieben Motive, JPEG) und
bleibt als Rückfall liegen.
