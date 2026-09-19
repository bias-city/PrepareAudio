# PrepareAudio 0.3.0 im Mac App Store — Anleitung fürs Apple-Portal

Alles Nötige liegt bereit. Texte: `appstore/texte/<sprache>/` (erzeugt von
`scripts/appstore-texte.py`, Längen werden geprüft). Bilder: `appstore/upload/<sprache>/`
(erzeugt von `node scripts/screenshots.mjs appstore/upload --store`, 2880 × 1800, JPEG ohne
Alpha, in der Reihenfolge nummeriert). App-Symbol: `appstore/app-icon-1024.png`. Beides lässt
sich jederzeit neu erzeugen.

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

## C. Versionsseite 0.3.0, je Sprache (Deutsch, Englisch (UK), Französisch, Italienisch)

| Feld | Datei |
|---|---|
| Werbetext | `werbetext.txt` |
| Beschreibung | `beschreibung.txt` |
| Schlagwörter | `schlagwoerter.txt` |
| Neuerungen | `neu.txt` |
| Untertitel (unter App-Informationen) | `untertitel.txt` |
| Bildschirmfotos | `appstore/upload/<sprache>/01…07` in dieser Reihenfolge |

Die sieben Motive: Synchronisieren, Zusammenfügen, Mastern, Synchronisieren dunkel, Handbuch,
Leerzustand, Info-Feld mit Lizenzen.

Support-URL, Marketing-URL, Datenschutz-URL und Copyright: `appstore/texte/urls.txt`.

## D. Einmalige Angaben

- **Kategorien:** Dienstprogramme primär, Musik sekundär.
- **Altersfreigabe:** alles «Nein/Keine» ergibt 4+.
- **App-Datenschutz:** «Daten werden nicht erfasst», Datenschutz-URL eintragen, veröffentlichen.
- **Verschlüsselung:** keine (steht schon in der Info.plist). **DSA:** wie bei ResearchTranscript.
- **Preis:** kostenlos. **Verfügbarkeit:** alle Länder. **Lizenzvertrag:** Apples Standard.
- **Informationen zur App-Prüfung:** keine Anmeldung nötig; Notiz aus
  `appstore/texte/pruefnotiz-en.txt` einsetzen. Sie verweist auf das Demo-Paket unter
  `https://bias.city/prepareaudio/demo/prepareaudio-demo.zip` (liegt dort, 90 MB, erfundene Stimmen).

## E. Paket bauen und hochladen

```bash
npm run release:mas            # optional: node scripts/release-mas.mjs --build 2
```

Das Skript baut mit dem Feature `mas`, versiegelt mit «Apple Distribution» und den
Store-Entitlements, prüft (Sandbox auf jedem Programm, keine Rechnerpfade, LAME über `@rpath`)
und schreibt `src-tauri/target/release/bundle/mas/PrepareAudio.pkg`. Dieses Paket in
**Transporter** ziehen und senden. Nach etwa 15 Minuten erscheint der Build unter TestFlight;
dort testen, dann auf der Versionsseite den Build wählen und «Zur Prüfung hinzufügen».

Vorher einmal ohne Profil prüfen: `npm run probe:sandbox` baut dieselbe App mit Sandbox-Rechten,
signiert mit der Developer ID, und lässt sich lokal starten.

## Was schon erledigt ist

- Datenschutzseite: <https://bias.city/prepareaudio/privacy.html> (viersprachig, Du-Form)
- Demo-Paket für die Prüfer: <https://bias.city/prepareaudio/demo/prepareaudio-demo.zip>
- LAME-Quellen (LGPL): <https://bias.city/prepareaudio/quellen/lame-3.100.tar.gz>
- Zusatzerlaubnis AGPL §7: `LICENSE-EXCEPTION`, im App-Paket und auf GitHub
- Website und GitHub-Release stehen auf 0.3.0
