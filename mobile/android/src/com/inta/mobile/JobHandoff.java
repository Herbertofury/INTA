package com.inta.mobile;

import java.util.Locale;

final class JobHandoff {
    private JobHandoff() {}

    static String render(JobTimer job, InvoiceDraft invoice, String[] steps) {
        if (job.phase != JobTimer.Phase.COMPLETE || steps.length != Integer.bitCount(JobTimer.ALL_STEPS)) {
            throw new IllegalArgumentException("Complete the job before preparing a handoff");
        }
        if (invoice != null && invoice.recordedMs != job.recordedMs) {
            throw new IllegalArgumentException("Saved billing draft differs from the completed job");
        }
        StringBuilder text = new StringBuilder("INTA service handoff\n\nStatus: Completed locally\n");
        long seconds = job.recordedMs / 1000;
        text.append(String.format(Locale.ROOT, "Recorded time: %02d:%02d:%02d%n%n",
                seconds / 3600, (seconds / 60) % 60, seconds % 60));
        if (job.interrupted) text.append("Unverified time from a previous clock was excluded.\n\n");
        text.append("Technician checklist\n");
        for (int i = 0; i < steps.length; i++) {
            text.append((job.steps & (1 << i)) == 0 ? "[ ] " : "[x] ").append(steps[i]).append('\n');
        }
        if (invoice != null) {
            text.append("\nSaved billing draft (USD)\n");
            text.append("Billing profile: ").append(invoice.profile == InvoiceDraft.Profile.BUSINESS
                    ? "Business" : "Residential").append('\n');
            text.append("Hourly rate: ").append(invoice.hourlyDollars()).append('\n');
            text.append(invoice.minutes < 0 ? "Billable time: Recorded time\n"
                    : "Billable minutes: " + invoice.minutes + "\n");
            text.append("Draft subtotal: ").append(invoice.subtotalDollars()).append('\n');
        }
        return text.toString();
    }
}
