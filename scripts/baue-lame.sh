#!/bin/sh
# libmp3lame für das Bundle bauen (wie in ResearchTranscript, docs/PLAN-APPSTORE.md).
#
# LAME ist LGPL-2.0+: dynamisch gelinkt (Contents/Frameworks), damit die Bibliothek
# ausgetauscht werden kann; der Quell-Tarball liegt daneben, kommt mit ins App-Paket und
# wird zu jedem Release auf bias.city/prepareaudio/quellen/ gehostet.
# Version 3.100 mit Absicht: Limiter und Nachmessung beim Mastern sind darauf kalibriert
# (Test detects_formats_masters_to_target_and_skips_existing vergleicht die MP3-Bytes).
# Ohne Decoder und ohne Frontend: eine einzige dylib, die nur an libSystem hängt.
#
# Aufruf: scripts/baue-lame.sh [tarball]   (ohne Argument: Download)
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
ZIEL="$ROOT/src-tauri/frameworks"
VERSION=3.100
SHA=ddfe36cab873794038ae2c1210557ad34857a4b6bdc515785d1da9e175b1da1e
URL="https://downloads.sourceforge.net/project/lame/lame/$VERSION/lame-$VERSION.tar.gz"
BAU=$(mktemp -d /tmp/lame-bau.XXXXXX)
TAR="${1:-$BAU/lame-$VERSION.tar.gz}"
[ -f "$TAR" ] || curl -sSL -o "$TAR" "$URL"
echo "$SHA  $TAR" | shasum -a 256 -c - >/dev/null || { echo "ABBRUCH: Prüfsumme des Tarballs stimmt nicht"; exit 1; }
tar xf "$TAR" -C "$BAU"
cd "$BAU/lame-$VERSION"
# Exportliste von 3.100: ein Symbol gibt es nicht mehr, und ohne Decoder fehlen die
# hip_*- und lame_decode*-Funktionen — beides bricht sonst den Linker.
sed -i '' -e '/lame_init_old/d' -e '/^hip_/d' -e '/^lame_decode/d' include/libmp3lame.sym
export MACOSX_DEPLOYMENT_TARGET=12.0
./configure --prefix="$BAU/out" --enable-shared --disable-static \
  --disable-decoder --disable-frontend --disable-gtktest >"$BAU/configure.log" 2>&1
make -j8 >"$BAU/make.log" 2>&1 || { echo "ABBRUCH: make (siehe $BAU/make.log)"; exit 1; }
make install >"$BAU/install.log" 2>&1
mkdir -p "$ZIEL"
cp "$BAU/out/lib/libmp3lame.0.dylib" "$ZIEL/libmp3lame.dylib"
cp "$TAR" "$ZIEL/lame-$VERSION.tar.gz"
# Install-Name auf @rpath — die App trägt @executable_path/../Frameworks
install_name_tool -id @rpath/libmp3lame.dylib "$ZIEL/libmp3lame.dylib"
otool -L "$ZIEL/libmp3lame.dylib" | sed -n 2,4p
echo "fertig: $ZIEL/libmp3lame.dylib ($(stat -f %z "$ZIEL/libmp3lame.dylib") Bytes), Quelle daneben"
rm -rf "$BAU"
