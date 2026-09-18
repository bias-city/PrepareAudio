# PrepareAudio im Mac App Store — Anleitung fürs Apple-Portal

Texte: `appstore/texte/<sprache>/` (erzeugt von `scripts/appstore-texte.py`). Bilder:
`appstore/upload/<sprache>/` (erzeugt von `scripts/screenshots.mjs appstore/upload --store`,
2880 × 1800, JPEG ohne Alpha). Beides lässt sich jederzeit neu erzeugen.

## A. developer.apple.com › Certificates, Identifiers & Profiles

1. **Identifiers › +** › App IDs › App. Description `PrepareAudio`, Bundle ID **Explicit**
   `city.bias.prepareaudio`. Keine Capabilities ankreuzen. Register.
2. **Profiles › +** › Distribution › **Mac App Store Connect**. App ID `city.bias.prepareaudio`,
   Zertifikat «Apple Distribution» (das vorhandene), Name `PrepareAudio MAS`. Laden und ablegen als
   `src-tauri/profiles/PrepareAudio.provisionprofile` (gitignored).

Die Zertifikate «Apple Distribution» und «3rd Party Mac Developer Installer» liegen schon im
Schlüsselbund (von ResearchTranscript).

## B. App Store Connect › Apps › + › Neue App

Plattform macOS · Name `PrepareAudio` · Hauptsprache wie bei ResearchTranscript · Bundle-ID
`city.bias.prepareaudio` · SKU `prepareaudio-mac` · Zugriff uneingeschränkt.

## C. Versionsseite, je Sprache (Deutsch, Englisch (UK), Französisch, Italienisch)

| Feld | Datei |
|---|---|
| Werbetext | `werbetext.txt` |
| Beschreibung | `beschreibung.txt` |
| Schlagwörter | `schlagwoerter.txt` |
| Neuerungen | `neu.txt` |
| Untertitel (unter App-Informationen) | `untertitel.txt` |
| Bildschirmfotos | `appstore/upload/<sprache>/01…06` in dieser Reihenfolge |

Support-URL, Marketing-URL, Datenschutz-URL und Copyright: `appstore/texte/urls.txt`.

## D. Einmalige Angaben

- **Kategorien:** Dienstprogramme primär, Musik sekundär.
- **Altersfreigabe:** alles «Nein/Keine» ergibt 4+.
- **App-Datenschutz:** «Daten werden nicht erfasst», Datenschutz-URL eintragen, veröffentlichen.
- **Verschlüsselung:** keine (steht schon in der Info.plist). **DSA:** wie bei ResearchTranscript.
- **Preis:** kostenlos. **Verfügbarkeit:** alle Länder. **Lizenzvertrag:** Apples Standard.
- **Informationen zur App-Prüfung:** keine Anmeldung nötig; Notiz aus
  `appstore/texte/pruefnotiz-en.txt`. Vorher muss das Demo-Paket unter
  `https://bias.city/prepareaudio/demo/prepareaudio-demo.zip` liegen.

## E. Paket bauen und hochladen

```bash
npm run release:mas            # optional: node scripts/release-mas.mjs --build 2
```

Ergebnis `src-tauri/target/release/bundle/mas/PrepareAudio.pkg` in **Transporter** ziehen und
senden. Nach etwa 15 Minuten erscheint der Build unter TestFlight; dort testen, dann auf der
Versionsseite den Build wählen und «Zur Prüfung hinzufügen».

## Vor dem Einreichen auf bias.city (nur auf ausdrückliches Go, Live-Stand vorher sichern)

`site/privacy.html`, `site/index.html`, `quellen/lame-3.100.tar.gz` (aus `src-tauri/frameworks/`),
`demo/prepareaudio-demo.zip` (aus `appstore/demo/`). Kein Store-Emblem vor der Freigabe.
