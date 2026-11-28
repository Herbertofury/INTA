#!/bin/bash
set -euo pipefail
exec >/dev/ttyS0 2>&1
trap 'echo INTA_REPAIR_EXIT=$?' EXIT
source /run/inta-lab/fixture.env
udevadm settle
# A loop-mounted clone carries the source label. Resolve both devices before
# the clone exists so udev cannot redirect verification to the repaired copy.
source_disk=$(realpath -e /dev/disk/by-label/INTASRC)
target_disk=$(realpath -e /dev/disk/by-label/INTAOUT)
[[ "$(blockdev --getro "$source_disk")" == 1 ]]
[[ "$(blockdev --getsize64 "$source_disk")" == 33554432 ]]
[[ "$(blockdev --getsize64 "$target_disk")" == 100663296 ]]
[[ "$source_disk" != "$target_disk" ]]
mkdir -p /run/inta-output /run/inta-recovered
mount -o nodev,nosuid,noexec "$target_disk" /run/inta-output
cd /run/inta-output
[[ "$(cat QA.ID)" == "$FIXTURE_ID" ]]
[[ ! -e recovered.img && ! -e rescue.map ]]
printf '%s  %s\n' "$SOURCE_HASH" "$source_disk" | sha256sum -c -

ddrescue --quiet --no-scrape "$source_disk" recovered.img rescue.map
printf '%s  recovered.img\n' "$SOURCE_HASH" | sha256sum -c -
if fsck.fat -n recovered.img; then
    echo 'The expected filesystem defect was absent.'
    exit 1
fi
status=0
fsck.fat -a recovered.img || status=$?
[[ "$status" -le 1 ]]
fsck.fat -n recovered.img
mount -o loop,ro,nodev,nosuid,noexec recovered.img /run/inta-recovered
printf '%s  /run/inta-recovered/RECOVER.TXT\n' "$FILE_HASH" | sha256sum -c -
umount /run/inta-recovered
printf '%s  %s\n' "$SOURCE_HASH" "$source_disk" | sha256sum -c -
[[ -s rescue.map ]]
printf 'INTA_RECOVERED_SHA256=%s\n' "$FILE_HASH"
sync
cd /
umount /run/inta-output
echo INTA_REPAIR_PASS
