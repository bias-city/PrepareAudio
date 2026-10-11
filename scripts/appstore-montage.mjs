#!/usr/bin/env node
// Store-Bilder montieren: Rohaufnahme der App + Beschriftung auf ruhigem Grund.
//
// Richtlinie 2.3.3 erlaubt Text- und Bildauflagen ausdrücklich; verlangt ist, dass
// das Fenster die eingereichte Fassung in Benutzung zeigt. Die Rohaufnahmen kommen
// deshalb aus der echten Oberfläche (ui/) mit den Analysedaten echter Aufnahmen:
//   PLAYWRIGHT_FROM=… node scripts/screenshots.mjs appstore/roh --breite 1440 --hoehe 720 --faktor 2
//   … dasselbe noch einmal mit --dunkel (liefert sync-dunkel)
//
// Texte: appstore/texte/bildtexte.json (alle vier Sprachen, einzige Quelle).
// Gestaltung und Begründung: appstore/texte/bildtexte.md.
// Schrift: Recursive aus site/fonts (SIL OFL 1.1, Lizenztext in OFL.txt) — dieselbe
// wie auf der Website. Farben: der Aubergine-Ton der Marke.
//
// Aufruf: node scripts/appstore-montage.mjs [--sprachen de,en] [--quelle appstore/roh]
//                                           [--ziel appstore/store]
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const require = createRequire(process.env.PLAYWRIGHT_FROM || import.meta.url);
const { webkit } = require("playwright");

const arg = (name, std) => { const i = process.argv.indexOf(`--${name}`); return i > 0 ? process.argv[i + 1] : std; };
const SPRACHEN = arg("sprachen", "de,en,fr,it").split(",");
const QUELLE = path.join(ROOT, arg("quelle", "appstore/roh"));
const ZIEL = path.join(ROOT, arg("ziel", "appstore/store"));
const DATEN = JSON.parse(fs.readFileSync(path.join(ROOT, "appstore/texte/bildtexte.json"), "utf8"));

// Apple, Mac: 16:10. Wir liefern die grösste Fassung, Apple skaliert herunter.
const B = 2880, H = 1800;
const RAND = 100;
// Das Fenster wird GANZ gezeigt, nicht angeschnitten: in der Timeline sitzt die
// Legende unten, in den Listen die Fusszeile — genau die wären sonst weg. Also
// den Kopf knapp halten und das Bild in die Restfläche einpassen.
const TEXTHOEHE = 300;     // Linie + Schlagzeile (zwei Zeilen) + Unterzeile
const LUFT = 44;           // zwischen Text und Fenster

const TYPEN = { ".png": "image/png", ".woff2": "font/woff2", ".html": "text/html; charset=utf-8" };
const server = http.createServer((req, res) => {
  const rel = decodeURIComponent(req.url.split("?")[0]).replace(/^\/+/, "");
  const datei = path.join(ROOT, rel);
  if (!datei.startsWith(ROOT)) { res.writeHead(403).end(); return; }
  fs.readFile(datei, (err, buf) => {
    if (err) { res.writeHead(404).end(); return; }
    res.writeHead(200, { "content-type": TYPEN[path.extname(datei)] || "application/octet-stream" });
    res.end(buf);
  });
});
await new Promise((ok) => server.listen(0, "127.0.0.1", ok));
const BASIS = `http://127.0.0.1:${server.address().port}/`;

const esc = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
/** Zeilenumbrüche der Schlagzeile stehen in den Texten als \n. */
const zeilen = (s) => esc(s).split("\n").join("<br>");

// EIN Grund für alle Motive: der helle Aubergine-Ton der Website (--aub-050). Das
// neutrale Grau von dort (--wash) liegt zu nah an der Fensterleiste von macOS — das
// Fenster verschwände im Hintergrund. `dunkel` steuert deshalb nur den Schatten.
function seite(bild, titel, unter, dunkel) {
  const grund = "#f3eaf6";   // --aub-050
  const tinte = "#000000";
  const leise = "rgba(0,0,0,.70)";
  const schatten = dunkel ? "0 26px 70px rgba(26,22,30,.40)" : "0 24px 60px rgba(67,6,95,.20)";
  const rahmen = "rgba(67,6,95,.14)";
  return `<!doctype html><meta charset="utf-8"><style>
  @font-face { font-family:"Recursive"; src:url("${BASIS}site/fonts/Recursive_VF.woff2") format("woff2");
               font-weight:300 1000; font-display:block; }
  * { margin:0; padding:0; box-sizing:border-box; }
  html, body { width:${B}px; height:${H}px; overflow:hidden; }
  body { background:${grund}; color:${tinte};
         font-family:"Recursive", system-ui, sans-serif;
         font-variation-settings:'CASL' 0, 'MONO' 0;
         -webkit-font-smoothing:antialiased; }
  .blatt { padding:${RAND}px; height:${H}px; display:flex; flex-direction:column; }
  .kopf { height:${TEXTHOEHE}px; }
  /* Feine Linie in der Akzentfarbe: der einzige Schmuck. */
  .strich { width:110px; height:4px; background:#56107a; border-radius:2px; margin-bottom:26px; }
  h1 { font-size:76px; line-height:1.1; font-weight:640; letter-spacing:-.015em; }
  p { font-size:38px; line-height:1.3; font-weight:400; color:${leise}; margin-top:16px;
      max-width:2200px; }
  /* Ganzes Fenster, mittig in der Restfläche — nichts wird abgeschnitten. */
  .buehne { flex:1; margin-top:${LUFT}px; display:flex; align-items:center; justify-content:center;
            min-height:0; }
  .fenster { border-radius:18px; overflow:hidden; max-width:100%; max-height:100%;
             box-shadow:${schatten}; outline:1px solid ${rahmen}; outline-offset:-1px; }
  .fenster img { display:block; max-width:100%; max-height:${H - 2 * RAND - TEXTHOEHE - LUFT}px;
                 width:auto; height:auto; }
  </style>
  <div class="blatt">
    <div class="kopf">
      <div class="strich"></div>
      <h1>${zeilen(titel)}</h1>
      <p>${esc(unter)}</p>
    </div>
    <div class="buehne"><div class="fenster"><img src="${BASIS}${bild}"></div></div>
  </div>`;
}

const browser = await webkit.launch();
let zahl = 0;
for (const sprache of SPRACHEN) {
  const ordner = path.join(ZIEL, sprache);
  fs.mkdirSync(ordner, { recursive: true });
  // Alte Ergebnisse weg: die Nummern richten sich nach der Reihenfolge im JSON, ein
  // umgestelltes Motiv liesse sonst eine verwaiste Datei mit alter Nummer liegen —
  // und die landet beim Hochladen versehentlich mit im Store.
  for (const alt of fs.readdirSync(ordner)) {
    if (alt.endsWith(".png")) fs.unlinkSync(path.join(ordner, alt));
  }
  let nr = 0;
  for (const eintrag of DATEN.bilder) {
    nr += 1;
    const quelle = path.join(QUELLE, `${eintrag.quelle}-${sprache}.png`);
    if (!fs.existsSync(quelle)) { console.error(`ABBRUCH: Rohaufnahme fehlt: ${path.relative(ROOT, quelle)}`); process.exit(1); }
    const text = eintrag[sprache];
    if (!text) { console.error(`ABBRUCH: kein Text für ${sprache} in ${eintrag.quelle}`); process.exit(1); }
    const page = await browser.newPage({ viewport: { width: B, height: H }, deviceScaleFactor: 1 });
    await page.setContent(seite(path.relative(ROOT, quelle), text.titel, text.unter, eintrag.grund === "dunkel"),
                          { waitUntil: "networkidle" });
    await page.evaluate(() => document.fonts.ready);
    const datei = path.join(ordner, `${String(nr).padStart(2, "0")}-${eintrag.quelle}.png`);
    await page.screenshot({ path: datei });
    await page.close();
    zahl += 1;
  }
  console.log(`✓ ${sprache}: ${nr} Motive → ${path.relative(ROOT, ordner)}`);
}
await browser.close();
server.close();
console.log(`${zahl} Bilder montiert (${B}×${H}).`);
