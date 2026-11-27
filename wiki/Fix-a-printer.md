[INTA](https://github.com/Herbertofury/INTA/wiki) / [Decision desk](https://github.com/Herbertofury/INTA/wiki/Decision-Desk) · [Service map](https://github.com/Herbertofury/INTA/wiki/Service-Map) · [Playbooks](https://github.com/Herbertofury/INTA/wiki/Playbooks) · [Tools](https://github.com/Herbertofury/INTA/wiki/Tool-Directory) · [Methods](https://github.com/Herbertofury/INTA/wiki/Methods) · [Open hub](https://herbertofury.github.io/INTA/)

# 🖨️ Fix a printer

**Check that the intended printer is powered on, ready and connected before changing its queue or driver.**

<a id="working-kit"></a>

## 🧰 Tools and links for this job

**[Open working view](https://herbertofury.github.io/INTA/#job/printer-repair/quick/fix-a-printer)** · Keep the tool panel beside the instructions.

**First move:** Check that the intended printer is powered on, ready and connected before changing its queue or driver.

**Driver shortcuts:** **[HP printer drivers](https://support.hp.com/us-en/drivers/printers)** · **[Brother printer drivers](https://www.brother-usa.com/support/drivers)** · **[Canon printer drivers](https://www.usa.canon.com/support/software-and-drivers)** · **[Epson printer support](https://support.epson.com/)** · **[Xerox printer drivers](https://www.support.xerox.com/en-us)** · **[Ricoh drivers and downloads](https://www.ricoh-usa.com/en/support-and-download)** · **[Lexmark printer drivers](https://support.lexmark.com/en_us/drivers-downloads.html)** · **[Kyocera download center](https://www.kyoceradocumentsolutions.us/en/support/downloads.html)** · **[Konica Minolta model search](https://onyxweb.mykonicaminolta.com/OneStopProductSupport?appMode=Public)** · **[Sharp product downloads](https://business.sharpusa.com/product-downloads)** · **[Zebra printer support](https://www.zebra.com/us/en/support-downloads/printers.html)**

| Use this for | Official tool + exact action |
| --- | --- |
| Fix a printer | **[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · [Official page](https://support.microsoft.com/en-us/support/get-help/about-get-help) · **[Start the printer troubleshooter](https://aka.ms/PrinterConnection)** |
| Find the printer’s current address within an approved local subnet. | **[Advanced IP Scanner](https://www.advanced-ip-scanner.com/)** · [Official page](https://www.advanced-ip-scanner.com/) · **[Download Windows scanner · EXE](https://download.advanced-ip-scanner.com/download/files/Advanced_IP_Scanner_2.5.4594.1.exe)** |
| Read the exact installed queue, driver and port. | **[PowerShell](https://learn.microsoft.com/en-us/powershell/)** · [Official page](https://learn.microsoft.com/en-us/powershell/) · **[Use Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell)** |
| Test scanner, feeder and multipage PDF output. | **[NAPS2](https://www.naps2.com/)** · [Official page](https://www.naps2.com/) · **[Download Windows x64 portable · ZIP](https://github.com/cyanfish/naps2/releases/download/v8.6.1/naps2-8.6.1-win-x64.zip)** |
| Identifying the actual USB port, hub, device and negotiated connection when a printer, scanner or peripheral disconnects or runs slowly. | **[USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html)** · [Official page](https://www.uwe-sieber.de/usbtreeview_e.html) · **[Download Windows x64 portable · ZIP](https://www.uwe-sieber.de/files/UsbTreeView_x64.zip)** |
| Testing the same known-good PDF through an independent renderer to separate an application-specific print fault from the queue or printer. | **[SumatraPDF](https://www.sumatrapdfreader.org/)** · [Official page](https://www.sumatrapdfreader.org/) · **[Download Windows x64 portable · ZIP](https://www.sumatrapdfreader.org/dl/rel/3.6.1/SumatraPDF-3.6.1-64.zip)** |

### Before driver changes

| Open the exact resource | What to do there |
| --- | --- |
| **[Windows printer-driver selection](https://support.microsoft.com/en-us/windows/hardware/printer/download-and-install-the-latest-printer-drivers)** | Check [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/), exact model, operating system and architecture. |
| **[Windows protected print mode](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode)** | [WPP](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) uses the modern print stack and removes or blocks third-party drivers. |

### Find the printer

| Open the exact resource | What to do there |
| --- | --- |
| **[Read installed printer ports](https://learn.microsoft.com/en-us/powershell/module/printmanagement/get-printerport)** | Compare the configured port/address with the printer’s network report. |
| **[Read installed queues and drivers](https://learn.microsoft.com/en-us/powershell/module/printmanagement/get-printer)** | Identify the exact queue, port and driver before changing it. |

### Printer drivers by manufacturer

| Open the exact resource | What to do there |
| --- | --- |
| **[HP printer drivers](https://support.hp.com/us-en/drivers/printers)** | Identify the exact printer and supported package for this PC. |
| **[Brother printer drivers](https://www.brother-usa.com/support/drivers)** | Search the model, then select its supported operating system. |
| **[Canon printer drivers](https://www.usa.canon.com/support/software-and-drivers)** | Match the exact model and driver/software category. |
| **[Epson printer support](https://support.epson.com/)** | Find the printer model in the current support search. |
| **[Xerox printer drivers](https://www.support.xerox.com/en-us)** | Search the device and review its OS-specific downloads. |
| **[Ricoh drivers and downloads](https://www.ricoh-usa.com/en/support-and-download)** | Choose the exact device before selecting a supported driver. |
| **[Lexmark printer drivers](https://support.lexmark.com/en_us/drivers-downloads.html)** | Review the model and supported print/scan options. |
| **[Kyocera download center](https://www.kyoceradocumentsolutions.us/en/support/downloads.html)** | Locate the model and match its supported [Windows](https://www.microsoft.com/windows) package. |
| **[Konica Minolta model search](https://onyxweb.mykonicaminolta.com/OneStopProductSupport?appMode=Public)** | Search the device in the publisher-linked driver portal. |
| **[Sharp product downloads](https://business.sharpusa.com/product-downloads)** | Choose the product and required driver or utility. |
| **[Zebra printer support](https://www.zebra.com/us/en/support-downloads/printers.html)** | Select the printer model before downloading its setup tools. |

### Scan verification

| Open the exact resource | What to do there |
| --- | --- |
| **[NAPS2 scanning instructions](https://www.naps2.com/windows-scanning)** | Create a profile; use supported WIA/TWAIN drivers or an eSCL-capable network scanner. |

## ⚡ First three actions

1. Check paper, ink or toner, displayed errors and the selected printer; compare a device test page if available.
2. Run the printer troubleshooter in [Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help) on the affected PC and review the exact queue or connection finding.
3. Apply the indicated repair, then print a test page and the user’s original document.

**[Windows Get Help](https://support.microsoft.com/en-us/support/get-help/about-get-help)** · **[Start the printer troubleshooter](https://aka.ms/PrinterConnection)**

**[Complete procedure](https://github.com/Herbertofury/INTA/wiki/Job-printer-repair)** · **[Specific fixes for this job](https://herbertofury.github.io/INTA/#job/printer-repair/quick/fix-a-printer?section=fixes)** · **[Selection evidence](https://github.com/Herbertofury/INTA/wiki/Free-Tool-Picks)**

<a id="next-action"></a>

## 🧭 Match the symptom

| You see this | Do this next | You should get |
| --- | --- | --- |
| **The printer cannot print its own internal page** | Resolve paper, consumables, displayed errors and hardware state before changing the PC. | The device produces its own clean test/configuration page. |
| **The printer IP is unknown** | Print the printer’s network report or check its approved DHCP record. If still needed, scan only the authorized local subnet with [Advanced IP Scanner](https://www.advanced-ip-scanner.com/) and match the device using its physical model and MAC address. | The address is tied to the intended printer, not merely an open port. |
| **The queue is offline or targets an old address** | Run Get-Printer \| Format-Table Name,PortName,DriverName and Get-PrinterPort in [Windows PowerShell 5.1](https://learn.microsoft.com/en-us/powershell/scripting/windows-powershell/starting-windows-powershell); compare the target with the network report. | The selected queue uses the correct live device and connection. |
| **A driver reinstall or vendor package is being considered** | Check Settings > Bluetooth & devices > Printers & scanners for [Windows protected print mode](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) and managed policy; use supported IPP/Mopria and a vendor [Print Support App](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-mode-faq) when [WPP](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode) applies. | The chosen print path is compatible with the active security mode; a legacy package is not forced onto [WPP](https://learn.microsoft.com/en-us/windows/modern-print/windows-protected-print-mode/windows-protected-print-mode). |
| **A conventional driver is actually required** | Use [Windows Update](https://learn.microsoft.com/en-us/windows/deployment/update/) or the exact manufacturer link above; match model, [Windows](https://www.microsoft.com/windows) version and x64/ARM architecture before installation. | The supported driver installs for this device and retains required trays, duplex and color options. |
| **One job or application fails** | Cancel only the affected stuck job when appropriate, test a [Windows](https://www.microsoft.com/windows) test page and print from another app before changing a shared queue. | The failing job/application is distinguished from a device-wide or queue-wide fault. |
| **Printing works but scanning fails** | In [NAPS2](https://www.naps2.com/) choose New Profile > Choose device. Select a supported WIA/TWAIN device, or eSCL for a compatible network scanner; scan a page and save/open the PDF. Test the feeder and duplex if required. | The saved PDF contains every intended page in the correct order; scanning works independently of the print queue. |

### ✅ Finish the job

- [ ] The original document prints with correct page size, tray, duplex and color.
- [ ] Another user can still print through any shared queue.
- [ ] Required scan/ADF output opens correctly and the model, IP, port and driver are recorded.

**Change route when:** Changing a shared spooler/driver would interrupt other users without coordination. · Protected-print policy, ARM compatibility or unsupported hardware requires the managed print owner.


---

Editorial review: **2026-11-26** · [Review standard](https://github.com/Herbertofury/INTA/wiki/Knowledge-Standards) · [Suggest a correction](https://github.com/Herbertofury/INTA/issues/new/choose)
