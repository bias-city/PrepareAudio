#!/usr/bin/env node
// Mac-App-Store-Build: App mit Feature `mas` bauen → mit «Apple Distribution» und den
// Store-Entitlements versiegeln → Prüfungen → .pkg mit dem Installer-Zertifikat.
// Hochgeladen wird mit Transporter.app.
//
// Voraussetzungen (einmalig, Apple-Portal): Identifier city.bias.prepareaudio, Profil unter
// src-tauri/profiles/PrepareAudio.provisionprofile, Zertifikate «Apple Distribution» und
// «3rd Party Mac Developer Installer» im Schlüsselbund; scripts/baue-lame.sh ist gelaufen.
//
//   node scripts/release-mas.mjs [--build 2]      Store-Paket (CFBundleVersion = --build)
//   … --ab-signatur                             ohne Neubau: nur versiegeln, prüfen, paketieren
//   node scripts/release-mas.mjs --probe          Sandbox-Probe OHNE Profil: mit Developer ID
//                                                 versiegelt, lokal startbar (Phase 2 des Plans)
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const TAURI = path.join(ROOT, "src-tauri");
const APP = path.join(TAURI, "target/release/bundle/macos/PrepareAudio.app");
const PKG = path.join(TAURI, "target/release/bundle/mas/PrepareAudio.pkg");
const PROFIL = path.join(TAURI, "profiles/PrepareAudio.provisionprofile");
const ENT = path.join(TAURI, "entitlements.mas.plist");
const probe = process.argv.includes("--probe");
const arg = (n) => (process.argv.includes(n) ? process.argv[process.argv.indexOf(n) + 1] : null);
const sh = (cmd, args, opts = {}) => execFileSync(cmd, args, { stdio: "inherit", cwd: ROOT, ...opts });
const out = (cmd, args) => execFileSync(cmd, args, { encoding: "utf8", cwd: ROOT });
const abbruch = (text) => { console.error("ABBRUCH: " + text); process.exit(1); };

function identitaet(name, nurCodesign = true) {
  const liste = out("/usr/bin/security", ["find-identity", "-v", ...(nurCodesign ? ["-p", "codesigning"] : [])]);
  const z = liste.split("\n").find((l) => l.includes(name));
  if (!z) abbruch(`Zertifikat «${name}» fehlt im Schlüsselbund.`);
  return { hash: z.trim().split(" ")[1], name: z.match(/"([^"]+)"/)[1] };
}
// Probe: dieselbe Developer ID wie der DMG-Build (tauri.conf.json) — im Schlüsselbund können
// mehrere liegen, und eine fremde wartet womöglich auf eine Freigabe am Bildschirm.
const SIGN = probe ? JSON.parse(fs.readFileSync(path.join(TAURI, "tauri.conf.json"), "utf8")).bundle.macOS.signingIdentity
  : identitaet("Apple Distribution:").hash;
if (!probe && !fs.existsSync(PROFIL)) abbruch(`Provisioning-Profil fehlt: ${PROFIL}`);
if (!fs.existsSync(path.join(TAURI, "frameworks/libmp3lame.dylib"))) sh("/bin/sh", [path.join(ROOT, "scripts/baue-lame.sh")]);

// --ab-signatur: die schon gebaute App nur neu versiegeln, prüfen, paketieren
if (!process.argv.includes("--ab-signatur")) {
console.log("1/4 App bauen (Feature mas)");
const config = { bundle: { macOS: { ...(arg("--build") ? { bundleVersion: arg("--build") } : {}) } } };
sh("npx", ["tauri", "build", "--bundles", "app", "--config", JSON.stringify(config), "--", "--features", "mas"],
   { env: { ...process.env, PA_RELEASE: "1", APPLE_ID: undefined, APPLE_PASSWORD: undefined, APPLE_TEAM_ID: undefined } });
}

console.log(`2/4 Versiegeln (${probe ? "Sandbox-Probe, Developer ID" : "Apple Distribution"})`);
let ent = ENT;
if (probe) {
  // Ohne Profil darf die Signatur keine Identifier-Berechtigungen tragen.
  ent = path.join(os.tmpdir(), "prepareaudio-probe.entitlements.plist");
  fs.copyFileSync(ENT, ent);
  for (const k of ["com.apple.application-identifier", "com.apple.developer.team-identifier"]) sh("/usr/bin/plutil", ["-remove", k.replaceAll(".", "\\."), ent]);
} else {
  fs.copyFileSync(PROFIL, path.join(APP, "Contents/embedded.provisionprofile"));
}
sh("/usr/bin/codesign", ["--force", "--options", "runtime", "--timestamp", "-s", SIGN, path.join(APP, "Contents/Frameworks/libmp3lame.dylib")]);
sh("/usr/bin/codesign", ["--force", "--options", "runtime", "--timestamp", "--entitlements", ent, "-s", SIGN, APP]);

console.log("3/4 Prüfungen");
sh("/usr/bin/codesign", ["--verify", "--deep", "--strict", "--verbose=2", APP]);
const rechte = out("/usr/bin/codesign", ["-d", "--entitlements", "-", "--xml", APP]);
for (const k of ["com.apple.security.app-sandbox", "com.apple.security.files.user-selected.read-write", ...(probe ? [] : ["com.apple.application-identifier"])]) {
  if (!rechte.includes(k)) abbruch(`Entitlement ${k} fehlt`);
}
const programm = path.join(APP, "Contents/MacOS", out("/usr/libexec/PlistBuddy", ["-c", "Print :CFBundleExecutable", path.join(APP, "Contents/Info.plist")]).trim());
// otool nennt in der ersten Zeile die geprüfte Datei selbst — die zählt nicht.
const bindung = [["-L"], ["-l"]].map((a) => out("/usr/bin/otool", [...a, programm]).split("\n").slice(1).join("\n")).join("\n");
const fremd = bindung.split("\n").filter((z) => /^\s*(path |\t?\/)/.test(z) && /\/(opt|usr\/local|Users)\//.test(z));
if (fremd.length) abbruch("das Programm trägt einen Pfad dieses Rechners:\n" + fremd.join("\n"));
if (!bindung.includes("@rpath/libmp3lame.dylib")) abbruch("libmp3lame ist nicht dynamisch über @rpath gebunden");
// Apple verlangt app-sandbox auf JEDEM ausführbaren Programm im Paket (Fehler 90296)
const programme = out("/bin/sh", ["-c", `find "${APP}" -type f -perm +111 -print0 | xargs -0 file | grep "Mach-O.*executable" | cut -d: -f1`]).trim().split("\n").filter(Boolean);
for (const p of programme) {
  if (!out("/usr/bin/codesign", ["-d", "--entitlements", "-", "--xml", p]).includes("com.apple.security.app-sandbox")) abbruch(`Programm ohne Sandbox: ${p}`);
}
console.log(`${programme.length} ausführbares Programm, mit app-sandbox; keine Rechnerpfade; LAME über @rpath`);

if (probe) {
  console.log(`4/4 übersprungen. Sandbox-Probe: open "${APP}"`);
} else {
  console.log("4/4 Installer-Paket");
  fs.mkdirSync(path.dirname(PKG), { recursive: true });
  fs.rmSync(PKG, { force: true });
  sh("/usr/bin/xcrun", ["productbuild", "--sign", identitaet("3rd Party Mac Developer Installer", false).name, "--component", APP, "/Applications", PKG]);
  sh("/usr/sbin/pkgutil", ["--check-signature", PKG]);
  console.log(`Paket: ${PKG} — mit Transporter.app hochladen.`);
}
