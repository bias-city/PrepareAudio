#!/usr/bin/env node
// Bildschirmfotos der echten Oberfläche (ui/) im Browser, mit scripts/demo-backend.js.
//   node scripts/screenshots.mjs <zielordner> [--breite 1200 --hoehe 750 --faktor 2] [--sprachen de,en] [--dunkel]
//   node scripts/screenshots.mjs appstore/upload --store   Store-Bilder: 2880 × 1800, JPEG ohne Alpha,
//                                                            je Sprache ein Ordner, nummeriert, dazu die Timeline dunkel
//   node scripts/screenshots.mjs <ziel> --groessen 600x480,940x720,1440x900 [--daten <ordner>]
//       Layout-Prüfung: jede Ansicht in jeder Fenstergrösse (Faktor 1, nur die erste Sprache)
//   node scripts/screenshots.mjs ui/hilfe --handbuch   Bilder fürs Handbuch: hell und dunkel,
//                                                     je Sprache ein Ordner, JPEG 1200 × 750
//   node scripts/screenshots.mjs site/img --site      Bilder für bias.city/prepareaudio:
//                                                     hero 1600×1000, Motive 1200×750, PNG, je Sprache
// Braucht Playwright (npx playwright install webkit). Motive: leer, merge, sync, master, info.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import http from "node:http";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const arg = (name, std) => { const i = process.argv.indexOf(`--${name}`); return i > 0 ? process.argv[i + 1] : std; };
const ZIEL = process.argv[2];
if (!ZIEL || ZIEL.startsWith("--")) { console.error("Zielordner fehlt"); process.exit(1); }
const STORE = process.argv.includes("--store");
const HANDBUCH = process.argv.includes("--handbuch");
const SITE = process.argv.includes("--site");
const B = STORE ? 1440 : +arg("breite", 1200), H = STORE ? 900 : +arg("hoehe", 750), F = HANDBUCH || SITE ? 1 : +arg("faktor", 2);
const SPRACHEN = arg("sprachen", "de,en,fr,it").split(",");
const DUNKEL = process.argv.includes("--dunkel");
const require = createRequire(process.env.PLAYWRIGHT_FROM || import.meta.url);
const { webkit } = require("playwright");

fs.mkdirSync(ZIEL, { recursive: true });
const DATEN = arg("daten", path.join(ROOT, "scripts/demo-data"));
const GROESSEN = arg("groessen", "").split(",").filter(Boolean).map((g) => g.split("x").map(Number));
const lies = (n) => JSON.parse(fs.readFileSync(path.join(DATEN, n), "utf8"));
const backend = fs.readFileSync(path.join(ROOT, "scripts/demo-backend.js"), "utf8");

// Die Oberfläche wird über einen kleinen Server ausgeliefert, nicht über file://: nur so darf
// die Seite ihre eigene licenses.json nachladen (WebKit verbietet das bei file://).
const TYPEN = { ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".css": "text/css; charset=utf-8", ".json": "application/json; charset=utf-8", ".jpg": "image/jpeg", ".png": "image/png", ".svg": "image/svg+xml" };
const server = http.createServer((req, res) => {
  const rel = decodeURIComponent(req.url.split("?")[0]).replace(/^\/+/, "") || "index.html";
  const datei = path.join(ROOT, "ui", rel);
  if (!datei.startsWith(path.join(ROOT, "ui"))) { res.writeHead(403).end(); return; }
  fs.readFile(datei, (err, buf) => {
    if (err) { res.writeHead(404).end(); return; }
    res.writeHead(200, { "content-type": TYPEN[path.extname(datei)] || "application/octet-stream" });
    res.end(buf);
  });
});
await new Promise((ok) => server.listen(0, "127.0.0.1", ok));
const BASIS = `http://127.0.0.1:${server.address().port}/`;

const browser = await webkit.launch();
const REIHE = ["sync", "merge", "master", "sync-dunkel", "handbuch", "leer", "info"]; // Reihenfolge im Store
async function lauf(lang, dunkel, motive, groesse) {
  const p = lies(`plan-${lang}.json`);
  const demo = { scan: lies(`scan-${lang}.json`), plan: p.plan, peaks: p.peaks, master: lies(`master-${lang}.json`) };
  const [b, h] = groesse || [B, H];
  const page = await browser.newPage({ viewport: { width: b, height: h }, deviceScaleFactor: groesse ? 1 : F, colorScheme: dunkel ? "dark" : "light" });
  page.on("pageerror", (e) => { if (!/licenses\.json/.test(e.message)) console.log(`[${lang}] FEHLER`, e.message); });
  await page.addInitScript(`localStorage.setItem("prepareaudio.lang", ${JSON.stringify(lang)}); window.PA_DEMO = ${JSON.stringify(demo)}; ${backend}`);
  await page.goto(BASIS + "index.html");
  await page.evaluate((l) => window.I18N && I18N.setLang(l), lang);
  const knips = async (name) => {
    if (!motive.includes(name)) return;
    await page.waitForTimeout(400);
    const key = dunkel ? `${name}-dunkel` : name;
    const datei = SITE ? path.join(ZIEL, `${name}-${lang}.png`) : HANDBUCH ? path.join(ZIEL, lang, dunkel ? "dunkel" : "hell", `${name}.jpg`) : groesse ? path.join(ZIEL, `${name}-${b}x${h}.png`) : STORE ? path.join(ZIEL, lang, `${String(REIHE.indexOf(key) + 1).padStart(2, "0")}-${key}.jpg`) : path.join(ZIEL, `${key}-${lang}.png`);
    fs.mkdirSync(path.dirname(datei), { recursive: true });
    await page.screenshot(STORE || HANDBUCH ? { path: datei, type: "jpeg", quality: HANDBUCH ? 82 : 92 } : { path: datei });
  };
  await knips("leer");
  await page.click("#pick"); await page.waitForTimeout(300);
  await page.evaluate(() => document.querySelectorAll(".partlist").forEach((d) => { d.open = true; }));
  await knips("merge");
  await page.click("#tabs button[data-mode=sync]"); await page.click("#sync-pick"); await page.waitForTimeout(600);
  await knips("sync");
  await knips("hero");
  if (motive.includes("edit")) {
    // Menü an einem Abschnitt: zeigt Ziel und Position
    const box = await page.locator("#ed-canvas").boundingBox();
    await page.mouse.click(box.x + 330, box.y + 24 + 18 + 23);
    await page.waitForTimeout(300);
    await knips("edit");
    await page.keyboard.press("Escape");
  }
  if (motive.includes("done")) {
    await page.evaluate(() => {
      const f = document.querySelector("#sync-finished");
      const done = window.syncState && window.syncState.plan ? window.syncState.plan.items.length : 4;
      f.innerHTML = `<div class="finished-card"><div class="finished-icon">\u2713</div><h2>${done} von ${done} erledigt</h2>`
        + `<p>${done} Dateien geschrieben</p><p class="path">/Users/demo/Gespraech/sync</p>`
        + `<div class="finished-actions"><button class="btn">Ordner öffnen</button><button class="btn primary">Weiter zum Mastern</button></div></div>`;
      f.hidden = false;
      document.querySelector("#sync-results").hidden = true;
    });
    await knips("done");
    await page.evaluate(() => { document.querySelector("#sync-finished").hidden = true; document.querySelector("#sync-results").hidden = false; });
  }
  await page.click("#tabs button[data-mode=master]"); await page.click("#master-pick"); await knips("master");
  await page.evaluate(() => window.openInfo && window.openInfo());
  await page.waitForTimeout(250);
  await knips("info");
  await page.close();
}
for (const g of GROESSEN) { await lauf(SPRACHEN[0], DUNKEL, ["leer", "merge", "sync", "master", "info"], g); console.log("✓", g.join("x")); }
/** Das Handbuch ist eine eigene Seite: eigener Aufruf, gleiche Grösse wie die übrigen Motive. */
async function handbuchSeite(lang) {
  const page = await browser.newPage({ viewport: { width: B, height: H }, deviceScaleFactor: F });
  await page.addInitScript(`localStorage.setItem("prepareaudio.lang", ${JSON.stringify(lang)}); localStorage.setItem("prepareaudio.hilfe.kapitel", "timeline");`);
  await page.goto(BASIS + "hilfe.html");
  await page.waitForTimeout(400);
  const datei = path.join(ZIEL, lang, `${String(REIHE.indexOf("handbuch") + 1).padStart(2, "0")}-handbuch.jpg`);
  fs.mkdirSync(path.dirname(datei), { recursive: true });
  await page.screenshot({ path: datei, type: "jpeg", quality: 92 });
  await page.close();
}

for (const lang of GROESSEN.length ? [] : SPRACHEN) {
  if (SITE) {
    await lauf(lang, false, ["hero"], [1600, 1000]);
    await lauf(lang, false, ["merge", "sync", "edit", "done", "master", "info"], [1200, 750]);
  } else if (STORE) {
    await lauf(lang, false, ["leer", "merge", "sync", "master", "info"]);
    await lauf(lang, true, ["sync"]);
    await handbuchSeite(lang);
  } else if (HANDBUCH) { for (const d of [false, true]) await lauf(lang, d, ["leer", "merge", "sync", "master"]); }
  else await lauf(lang, DUNKEL, ["leer", "merge", "sync", "master", "info"]);
  console.log("✓", lang);
}
await browser.close();
server.close();
