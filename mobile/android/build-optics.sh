#!/usr/bin/env bash
set -euo pipefail
# Compile a freestanding JNI leaf library. It uses no libc, NDK, dynamic imports or heap.
root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
output=${1:?Pass the APK staging directory}
host=$(rustc -vV | sed -n 's/^host: //p')
linker="$(rustc --print sysroot)/lib/rustlib/$host/bin/rust-lld"
[[ -x "$linker" ]]
for pair in aarch64-linux-android:arm64-v8a armv7-linux-androideabi:armeabi-v7a i686-linux-android:x86 x86_64-linux-android:x86_64; do
    target=${pair%:*}
    abi=${pair#*:}
    dir="$output/lib/$abi"
    mkdir -p "$dir"
    rustc --edition=2021 --crate-name inta_optics --target "$target" --crate-type lib --emit=obj \
        -C opt-level=s -C panic=abort -C relocation-model=pic -C strip=symbols \
        "$root/native/optics/src/lib.rs" -o "$output/$abi.o"
    builtins=("$(rustc --target "$target" --print target-libdir)"/libcompiler_builtins-*.rlib)
    [[ ${#builtins[@]} == 1 && -f ${builtins[0]} ]]
    "$linker" -flavor gnu -shared --no-undefined --gc-sections --strip-all -z noexecstack \
        -z relro -z now -z max-page-size=16384 -soname libinta_optics.so \
        "$output/$abi.o" "${builtins[0]}" -o "$dir/libinta_optics.so"
done
