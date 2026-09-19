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
"untertitel": "Gesprächsaufnahmen vorbereiten",
"werbetext": "Aus verstreuten Aufnahmen wird ein Gespräch: Teile verlustfrei zusammenfügen, beliebig viele Mikrofone synchronisieren, als MP3 mastern. Alles lokal auf deinem Mac.",
"schlagwoerter": "Audio,Recorder,WAV,zusammenfügen,synchronisieren,Mastern,MP3,LUFS,Interview,Podcast,Video,offline",
"beschreibung": """PrepareAudio macht aus dem, was Audiorecorder, Ansteckmikrofone, Handys und Kameras abliefern, fertige Aufnahmen: Teile zusammenfügen, beliebig viele Mikrofone synchronisieren, als MP3 mastern. Drei Schritte, jeder für sich nutzbar. Alles läuft lokal auf deinem Mac: keine Cloud, kein Konto, keine Daten verlassen den Rechner.

1 · ZUSAMMENFÜGEN
Viele Recorder schneiden lange Aufnahmen in Teile. PrepareAudio setzt sie wieder zusammen.
• Nimmt WAV in mono, stereo und mehrkanalig, dazu MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG Vorbis und den Ton aus MP4- und MOV-Videos
• Ordner hineinziehen oder mehrere auf einmal wählen; alle Unterordner werden durchsucht, egal wie sortiert
• Erkennung über Aufnahmezeit in den Metadaten (Broadcast-WAV, iXML), Dateiname oder Dateidatum
• Bei zwei Sendern mit gleichen Dateinamen entscheidet das Audio an der Nahtstelle
• Ausgabe immer WAV, aus WAV-Quellen bitgenau kopiert, ab 4 GB automatisch als RF64
• Unsichere Verkettungen sind markiert und nicht vorausgewählt

2 · SYNCHRONISIEREN
Mehrere Personen, mehrere Mikrofone, mehrere Uhren. PrepareAudio findet die gemeinsamen Ereignisse und legt alle Spuren übereinander.
• Misst Zeitversatz und Uhrendrift auf Millisekunden, schon ab einer Minute Überlappung
• Gemeinsame Abschnitte werden eine Datei mit einem Kanal je Sender: bei zweien links und rechts, bei mehreren eine polyphone WAV mit Timecode und Spurnamen
• Getrennte Gespräche werden Mono-Dateien
• Timeline wie im Schnittprogramm: eine Bahn je Sender, trimmen, trennen, zusammenführen, löschen, rückgängig machen
• Ein Klick auf einen Abschnitt setzt sein Ziel und seine Position im späteren Stereo-Mixdown: links, Mitte oder rechts
• Zeit ohne Aufnahme fällt aus der Timeline; eine Sessiongrenze nennt die Dauer der Lücke
• Vorhören genau so, wie die Datei entsteht; Sender stumm oder solo

3 · MASTERN
Gleich laute, gut hörbare Dateien zum Weitergeben, Transkribieren oder Veröffentlichen.
• Lautheit nach EBU R128 (ITU-R BS.1770-4) mit True Peak und Lautheitsumfang
• Profil «Für Transkription» (Standard): feste Verstärkung auf −16 LUFS, kein Eingriff in die Dynamik — an echten Interviews geprüft, Spracherkennung und Sprechertrennung arbeiten damit besser
• Profil «Fürs Hören»: gleicht zusätzlich die Lautstärken der Sprechenden aus, hebt leise Passagen an und senkt Übersprechen
• Mehrkanalige Dateien werden in beiden Profilen nach L/M/R auf Stereo gemischt
• Limiter bei −1,5 dBTP, MP3 mit 192 kbit/s; das fertige MP3 wird nachgemessen und bei Bedarf neu kodiert

HANDBUCH IN DER APP
Neun Kapitel mit Suche und Bildern, in allen vier Sprachen: die drei Schritte, die Timeline, alle Tastenkürzel und was die App lokal speichert.

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
"untertitel": "Prepare conversation audio",
"werbetext": "Scattered recordings become one conversation: merge the parts losslessly, synchronise any number of microphones, master as MP3. All locally on your Mac.",
"schlagwoerter": "audio,recorder,WAV,merge,synchronise,mastering,MP3,LUFS,interview,podcast,video,offline",
"beschreibung": """PrepareAudio turns what audio recorders, lavalier microphones, phones and cameras deliver into finished recordings: merge the parts, synchronise any number of microphones, master as MP3. Three steps, each useful on its own. Everything runs locally on your Mac: no cloud, no account, no data leaves the computer.

1 · MERGE
Many recorders cut long recordings into parts. PrepareAudio puts them back together.
• Takes WAV in mono, stereo and multichannel, plus MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG Vorbis and the sound from MP4 and MOV videos
• Drag a folder in or choose several at once; all subfolders are searched, however they are sorted
• Detection by recording time in the metadata (Broadcast WAV, iXML), file name or file date
• With two transmitters and identical file names, the audio at the seam decides
• Output is always WAV, copied bit for bit from WAV sources, from 4 GB automatically as RF64
• Uncertain chains are marked and not preselected

2 · SYNCHRONISE
Several people, several microphones, several clocks. PrepareAudio finds the shared events and lays all the tracks on top of each other.
• Measures time offset and clock drift to the millisecond, from as little as a minute of overlap
• Shared passages become one file with one channel per transmitter: left and right with two, a polyphonic WAV with timecode and track names with more
• Separate conversations become mono files
• A timeline like in an editing program: one lane per transmitter, trim, split, join, delete, undo
• A click on a segment sets its destination and its position in the later stereo mixdown: left, middle or right
• Time without a recording drops out of the timeline; a session boundary names the length of the gap
• Preview exactly as the file will turn out; mute or solo a transmitter

3 · MASTER
Equally loud, clearly audible files to pass on, transcribe or publish.
• Loudness according to EBU R128 (ITU-R BS.1770-4) with True Peak and loudness range
• Profile “For transcription” (default): fixed gain to −16 LUFS, no intervention in the dynamics — tested on real interviews, speech recognition and speaker separation work better on it
• Profile “For listening”: additionally evens out the levels of the speakers, lifts quiet passages and turns down bleed
• Multichannel files are mixed down to stereo by L/M/R in both profiles
• Limiter at −1.5 dBTP, MP3 at 192 kbit/s; the finished MP3 is measured again and re-encoded if needed

MANUAL IN THE APP
Nine chapters with search and images, in all four languages: the three steps, the timeline, all keyboard shortcuts and what the app stores locally.

YOUR ORIGINALS STAY UNTOUCHED
The app never changes or overwrites a source file. Results go into the folders tracks, sync and master in the place you choose. What already exists is recognised and skipped.

SOURCE PROTECTION: NO DATA LEAVES THE COMPUTER
If you record conversations, you owe the people involved confidentiality. PrepareAudio collects no data, has no telemetry, needs no microphone and downloads nothing. The app runs in the App Sandbox of macOS.

GOES WITH RESEARCHTRANSCRIPT
The mastered files are the ideal starting point for transcription with ResearchTranscript, also by B/IAS.

OPEN SOURCE
PrepareAudio is free software under the AGPL. The source code is public on GitHub and open to inspection. Developed at B/IAS – Basel Institut für angewandte Stadtforschung.

REQUIREMENTS
Mac with Apple Silicon, macOS 12 or later. Interface in German, English, French and Italian.""",
"neu": "First version in the Mac App Store.",
},
"fr": {
"portal": "Französisch",
"name": "PrepareAudio",
"untertitel": "Préparer tes enregistrements",
"werbetext": "Des enregistrements épars deviennent une conversation : assembler sans perte, synchroniser autant de micros que tu veux, masteriser en MP3. Tout en local sur ton Mac.",
"schlagwoerter": "audio,enregistreur,WAV,assembler,synchroniser,mastering,MP3,LUFS,entretien,podcast,vidéo",
"beschreibung": """PrepareAudio transforme ce que livrent enregistreurs audio, micros-cravates, téléphones et caméras en enregistrements finis : assembler les fragments, synchroniser autant de micros que tu veux, masteriser en MP3. Trois étapes, chacune utilisable seule. Tout s’exécute localement sur ton Mac : pas de cloud, pas de compte, aucune donnée ne quitte l’ordinateur.

1 · ASSEMBLER
Beaucoup d’enregistreurs découpent les longs enregistrements en fragments. PrepareAudio les réunit.
• Accepte le WAV en mono, en stéréo et en multicanal, ainsi que MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG Vorbis et le son des vidéos MP4 et MOV
• Glisse un dossier ou choisis-en plusieurs d’un coup ; tous les sous-dossiers sont parcourus, quel que soit leur classement
• Détection par l’heure d’enregistrement dans les métadonnées (Broadcast WAV, iXML), le nom ou la date du fichier
• Avec deux émetteurs et des noms de fichiers identiques, c’est l’audio à la jointure qui décide
• La sortie est toujours du WAV, copié bit à bit depuis les sources WAV, automatiquement en RF64 à partir de 4 Go
• Les chaînes incertaines sont signalées et ne sont pas présélectionnées

2 · SYNCHRONISER
Plusieurs personnes, plusieurs micros, plusieurs horloges. PrepareAudio trouve les événements communs et superpose toutes les pistes.
• Mesure le décalage et la dérive d’horloge à la milliseconde, dès une minute de recouvrement
• Les passages communs deviennent un fichier avec un canal par émetteur : à deux, à gauche et à droite ; à plusieurs, un WAV polyphonique avec timecode et noms de pistes
• Les conversations séparées deviennent des fichiers mono
• Une timeline comme dans un logiciel de montage : une voie par émetteur, rogner, couper, joindre, supprimer, annuler
• Un clic sur un segment fixe sa destination et sa position dans le futur mixage stéréo : gauche, milieu ou droite
• Le temps sans enregistrement sort de la timeline ; une limite de session indique la durée du trou
• Pré-écoute exactement telle que le fichier sera produit ; émetteur muet ou solo

3 · MASTERISER
Des fichiers de même niveau, bien audibles, à transmettre, transcrire ou publier.
• Loudness selon EBU R128 (ITU-R BS.1770-4) avec True Peak et plage de loudness
• Profil « Pour la transcription » (par défaut) : gain fixe à −16 LUFS, aucune intervention sur la dynamique — vérifié sur de vrais entretiens, la reconnaissance vocale et la séparation des locuteurs fonctionnent mieux ainsi
• Profil « Pour l’écoute » : égalise en plus le niveau des personnes qui parlent, relève les passages faibles et atténue la diaphonie
• Les fichiers multicanaux sont mixés en stéréo selon L/M/R dans les deux profils
• Limiteur à −1,5 dBTP, MP3 à 192 kbit/s ; le MP3 fini est remesuré et réencodé si nécessaire

MANUEL DANS L’APP
Neuf chapitres avec recherche et images, dans les quatre langues : les trois étapes, la timeline, tous les raccourcis clavier et ce que l’app enregistre localement.

TES ORIGINAUX RESTENT INTACTS
L’app ne modifie ni n’écrase jamais un fichier source. Les résultats vont dans les dossiers tracks, sync et master à l’endroit que tu choisis. Ce qui existe déjà est reconnu et ignoré.

PROTECTION DES SOURCES : AUCUNE DONNÉE NE QUITTE L’ORDINATEUR
Quand tu enregistres des conversations, tu dois la confidentialité aux personnes concernées. PrepareAudio ne collecte aucune donnée, n’a pas de télémétrie, n’a pas besoin du micro et ne télécharge rien. L’app s’exécute dans l’App Sandbox de macOS.

VA AVEC RESEARCHTRANSCRIPT
Les fichiers masterisés sont le point de départ idéal pour la transcription avec ResearchTranscript, également du B/IAS.

OPEN SOURCE
PrepareAudio est un logiciel libre sous AGPL. Le code source est public sur GitHub et peut être vérifié. Développé au B/IAS – Basel Institut für angewandte Stadtforschung.

CONFIGURATION REQUISE
Mac avec Apple Silicon, macOS 12 ou ultérieur. Interface en allemand, anglais, français et italien.""",
"neu": "Première version sur le Mac App Store.",
},
"it": {
"portal": "Italienisch",
"name": "PrepareAudio",
"untertitel": "Prepara le tue registrazioni",
"werbetext": "Da registrazioni sparse nasce una conversazione: unire le parti senza perdita, sincronizzare quanti microfoni vuoi, masterizzare in MP3. Tutto in locale sul tuo Mac.",
"schlagwoerter": "audio,registratore,WAV,unire,sincronizzare,mastering,MP3,LUFS,intervista,podcast,video",
"beschreibung": """PrepareAudio trasforma ciò che consegnano registratori audio, microfoni lavalier, telefoni e videocamere in registrazioni finite: unire le parti, sincronizzare quanti microfoni vuoi, masterizzare in MP3. Tre passaggi, ciascuno utilizzabile da solo. Tutto avviene in locale sul tuo Mac: niente cloud, niente account, nessun dato lascia il computer.

1 · UNIRE
Molti registratori dividono le registrazioni lunghe in parti. PrepareAudio le ricompone.
• Accetta WAV in mono, stereo e multicanale, oltre a MP3, M4A/AAC, FLAC, ALAC, AIFF, CAF, OGG Vorbis e l’audio dei video MP4 e MOV
• Trascina una cartella o scegline più d’una alla volta; vengono esaminate tutte le sottocartelle, comunque siano ordinate
• Riconoscimento tramite l’ora di registrazione nei metadati (Broadcast WAV, iXML), il nome o la data del file
• Con due trasmettitori e nomi di file identici decide l’audio nel punto di giunzione
• L’uscita è sempre WAV, copiato bit per bit dalle sorgenti WAV, da 4 GB automaticamente come RF64
• Le catene incerte sono segnalate e non preselezionate

2 · SINCRONIZZARE
Più persone, più microfoni, più orologi. PrepareAudio trova gli eventi comuni e sovrappone tutte le tracce.
• Misura lo sfasamento e la deriva dell’orologio al millisecondo, già da un minuto di sovrapposizione
• I tratti comuni diventano un unico file con un canale per trasmettitore: con due a sinistra e a destra, con più trasmettitori un WAV polifonico con timecode e nomi delle tracce
• Le conversazioni separate diventano file mono
• Una timeline come in un programma di montaggio: una corsia per trasmettitore, rifilare, dividere, unire, eliminare, annullare
• Un clic su un segmento fissa la sua destinazione e la sua posizione nel successivo mixdown stereo: sinistra, centro o destra
• Il tempo senza registrazione esce dalla timeline; un limite di sessione indica la durata del vuoto
• Preascolto esattamente come nascerà il file; trasmettitore muto o solo

3 · MASTERIZZARE
File di pari volume, ben udibili, da trasmettere, trascrivere o pubblicare.
• Loudness secondo EBU R128 (ITU-R BS.1770-4) con True Peak e gamma di loudness
• Profilo «Per la trascrizione» (predefinito): guadagno fisso a −16 LUFS, nessun intervento sulla dinamica — verificato su interviste reali, riconoscimento vocale e separazione dei parlanti funzionano meglio così
• Profilo «Per l’ascolto»: pareggia inoltre i livelli di chi parla, alza i passaggi deboli e attenua la diafonia
• I file multicanale vengono miscelati in stereo secondo L/M/R in entrambi i profili
• Limiter a −1,5 dBTP, MP3 a 192 kbit/s; l’MP3 finito viene rimisurato e, se serve, ricodificato

MANUALE NELL’APP
Nove capitoli con ricerca e immagini, in tutte e quattro le lingue: i tre passaggi, la timeline, tutte le scorciatoie da tastiera e che cosa l’app salva in locale.

I TUOI ORIGINALI RESTANO INTATTI
L’app non modifica né sovrascrive mai un file sorgente. I risultati vanno nelle cartelle tracks, sync e master nel luogo che scegli. Ciò che esiste già viene riconosciuto e saltato.

TUTELA DELLE FONTI: NESSUN DATO LASCIA IL COMPUTER
Se registri conversazioni, devi riservatezza alle persone coinvolte. PrepareAudio non raccoglie dati, non ha telemetria, non ha bisogno del microfono e non scarica nulla. L’app funziona nell’App Sandbox di macOS.

SI ABBINA A RESEARCHTRANSCRIPT
I file masterizzati sono il punto di partenza ideale per la trascrizione con ResearchTranscript, anch’esso del B/IAS.

OPEN SOURCE
PrepareAudio è software libero sotto AGPL. Il codice sorgente è pubblico su GitHub e verificabile. Sviluppato al B/IAS – Basel Institut für angewandte Stadtforschung.

REQUISITI
Mac con Apple Silicon, macOS 12 o successivo. Interfaccia in tedesco, inglese, francese e italiano.""",
"neu": "Prima versione sul Mac App Store.",
},
}

PRUEFNOTIZ = """PrepareAudio works fully offline: no account, no login, no server, no telemetry. Nothing needs to be configured.

The network-client entitlement is present only because WebKit requires it to render the app's own bundled pages inside the App Sandbox; the app opens no network connections. No microphone access is requested: the app only plays back.

The app prepares audio from field recordings. Because it needs recordings to show anything, we provide invented demo material (synthetic voices, no real persons), 90 MB:
https://bias.city/prepareaudio/demo/prepareaudio-demo.zip

How to test (about 4 minutes):
1. Unzip. Tab "1 Merge": drag the folder "aufnahmen" into the window. Three recordings of three chunks each appear. Click "Merge…", choose any folder; the app creates "tracks" there.
2. Tab "2 Synchronise": drag the folder "aufnahmen" (or the new "tracks") in. After the analysis a timeline shows one lane per transmitter; space bar plays. Click a segment to open its options (shared file or mono file, and the position left/middle/right). Click "Create…" and choose a folder; the app writes one shared file per conversation — with three transmitters a polyphonic WAV — plus mono files into "sync".
3. Tab "3 Master": drag the folder "mastern" in, pick a profile at the bottom ("For transcription" or "For listening") and click "Master…"; the app writes MP3 files at -16 LUFS into "master".
The pill "Help" at the bottom left opens the built-in handbook (searchable, four languages).

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
