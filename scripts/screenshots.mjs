#!/usr/bin/env node
// Bildschirmfotos der echten Oberfläche (ui/) im Browser, mit scripts/demo-backend.js.
//   node scripts/screenshots.mjs <zielordner> [--breite 1200 --hoehe 750 --faktor 2] [--sprachen de,en] [--dunkel]
//   node scripts/screenshots.mjs appstore/upload --store   Store-Bilder: 2880 × 1800, JPEG ohne Alpha,
//                                                            je Sprache ein Ordner, nummeriert, dazu die Timeline dunkel
//   node scripts/screenshots.mjs <ziel> --groessen 600x480,940x720,1440x900 [--daten <ordner>]
//       Layout-Prüfung: jede Ansicht in jeder Fenstergrösse (Faktor 1, nur die erste Sprache)
//   node scripts/screenshots.mjs ui/hilfe --handbuch   Bilder fürs Handbuch: hell und dunkel,
//                                                     je Sprache ein Ordner, JPEG 1200 × 750
// Braucht Playwright (npx playwright install webkit). Motive: leer, merge, sync, master, info.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath, pathToFileURL } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const arg = (name, std) => { const i = process.argv.indexOf(`--${name}`); return i > 0 ? process.argv[i + 1] : std; };
const ZIEL = process.argv[2];
if (!ZIEL || ZIEL.startsWith("--")) { console.error("Zielordner fehlt"); process.exit(1); }
const STORE = process.argv.includes("--store");
const HANDBUCH = process.argv.includes("--handbuch");
const B = STORE ? 1440 : +arg("breite", 1200), H = STORE ? 900 : +arg("hoehe", 750), F = HANDBUCH ? 1 : +arg("faktor", 2);
const SPRACHEN = arg("sprachen", "de,en,fr,it").split(",");
const DUNKEL = process.argv.includes("--dunkel");
const require = createRequire(process.env.PLAYWRIGHT_FROM || import.meta.url);
const { webkit } = require("playwright");

fs.mkdirSync(ZIEL, { recursive: true });
const DATEN = arg("daten", path.join(ROOT, "scripts/demo-data"));
const GROESSEN = arg("groessen", "").split(",").filter(Boolean).map((g) => g.split("x").map(Number));
const lies = (n) => JSON.parse(fs.readFileSync(path.join(DATEN, n), "utf8"));
const backend = fs.readFileSync(path.join(ROOT, "scripts/demo-backend.js"), "utf8");
const lizenzen = fs.readFileSync(path.join(ROOT, "ui/licenses.json"), "utf8");

const browser = await webkit.launch();
const REIHE = ["sync", "merge", "master", "sync-dunkel", "leer", "info"]; // Reihenfolge im Store
async function lauf(lang, dunkel, motive, groesse) {
  const p = lies(`plan-${lang}.json`);
  const demo = { scan: lies(`scan-${lang}.json`), plan: p.plan, peaks: p.peaks, master: lies(`master-${lang}.json`) };
  const [b, h] = groesse || [B, H];
  const page = await browser.newPage({ viewport: { width: b, height: h }, deviceScaleFactor: groesse ? 1 : F, colorScheme: dunkel ? "dark" : "light" });
  page.on("pageerror", (e) => { if (!/licenses\.json/.test(e.message)) console.log(`[${lang}] FEHLER`, e.message); });
  await page.route("**/licenses.json", (r) => r.fulfill({ contentType: "application/json", body: lizenzen }));
  await page.addInitScript(`localStorage.setItem("prepareaudio.lang", ${JSON.stringify(lang)}); window.PA_DEMO = ${JSON.stringify(demo)}; ${backend}`);
  await page.goto(pathToFileURL(path.join(ROOT, "ui/index.html")).href);
  await page.evaluate((l) => window.I18N && I18N.setLang(l), lang);
  const knips = async (name) => {
    if (!motive.includes(name)) return;
    await page.waitForTimeout(400);
    const key = dunkel ? `${name}-dunkel` : name;
    const datei = HANDBUCH ? path.join(ZIEL, lang, dunkel ? "dunkel" : "hell", `${name}.jpg`) : groesse ? path.join(ZIEL, `${name}-${b}x${h}.png`) : STORE ? path.join(ZIEL, lang, `${String(REIHE.indexOf(key) + 1).padStart(2, "0")}-${key}.jpg`) : path.join(ZIEL, `${key}-${lang}.png`);
    fs.mkdirSync(path.dirname(datei), { recursive: true });
    await page.screenshot(STORE || HANDBUCH ? { path: datei, type: "jpeg", quality: HANDBUCH ? 82 : 92 } : { path: datei });
  };
  await knips("leer");
  await page.click("#pick"); await page.waitForTimeout(300);
  await page.evaluate(() => document.querySelectorAll(".partlist").forEach((d) => { d.open = true; }));
  await knips("merge");
  await page.click("#tabs button[data-mode=sync]"); await page.click("#sync-pick"); await page.waitForTimeout(600); await knips("sync");
  await page.click("#tabs button[data-mode=master]"); await page.click("#master-pick"); await knips("master");
  await page.click("#info-open"); await knips("info");
  await page.close();
}
for (const g of GROESSEN) { await lauf(SPRACHEN[0], DUNKEL, ["leer", "merge", "sync", "master", "info"], g); console.log("✓", g.join("x")); }
for (const lang of GROESSEN.length ? [] : SPRACHEN) {
  if (HANDBUCH) { for (const d of [false, true]) await lauf(lang, d, ["leer", "merge", "sync", "master"]); }
  else if (STORE) { await lauf(lang, false, ["leer", "merge", "sync", "master", "info"]); await lauf(lang, true, ["sync"]); }
  else await lauf(lang, DUNKEL, ["leer", "merge", "sync", "master", "info"]);
  console.log("✓", lang);
}
await browser.close();
