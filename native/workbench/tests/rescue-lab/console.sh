#!/bin/bash
if [[ $- == *i* ]]; then
    export PS1='inta-rescue-lab# '
    export TERM=vt100
    bind 'set enable-bracketed-paste off'
    unset PROMPT_COMMAND HISTFILE
    printf '\nINTA_CONSOLE_READY\n'
    printf 'Offline INTA Rescue. /dev/vdc is your private working disk. Type exit to shut down.\n'
    return
fi
set -euo pipefail
test "$(id -u)" = 0
[[ "$(blockdev --getro /dev/vda)" == 1 && "$(blockdev --getro /dev/vdb)" == 1 ]]
[[ "$(blockdev --getro /dev/vdc)" == 0 ]]
exec setsid --ctty --wait /bin/bash --noprofile --rcfile /run/inta-control/console.sh -i \
    </dev/ttyS0 >/dev/ttyS0 2>&1
