[Home](https://github.com/Herbertofury/INTA/wiki) · [All playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Interactive checklist](https://herbertofury.github.io/INTA/#job/printer-repair)

# Restore printing without breaking other queues

> Isolate printer hardware, network reachability, queue state, driver, and application behavior.

| Job profile | Details |
| --- | --- |
| Category | Devices & peripherals |
| Working time | 15–45 min |
| Experience | Starter |

## Start here

Identify the exact printer and queue. Check whether another user or application can print to the same device.

## Your toolkit

| Tool | Why it belongs here |
| --- | --- |
| [Microsoft Sysinternals Suite](https://learn.microsoft.com/en-us/sysinternals/downloads/sysinternals-suite) | The core portable Windows troubleshooting kit. |

## Follow the workflow

### 01. Check the printer itself

- [ ] Inspect its display, paper path, consumables, and connection. Print the device’s internal test or configuration page when available. If that fails, work on the hardware state before changing Windows drivers.

### 02. Verify the connection and queue

- [ ] Confirm USB connection or the printer’s current network address against the queue’s configured port. Check that the intended queue is selected, not an old duplicate or offline copy. A changed DHCP address can leave a working printer behind an obsolete port.

### 03. Separate application and queue faults

- [ ] Print a Windows test page and a simple document from another application. If only one application fails, troubleshoot its print settings or installation. If all jobs fail, inspect queue status and run the Windows printer troubleshooter.

### 04. Clear the specific obstruction

- [ ] Cancel only the affected user’s stuck job when possible. Coordinate before restarting the Print Spooler because that affects other queues. On a shared print server, identify all affected users and obtain a service window before clearing queues or replacing a shared driver.

### 05. Use the supported driver path

- [ ] Install or repair the manufacturer-supported driver for the exact model and Windows architecture, or the supported Windows class driver where appropriate. Preserve existing ports and options; document any queue replacement so duplex, trays, and scanning are not lost.

### 06. Verify real output

- [ ] Print the original document and check page size, orientation, color, duplex, and selected tray. Test another user on shared printers, and verify scanning separately if the device is multifunction. Remove obsolete duplicate queues only after the working one is confirmed.

## Verify before handoff

- [ ] The printer’s internal page and Windows/application test output are correct.
- [ ] Required trays, duplex, color, and scanning still work.
- [ ] Other users and shared queues remain functional.

## Stop and escalate when

- The device has a hardware fault or unsafe condition.
- A shared spooler/driver change would disrupt uncoordinated production jobs.

## Related playbooks

- [Network Dns](network-dns.md)
- [New Pc](new-pc.md)
- [Client Handoff](client-handoff.md)

## Official references

- [Microsoft: printer connection and printing problems](https://support.microsoft.com/en-us/windows/hardware/printer/fix-printer-connection-and-printing-problems-in-windows)
- [Microsoft: print jobs stuck in the queue](https://support.microsoft.com/en-us/windows/hardware/printer/fix-print-job-stuck-in-queue-errors-in-windows)

---

INTA Computer Services · Editorial review: 2026-11-26. Timing is an estimate; stop conditions take priority.
