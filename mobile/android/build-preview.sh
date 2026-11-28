#!/usr/bin/env bash
set -euo pipefail
# Build with existing official tools. No downloads, install, release signing or device access.
if (( $# != 3 )); then
  printf '%s\n' 'Usage: bash build-preview.sh ANDROID_JAR BUILD_TOOLS NEW_OUTPUT_DIRECTORY' >&2
  exit 2
fi
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
jar=$(realpath -- "$1")
tools=$(realpath -- "$2")
mkdir -- "$3"
out=$(cd -- "$3" && pwd)
scratch=$(mktemp -d "$out/.build.XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT
mkdir "$scratch"/{classes,dex,test,fixture}
brand="$root/../../assets/inta-brand"
mkdir -p "$scratch"/{res/mipmap-xxxhdpi,gen}
install -m 644 "$brand/application-192.png" "$scratch/res/mipmap-xxxhdpi/inta_application.png"
"$tools/aapt2" compile --dir "$scratch/res" -o "$scratch/resources.zip"
"$tools/aapt2" link --manifest "$root/AndroidManifest.xml" -I "$jar" --java "$scratch/gen" -o "$scratch/base.apk" "$scratch/resources.zip"
javac --release 11 -Xlint:all -Werror -classpath "$jar" -d "$scratch/classes" "$scratch/gen/com/inta/mobile/R.java" "$root"/src/com/inta/mobile/*.java
javac --release 11 -Xlint:all -Werror -d "$scratch/test" \
  "$root"/src/com/inta/mobile/{JobTimer,JobStore,JobBook,RepairDraft,InvoiceDraft,JobHandoff}.java "$root"/tests/com/inta/mobile/JobStateTest.java "$root"/tests/com/inta/mobile/JobBookTest.java
java -cp "$scratch/test" com.inta.mobile.JobStateTest "$scratch/fixture"
java -cp "$scratch/test" com.inta.mobile.JobBookTest "$scratch/fixture-book"
bash "$root/build-optics.sh" "$scratch"
(cd -- "$scratch"; export TZ=UTC; touch -t 198001010000 lib/*/libinta_optics.so; zip -q -0 -X base.apk lib/*/libinta_optics.so)
mapfile -d '' -t classes < <(find "$scratch/classes" -name '*.class' -print0 | sort -z)
"$tools/d8" --release --min-api 26 --lib "$jar" --output "$scratch/dex" "${classes[@]}"
(cd -- "$scratch/dex"; export TZ=UTC; touch -t 198001010000 classes.dex; zip -q -X -j ../base.apk classes.dex)
"$tools/zipalign" -f -P 16 4 "$scratch/base.apk" "$out/inta-mobile-unsigned.apk"
"$tools/zipalign" -c -P 16 4 "$out/inta-mobile-unsigned.apk"
"$tools/aapt2" dump badging "$out/inta-mobile-unsigned.apk"
sha256sum "$out/inta-mobile-unsigned.apk"
