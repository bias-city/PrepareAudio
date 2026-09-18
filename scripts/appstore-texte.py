#!/usr/bin/env python3
"""Store-Texte für App Store Connect: prüft Apples Längen und schreibt
appstore/texte/<sprache>/<feld>.txt zum Hineinkopieren. Anrede überall Du (fr tu, it tu).
Keine Gerätemarken (Richtlinie 5.2.1). Aufruf: python3 scripts/appstore-texte.py"""
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LIMIT = {"name": 30, "untertitel": 30, "werbetext": 170, "schlagwoerter": 100, "beschreibung": 4000, "neu": 4000}
URLS = {
    "support": "https://bias.city/prepareaudio/",
    "marketing": "https://bias.city/prepareaudio/",
    "datenschutz": "https://bias.city/prepareaudio/privacy.html",
}
COPYRIGHT = "2026 B/IAS – Basel Institut für angewandte Stadtforschung"

T = {
"de": {
"portal": "Deutsch",
"name": "PrepareAudio",
"untertitel": "Interview-Audio vorbereiten",
"werbetext": "Aus Recorder-Stückwerk wird ein sauberes Gespräch: Teile verlustfrei zusammenfügen, zwei Mikrofone synchronisieren, als MP3 mastern. Alles lokal auf deinem Mac.",
"schlagwoerter": "Audio,Recorder,WAV,zusammenfügen,synchronisieren,Mastern,MP3,LUFS,Interview,Podcast,Lautheit,offline",
"beschreibung": """PrepareAudio macht aus dem, was Audiorecorder und Ansteckmikrofone abliefern, fertige Aufnahmen: Teile zusammenfügen, zwei Mikrofone synchronisieren, als MP3 mastern. Drei Schritte, jeder für sich nutzbar. Alles läuft lokal auf deinem Mac: keine Cloud, kein Konto, keine Daten verlassen den Rechner.

1 · ZUSAMMENFÜGEN
Viele Recorder schneiden lange Aufnahmen in Teile. PrepareAudio setzt sie wieder zusammen.
• Ordner hineinziehen, alle Unterordner werden durchsucht, egal wie sortiert
• Erkennung über Aufnahmezeit in den Metadaten (Broadcast-WAV, iXML), Dateiname oder Dateidatum
• Bei zwei Sendern mit gleichen Dateinamen entscheidet das Audio an der Nahtstelle
• Verlustfrei: die Audiodaten werden bitgenau kopiert, ab 4 GB automatisch als RF64
• Unsichere Verkettungen sind markiert und nicht vorausgewählt

2 · SYNCHRONISIEREN
Zwei Personen, zwei Ansteckmikrofone, zwei Uhren. PrepareAudio findet die gemeinsamen Ereignisse und legt beide Spuren übereinander.
• Misst Zeitversatz und Uhrendrift auf Millisekunden
• Gemeinsame Abschnitte werden Stereo (links die eine, rechts die andere Person), getrennte Gespräche Mono
• Timeline wie im Schnittprogramm: trimmen, trennen, zusammenführen, löschen, Stereo oder Mono wählen, rückgängig machen
• Vorhören genau so, wie die Datei entsteht; Sender stumm oder solo
• Eingaben: WAV sowie MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG, etwa vom Handy

3 · MASTERN
Gleich laute, gut hörbare Dateien zum Weitergeben, Transkribieren oder Veröffentlichen.
• Lautheit nach EBU R128 (ITU-R BS.1770-4) mit True Peak und Lautheitsumfang
• Feste Verstärkung auf −16 LUFS, Limiter bei −1,5 dBTP, keine Kompression: die Sprachdynamik bleibt
• MP3 mit 192 kbit/s; das fertige MP3 wird nachgemessen und bei Bedarf nachgeregelt
• Ehrlicher Fortschritt: die Prozente zählen nur erledigte Arbeit

DEINE ORIGINALE BLEIBEN UNANGETASTET
Die App verändert oder überschreibt nie eine Quelldatei. Ergebnisse landen in den Ordnern tracks, sync und master an dem Ort, den du wählst. Schon Vorhandenes wird erkannt und übersprungen.

QUELLENSCHUTZ: KEINE DATEN VERLASSEN DEN RECHNER
Wer Gespräche aufnimmt, schuldet den Beteiligten Vertraulichkeit. PrepareAudio erhebt keine Daten, hat keine Telemetrie, braucht kein Mikrofon und lädt nichts nach. Die App läuft in der App Sandbox von macOS.

PASST ZU RESEARCHTRANSCRIPT
Die gemasterten Dateien sind der ideale Ausgang für die Transkription mit ResearchTranscript, ebenfalls von B/IAS.

OPEN SOURCE
PrepareAudio ist freie Software unter der AGPL. Der Quellcode liegt offen auf GitHub und lässt sich prüfen. Entwickelt am B/IAS – Basel Institut für angewandte Stadtforschung.

VORAUSSETZUNGEN
Mac mit Apple Silicon, macOS 12 oder neuer. Oberfläche in Deutsch, Englisch, Französisch und Italienisch.""",
"neu": "Erste Fassung im Mac App Store.",
},
"en": {
"portal": "Englisch (UK)",
"name": "PrepareAudio",
"untertitel": "Prepare interview audio",
"werbetext": "From recorder fragments to a clean conversation: merge parts losslessly, sync two microphones, master as MP3. Everything stays on your Mac.",
"schlagwoerter": "audio,recorder,WAV,merge,join,sync,mastering,MP3,LUFS,interview,podcast,loudness,offline",
"beschreibung": """PrepareAudio turns what audio recorders and lavalier microphones deliver into finished recordings: merge the parts, synchronise two microphones, master as MP3. Three steps, each useful on its own. Everything runs locally on your Mac: no cloud, no account, no data leaves the computer.

1 · MERGE
Many recorders split long recordings into parts. PrepareAudio puts them back together.
• Drag in a folder; all subfolders are searched, however they are sorted
• Detection by recording time in the metadata (Broadcast WAV, iXML), file name or file date
• With two transmitters and identical file names, the audio at the seam decides
• Lossless: the audio data is copied bit for bit, from 4 GB automatically as RF64
• Uncertain chains are marked and not preselected

2 · SYNCHRONISE
Two people, two lavalier microphones, two clocks. PrepareAudio finds the shared events and lays both tracks on top of each other.
• Measures time offset and clock drift to the millisecond
• Shared passages become stereo (one person left, the other right), separate conversations mono
• A timeline like in an editing program: trim, split, join clips, delete, choose stereo or mono, undo
• Preview exactly what the file will be; mute or solo a transmitter
• Inputs: WAV as well as MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG, for example from a phone

3 · MASTER
Equally loud, clearly audible files to pass on, transcribe or publish.
• Loudness according to EBU R128 (ITU-R BS.1770-4) with true peak and loudness range
• Fixed gain to −16 LUFS, limiter at −1.5 dBTP, no compression: the dynamics of speech remain
• MP3 at 192 kbit/s; the finished MP3 is measured again and readjusted if needed
• Honest progress: the percentage only counts finished work

YOUR ORIGINALS STAY UNTOUCHED
The app never changes or overwrites a source file. Results go into the folders tracks, sync and master in the place you choose. What already exists is recognised and skipped.

SOURCE PROTECTION: NO DATA LEAVES THE COMPUTER
If you record conversations, you owe the people involved confidentiality. PrepareAudio collects no data, has no telemetry, needs no microphone and downloads nothing. The app runs in the macOS App Sandbox.

GOES WITH RESEARCHTRANSCRIPT
The mastered files are the ideal starting point for transcription with ResearchTranscript, also by B/IAS.

OPEN SOURCE
PrepareAudio is free software under the AGPL. The source code is public on GitHub and open to inspection. Developed at B/IAS – Basel Institut für angewandte Stadtforschung.

REQUIREMENTS
Mac with Apple silicon, macOS 12 or later. Interface in German, English, French and Italian.""",
"neu": "First version in the Mac App Store.",
},
"fr": {
"portal": "Französisch",
"name": "PrepareAudio",
"untertitel": "Préparer l’audio d’entretien",
"werbetext": "Des fragments d’enregistreur à une conversation propre : assemble sans perte, synchronise deux micros, masterise en MP3. Tout reste sur ton Mac.",
"schlagwoerter": "audio,enregistreur,WAV,assembler,fusionner,synchroniser,mastering,MP3,LUFS,entretien,podcast",
"beschreibung": """PrepareAudio transforme ce que livrent les enregistreurs audio et les micros-cravates en enregistrements finis : assembler les fragments, synchroniser deux micros, masteriser en MP3. Trois étapes, chacune utilisable seule. Tout s’exécute localement sur ton Mac : pas de cloud, pas de compte, aucune donnée ne quitte l’ordinateur.

1 · ASSEMBLER
Beaucoup d’enregistreurs découpent les longs enregistrements en fragments. PrepareAudio les réunit.
• Glisse un dossier ; tous les sous-dossiers sont parcourus, quel que soit leur classement
• Détection par l’heure d’enregistrement dans les métadonnées (Broadcast WAV, iXML), le nom ou la date du fichier
• Avec deux émetteurs et des noms de fichiers identiques, c’est l’audio à la jointure qui décide
• Sans perte : les données audio sont copiées bit à bit, automatiquement en RF64 à partir de 4 Go
• Les chaînes incertaines sont signalées et ne sont pas présélectionnées

2 · SYNCHRONISER
Deux personnes, deux micros-cravates, deux horloges. PrepareAudio trouve les événements communs et superpose les deux pistes.
• Mesure le décalage et la dérive d’horloge à la milliseconde
• Les passages communs deviennent stéréo (une personne à gauche, l’autre à droite), les conversations séparées mono
• Une timeline comme dans un logiciel de montage : rogner, couper, joindre, supprimer, choisir stéréo ou mono, annuler
• Pré-écoute exactement telle que le fichier sera produit ; émetteur muet ou solo
• Entrées : WAV ainsi que MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG, par exemple depuis un téléphone

3 · MASTERISER
Des fichiers de même niveau, bien audibles, à transmettre, transcrire ou publier.
• Sonie selon EBU R128 (ITU-R BS.1770-4) avec crête réelle et plage de sonie
• Gain fixe vers −16 LUFS, limiteur à −1,5 dBTP, aucune compression : la dynamique de la parole reste
• MP3 à 192 kbit/s ; le MP3 fini est remesuré et réajusté si nécessaire
• Une progression honnête : le pourcentage ne compte que le travail terminé

TES ORIGINAUX RESTENT INTACTS
L’app ne modifie ni n’écrase jamais un fichier source. Les résultats vont dans les dossiers tracks, sync et master à l’endroit que tu choisis. Ce qui existe déjà est reconnu et ignoré.

PROTECTION DES SOURCES : AUCUNE DONNÉE NE QUITTE L’ORDINATEUR
Quand tu enregistres des conversations, tu dois la confidentialité aux personnes concernées. PrepareAudio ne collecte aucune donnée, n’a pas de télémétrie, n’a pas besoin du micro et ne télécharge rien. L’app s’exécute dans le bac à sable de macOS.

VA AVEC RESEARCHTRANSCRIPT
Les fichiers masterisés sont le point de départ idéal pour la transcription avec ResearchTranscript, également du B/IAS.

OPEN SOURCE
PrepareAudio est un logiciel libre sous AGPL. Le code source est public sur GitHub et peut être vérifié. Développé au B/IAS – Basel Institut für angewandte Stadtforschung.

CONFIGURATION REQUISE
Mac avec puce Apple, macOS 12 ou ultérieur. Interface en allemand, anglais, français et italien.""",
"neu": "Première version sur le Mac App Store.",
},
"it": {
"portal": "Italienisch",
"name": "PrepareAudio",
"untertitel": "Prepara l’audio d’intervista",
"werbetext": "Dai frammenti del registratore a una conversazione pulita: unisci senza perdita, sincronizza due microfoni, masterizza in MP3. Tutto resta sul tuo Mac.",
"schlagwoerter": "audio,registratore,WAV,unire,sincronizzare,mastering,MP3,LUFS,intervista,podcast,loudness",
"beschreibung": """PrepareAudio trasforma ciò che consegnano registratori audio e microfoni lavalier in registrazioni finite: unire le parti, sincronizzare due microfoni, masterizzare in MP3. Tre passaggi, ciascuno utilizzabile da solo. Tutto avviene in locale sul tuo Mac: niente cloud, niente account, nessun dato lascia il computer.

1 · UNIRE
Molti registratori dividono le registrazioni lunghe in parti. PrepareAudio le ricompone.
• Trascina una cartella; vengono esaminate tutte le sottocartelle, comunque siano ordinate
• Riconoscimento tramite l’ora di registrazione nei metadati (Broadcast WAV, iXML), il nome o la data del file
• Con due trasmettitori e nomi di file identici decide l’audio nel punto di giunzione
• Senza perdita: i dati audio sono copiati bit per bit, da 4 GB automaticamente come RF64
• Le catene incerte sono segnalate e non preselezionate

2 · SINCRONIZZARE
Due persone, due microfoni lavalier, due orologi. PrepareAudio trova gli eventi comuni e sovrappone le due tracce.
• Misura lo scarto temporale e la deriva dell’orologio al millisecondo
• I passaggi comuni diventano stereo (una persona a sinistra, l’altra a destra), le conversazioni separate mono
• Una timeline come in un programma di montaggio: rifilare, dividere, unire, eliminare, scegliere stereo o mono, annullare
• Preascolto esattamente come sarà il file; trasmettitore muto o solo
• Ingressi: WAV e anche MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG, ad esempio dal telefono

3 · MASTERIZZARE
File di pari volume, ben udibili, da trasmettere, trascrivere o pubblicare.
• Loudness secondo EBU R128 (ITU-R BS.1770-4) con true peak e gamma di loudness
• Guadagno fisso a −16 LUFS, limiter a −1,5 dBTP, nessuna compressione: la dinamica del parlato resta
• MP3 a 192 kbit/s; l’MP3 finito viene rimisurato e, se serve, corretto
• Avanzamento onesto: la percentuale conta solo il lavoro concluso

I TUOI ORIGINALI RESTANO INTATTI
L’app non modifica né sovrascrive mai un file sorgente. I risultati vanno nelle cartelle tracks, sync e master nel luogo che scegli. Ciò che esiste già viene riconosciuto e saltato.

TUTELA DELLE FONTI: NESSUN DATO LASCIA IL COMPUTER
Se registri conversazioni, devi riservatezza alle persone coinvolte. PrepareAudio non raccoglie dati, non ha telemetria, non ha bisogno del microfono e non scarica nulla. L’app funziona nella sandbox di macOS.

SI ABBINA A RESEARCHTRANSCRIPT
I file masterizzati sono il punto di partenza ideale per la trascrizione con ResearchTranscript, anch’esso del B/IAS.

OPEN SOURCE
PrepareAudio è software libero sotto AGPL. Il codice sorgente è pubblico su GitHub e verificabile. Sviluppato al B/IAS – Basel Institut für angewandte Stadtforschung.

REQUISITI
Mac con chip Apple, macOS 12 o successivo. Interfaccia in tedesco, inglese, francese e italiano.""",
"neu": "Prima versione sul Mac App Store.",
},
}

PRUEFNOTIZ = """PrepareAudio works fully offline: no account, no login, no server, no telemetry. Nothing needs to be configured.

The network-client entitlement is present only because WebKit requires it to render the app's own bundled pages inside the App Sandbox; the app opens no network connections. No microphone access is requested: the app only plays back.

The app prepares audio from field recorders. Because it needs recordings to show anything, we provide invented demo material (synthetic voices, no real persons), 64 MB:
https://bias.city/prepareaudio/demo/prepareaudio-demo.zip

How to test (about 3 minutes):
1. Unzip. Tab "1 Merge": drag the folder "aufnahmen" into the window. Two recordings of three parts each appear. Click "Merge…", choose any folder; the app creates "tracks" there.
2. Tab "2 Synchronise": drag the folder "aufnahmen" (or the new "tracks") in. After the analysis a timeline shows stereo and mono clips. Space bar plays. Click "Create…" and choose a folder; the app creates "sync".
3. Tab "3 Master": drag the folder "mastern" in. Click "Master…"; the app writes MP3 files at −16 LUFS into "master".
The pill "Info" (bottom left) shows licences and the language switch (German, English, French, Italian).

The source code is public (AGPL-3.0-or-later with an additional permission under section 7 for App Store distribution): https://github.com/bias-city/PrepareAudio
LAME (LGPL) is linked dynamically; its source is inside the bundle and at https://bias.city/prepareaudio/quellen/"""


def main() -> None:
    fehler = []
    for lang, felder in T.items():
        ziel = ROOT / "appstore" / "texte" / lang
        ziel.mkdir(parents=True, exist_ok=True)
        for feld, grenze in LIMIT.items():
            text = felder[feld]
            if len(text) > grenze:
                fehler.append(f"{lang}/{feld}: {len(text)} > {grenze}")
            (ziel / f"{feld}.txt").write_text(text + "\n", encoding="utf-8")
        print(f"{lang}: Untertitel {len(felder['untertitel'])}/30 · Werbetext {len(felder['werbetext'])}/170 · "
              f"Schlagwörter {len(felder['schlagwoerter'])}/100 · Beschreibung {len(felder['beschreibung'])}/4000")
    gemeinsam = ROOT / "appstore" / "texte"
    (gemeinsam / "pruefnotiz-en.txt").write_text(PRUEFNOTIZ + "\n", encoding="utf-8")
    (gemeinsam / "urls.txt").write_text("\n".join(f"{k}: {v}" for k, v in URLS.items()) + f"\ncopyright: {COPYRIGHT}\n", encoding="utf-8")
    alles = " ".join(" ".join(f.values()) for f in T.values()) + PRUEFNOTIZ
    for marke in ("DJI", "Zoom H", "Rode", "RØDE", "Tascam", "Sennheiser"):
        if marke.lower() in alles.lower():
            fehler.append(f"Markenname im Text: {marke}")
    for wort in (" Sie ", " Ihr", " vous ", " votre ", " Lei ", " Suo "):
        if wort in alles:
            fehler.append(f"Höflichkeitsform im Text: «{wort.strip()}»")
    if fehler:
        raise SystemExit("FEHLER:\n  " + "\n  ".join(fehler))
    print("✓ alle Längen eingehalten, keine Marken, überall Du")


if __name__ == "__main__":
    main()
