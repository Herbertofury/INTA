# PC Deployment & Handover

## Before deployment
Confirm asset ownership, assigned user, role, apps, peripherals, network and accessibility needs. Check hardware compatibility, warranty, firmware, storage/RAM and vendor support. Confirm approved provisioning (Autopilot/Intune, image or documented manual setup), licensing and security baseline.

## Build checklist
- Use official Windows media and approved provisioning.
- Apply firmware/OS updates according to change policy.
- Enroll endpoint management and EDR; verify compliance/reporting.
- Enable encryption and escrow recovery material only to approved recovery system.
- Install approved apps from verified publishers.
- Configure profile, printers, VPN, Wi-Fi and peripherals.
- Remove temporary setup accounts; apply local admin policy.
- Validate backup, restore and support-agent status.

## Acceptance tests
- Boot, sign-in, lock/unlock and reboot.
- Network, DNS, VPN and internal applications.
- Audio, camera, display, dock, printer and peripherals.
- Activation, patch level, EDR health and management check-in.
- Encryption protection and recovery escrow status (never expose the key).
- User can access required work and knows the support route.

## Handover
Explain sign-in, MFA, VPN, approved storage, support requests and suspicious-activity reporting. Never send passwords or recovery keys in handover docs. Record asset/deployment details in approved inventory and ticketing systems.
