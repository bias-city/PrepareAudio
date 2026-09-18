#!/usr/bin/env python3
"""Erfundenes Demo-Material für PrepareAudio: Bildschirmfotos, Tests, App-Prüfung.

  python3 scripts/demo-material.py <zielordner>

Nichts davon ist eine echte Aufnahme. Zwei Ansteckmikrofone («1» und «2») nehmen
zwölf Minuten auf; der Recorder schneidet jede Aufnahme in drei Teile. Die Stimmen
sind synthetische Silbenfolgen (Obertonreihe mit Formanten), dazu Raumgeräusche.

  aufnahmen/1/REC001_…WAV (3 Teile)   Sender 1
  aufnahmen/2/REC001_…WAV (3 Teile)   Sender 2, startet 3,9 s später, Uhr 4 ppm schneller
  mastern/gespraech-leise.wav, mastern/gespraech-laut.wav

Ablauf im Material: 0–5 min gemeinsames Gespräch, 5–8 min zwei getrennte
Gespräche, 8–12 min wieder gemeinsam. Die Teile tragen Broadcast-WAV-Metadaten
(bext) mit exakter TimeReference; daran erkennt «Zusammenfügen» sie auch bei
kleinen Dateien (docs/CHUNK-ERKENNUNG.md, Regel 4.3). Fester Zufallsstart: jedes
Erzeugen ergibt dieselben Dateien.
"""
from __future__ import annotations

import struct
import sys
from pathlib import Path

import numpy as np

SR = 24_000
DATUM = "2026-05-12"
START = (10, 15, 0)  # Uhrzeit Sender 1
MIN = 60 * SR
TEILE = [int(4.5 * MIN), int(4.5 * MIN), 3 * MIN]
VERSATZ_S = 3.9
DRIFT = 4e-6


def stimme(rng, n, f0, dichte=1.0):
    """Silbenfolge: Obertonreihe mit zwei Formanten, harter Einsatz, Pausen."""
    out = np.zeros(n, dtype=np.float32)
    t = 0
    while t < n:
        if rng.random() < 0.12:  # Sprechpause
            t += int(rng.uniform(0.6, 2.5) * SR / dichte)
            continue
        dauer = int(rng.uniform(0.09, 0.26) * SR)
        ende = min(n, t + dauer)
        k = np.arange(ende - t) / SR
        f = f0 * rng.uniform(0.85, 1.25)
        f1, f2 = rng.uniform(300, 900), rng.uniform(1100, 2600)
        s = np.zeros_like(k)
        for h in range(1, int(5000 / f)):
            fh = f * h
            g = np.exp(-((fh - f1) / 220) ** 2) + 0.6 * np.exp(-((fh - f2) / 380) ** 2) + 0.03
            s += g / h**0.3 * np.sin(2 * np.pi * fh * k + rng.uniform(0, 6.28))
        s += 0.25 * rng.standard_normal(len(k)) * np.exp(-k * 40)  # Konsonant
        huelle = np.minimum(1, k / 0.008) * np.exp(-k * rng.uniform(3, 9))
        out[t:ende] += (s * huelle * rng.uniform(0.5, 1.0)).astype(np.float32)
        t = ende + int(rng.uniform(0.02, 0.12) * SR)
    return out / (np.abs(out).max() + 1e-9)


def raum(rng, n, je_minute=5):
    """Tassen, Stühle, Papier: kurze breitbandige Ereignisse."""
    out = np.zeros(n, dtype=np.float32)
    for _ in range(int(n / MIN * je_minute)):
        t = int(rng.uniform(0, n - SR))
        k = np.arange(int(rng.uniform(0.03, 0.2) * SR)) / SR
        out[t:t + len(k)] += (rng.standard_normal(len(k)) * np.exp(-k * rng.uniform(15, 60)) * rng.uniform(0.3, 1)).astype(np.float32)
    return out


def abwechselnd(rng, n, a, b):
    """Gespräch: mal spricht A, mal B (weiche Übergänge)."""
    gate = np.zeros(n, dtype=np.float32)
    t, wer = 0, 0
    while t < n:
        d = int(rng.uniform(4, 14) * SR)
        gate[t:t + d] = wer
        t, wer = t + d, 1 - wer
    kern = np.ones(SR // 10, dtype=np.float32) / (SR // 10)
    gate = np.convolve(gate, kern, mode="same")
    return a * (1 - gate), b * gate


def bext(zeit_s: float, frames_seit_mitternacht: int) -> bytes:
    h, m, s = int(zeit_s // 3600), int(zeit_s % 3600 // 60), int(zeit_s % 60)
    koerper = struct.pack(
        "<256s32s32s10s8sIIH64s10s180s",
        b"PrepareAudio Demo (synthetisch)", b"PrepareAudio", b"DEMO",
        DATUM.encode(), f"{h:02d}:{m:02d}:{s:02d}".encode(),
        frames_seit_mitternacht & 0xFFFFFFFF, frames_seit_mitternacht >> 32, 1, b"", b"", b"",
    )
    return b"bext" + struct.pack("<I", len(koerper)) + koerper


def schreibe_wav(pfad: Path, x: np.ndarray, kopf: bytes = b"", kanaele: int = 1) -> None:
    pcm = (np.clip(x, -1, 1) * 32767).astype("<i2").tobytes()
    fmt = b"fmt " + struct.pack("<IHHIIHH", 16, 1, kanaele, SR, SR * 2 * kanaele, 2 * kanaele, 16)
    daten = b"data" + struct.pack("<I", len(pcm)) + pcm
    rumpf = b"WAVE" + kopf + fmt + daten
    pfad.parent.mkdir(parents=True, exist_ok=True)
    pfad.write_bytes(b"RIFF" + struct.pack("<I", len(rumpf)) + rumpf)


def sender(ziel: Path, name: str, x: np.ndarray, start_s: float) -> None:
    pos = 0
    for laenge in TEILE:
        t = start_s + pos / SR
        h, m, s = int(t // 3600), int(t % 3600 // 60), int(t % 60)
        datei = ziel / name / f"REC001_{DATUM.replace('-', '')}_{h:02d}{m:02d}{s:02d}.WAV"
        schreibe_wav(datei, x[pos:pos + laenge], bext(t, round(start_s * SR) + pos))
        print(f"  {datei.relative_to(ziel.parent)}  {laenge / SR / 60:.1f} min")
        pos += laenge


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    ziel = Path(sys.argv[1])
    rng = np.random.default_rng(20260512)
    n = sum(TEILE)
    extra = int((VERSATZ_S + 1) * SR)
    N = n + extra  # gemeinsame Szene, etwas länger als eine Aufnahme

    a, b = stimme(rng, N, 118), stimme(rng, N, 205)          # gemeinsames Gespräch
    a, b = abwechselnd(rng, N, a, b)
    c, d = stimme(rng, N, 140, 0.8), stimme(rng, N, 182, 0.8)  # zwei getrennte Gespräche
    ereignisse, fremd1, fremd2 = raum(rng, N), raum(rng, N, 3), raum(rng, N, 3)

    getrennt = np.zeros(N, dtype=np.float32)
    getrennt[5 * MIN:8 * MIN] = 1
    getrennt = np.convolve(getrennt, np.ones(SR, dtype=np.float32) / SR, mode="same")
    gemeinsam = 1 - getrennt

    rausch = lambda: 0.002 * rng.standard_normal(N).astype(np.float32)  # noqa: E731
    mic1 = gemeinsam * (0.55 * a + 0.14 * b + 0.30 * ereignisse) + getrennt * (0.55 * c + 0.25 * fremd1) + rausch()
    nah = np.roll(a, int(0.003 * SR))  # 1 m weiter weg
    mic2 = gemeinsam * (0.50 * b + 0.13 * nah + 0.28 * ereignisse) + getrennt * (0.5 * d + 0.25 * fremd2) + rausch()

    print("Aufnahmen:")
    start1 = START[0] * 3600 + START[1] * 60 + START[2]
    sender(ziel / "aufnahmen", "1", mic1[:n], start1)
    # Sender 2: startet später, eigene Uhr läuft 4 ppm schneller (Drift)
    quelle = (VERSATZ_S * SR) + np.arange(n) * (1 + DRIFT)
    mic2_eigen = np.interp(quelle, np.arange(N), mic2).astype(np.float32)
    sender(ziel / "aufnahmen", "2", mic2_eigen, start1 + VERSATZ_S)

    print("Mastern:")
    stueck = slice(30 * SR, 150 * SR)
    stereo = np.stack([mic1[stueck], mic2[stueck]], axis=1).reshape(-1)
    schreibe_wav(ziel / "mastern" / "gespraech-leise.wav", stereo * 0.12, kanaele=2)
    schreibe_wav(ziel / "mastern" / "gespraech-laut.wav", np.tanh(mic1[stueck] * 4) * 0.95)
    print("  mastern/gespraech-leise.wav, mastern/gespraech-laut.wav")


if __name__ == "__main__":
    main()
