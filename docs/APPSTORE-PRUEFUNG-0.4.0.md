# Prüfung gegen die App-Store-Richtlinien — PrepareAudio 0.4.0

Stand 11.10.2026. Grundlage: <https://developer.apple.com/app-store/review/guidelines/de/>,
der Plan in `PLAN-APPSTORE.md` und die Erfahrung aus ResearchTranscript 0.6.1
(`~/Claude/enrich-transcript-spike/docs/appstore-einreichung.md`). Geprüft am Code
von `50c15f0`, an der gebauten Sandbox-Probe und am Store-Material in `appstore/`.

## Ergebnis

Technisch und inhaltlich ist alles beisammen. Offen sind nur zwei Schritte im
Apple-Portal, die ein Mensch mit Apple-ID macht: das Mac-App-Store-Profil und der
App-Eintrag. Danach baut und lädt `npm run release:mas` und `npm run upload:mas`.

## Richtlinien, Punkt für Punkt

| Richtlinie | Anforderung | Stand in 0.4.0 | Beleg |
|---|---|---|---|
| 2.1 | Vollständige Fassung, funktionierende URLs, Material für die Prüfung | Demo-Paket mit erfundenen Stimmen, Prüfnotiz mit Vier-Minuten-Ablauf | `appstore/texte/pruefnotiz-en.txt`; Demo, Datenschutzseite und LAME-Quelle antworten mit HTTP 200 |
| 2.3 / 2.3.3 | Metadaten und Bilder zeigen die App im Gebrauch | Sieben Bildschirmfotos je Sprache aus der laufenden App, 2880 × 1800, kein Alphakanal | `appstore/upload/<sprache>/` |
| 2.3.7 | Keine fremden Produktnamen in Schlagwörtern | Keine Marke in Schlagwörtern und Untertiteln; ein Test verbietet «DJI» in allen sichtbaren Texten | `appstore/texte/*/schlagwoerter.txt`; `src-tauri/src/i18n.rs` Test |
| 2.4.5(i) | Sandbox | `com.apple.security.app-sandbox` auf dem einzigen Programm im Paket; Container entsteht beim Start | Probe-Build, `~/Library/Containers/city.bias.prepareaudio` |
| 2.4.5(ii) | Verpackt mit Apple-Werkzeugen, kein fremdes Installationsprogramm | `productbuild` mit «3rd Party Mac Developer Installer» | `scripts/release-mas.mjs` |
| 2.4.5(iii) | Kein Autostart, kein Code bei der Anmeldung | Nichts dergleichen; die App startet nur auf Zuruf | Code, keine LaunchAgents |
| 2.4.5(iv) | Keine Hilfsprogramme nachladen oder installieren | Ein Prozess, reines Rust, keine Hilfsprogramme, kein Interpreter | `otool`-Prüfung im Release-Skript |
| 2.4.5(v) | Keine Root-Rechte, kein setuid | Nicht vorhanden | Code |
| 2.4.5(vii) | Updates nur über den Store | Kein Updater im Paket; die Store-Fassung zeigt keinen Download-Link | `Cargo.toml`, `ui/info.js` (Kanal `mas`) |
| 2.5.1 | Nur öffentliche Schnittstellen | Tauri, WebKit, CoreAudio über cpal, sonst eigene Rechnung | `Cargo.toml` |
| 2.5.2 | Kein Code nachladen, nichts außerhalb des eigenen Bereichs schreiben | Nichts wird nachgeladen. Das mitgelieferte LAME-Quellarchiv ist Material, kein ausführbarer Code. Geschrieben wird nur in vom Benutzer gewählte Ordner und in den Container | Entitlements, Phase-2-Prüfungen |
| 4.2 | Eigenständige Funktion, keine aufbereitete Website | Drei Arbeitsschritte mit Zeitachse, Wiedergabe und Messung | App |
| 5.1.1(i) | Datenschutz-Link in der App und im Portal | Info-Feld verlinkt die Datenschutzerklärung; URL in `appstore/texte/urls.txt` | `ui/info.js`, <https://bias.city/prepareaudio/privacy.html> |
| 5.1.1(ii) | Datenschutzerklärung nennt die Daten | Die App erfasst nichts; die Seite sagt das in vier Sprachen | `site/privacy.html` |
| 5.2 | Kein fremdes geistiges Eigentum | Gerätehersteller werden nirgends genannt; nur interne Feldnamen und Testdateien heißen noch so | `grep -ri dji ui src-tauri/src` |
| Abschnitt 5 allgemein | Rechtliche Lage | AGPL-3.0-or-later mit Zusatzerlaubnis nach Abschnitt 7 für den Store; LAME (LGPL) dynamisch gebunden, Quelle im Paket und online | `LICENSE-EXCEPTION`, `scripts/baue-lame.sh` |

## Was heute nachgewiesen wurde

- `cargo test --release`: 58 Tests bestanden, keine Fehlschläge.
- `npm run probe:sandbox`: baut mit Feature `mas`, versiegelt, besteht alle vier
  Prüfungen des Skripts (Sandbox auf jedem Programm, keine Rechnerpfade, LAME über
  `@rpath`), startet und legt seinen Container an.
- Entitlements der Probe: `app-sandbox`, `files.user-selected.read-write`,
  `network.client`. Kein Mikrofon, keine Bookmarks.
- Store-Material: je Sprache sieben Bilder 2880 × 1800 ohne Alphakanal, Symbol
  1024 × 1024 ohne Alphakanal, alle Texte innerhalb der Längengrenzen.
- Website: Startseite, Datenschutzerklärung, Demo-Paket und LAME-Quelle erreichbar.

## Unterschied zu ResearchTranscript

Dort kam die Ablehnung nach 2.5.2, weil die mitgelieferte Python-Laufzeit noch pip
und Build-Werkzeuge trug. PrepareAudio hat keinen Interpreter und keine
Hilfsprogramme, also entfällt dieser Punkt. Geblieben ist von dort die Begründung
für `network.client` in der Prüfnotiz; sie gilt hier genauso.

## Offen, nur im Apple-Portal zu erledigen

1. **App-ID** `city.bias.prepareaudio` (explizit) unter Identifiers anlegen.
2. **Profil** «Mac App Store Connect» mit dem Zertifikat «Apple Distribution»
   erzeugen und als `src-tauri/profiles/PrepareAudio.provisionprofile` ablegen.
3. **App-Eintrag** in App Store Connect anlegen (macOS, Bundle-ID, SKU
   `prepareaudio-mac`), Texte und Bilder aus `appstore/` eintragen.
4. Für den Upload ohne Transporter: **API-Schlüssel** unter Users and Access ›
   Integrations erzeugen, `.p8` nach `~/.appstoreconnect/private_keys/`,
   `APPLE_API_KEY_ID` und `APPLE_API_ISSUER` setzen.

Danach: `npm run release:mas`, prüfen mit
`node scripts/release-mas.mjs --ab-signatur --pruefen`, senden mit
`npm run upload:mas`. Erster Build zuerst über TestFlight.
