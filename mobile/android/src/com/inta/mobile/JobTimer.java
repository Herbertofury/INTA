package com.inta.mobile;

/** A single offline job; no calendar clock, network, or retry timer is involved. */
final class JobTimer {
    static final int STEPS = 6;
    static final int ALL_STEPS = (1 << STEPS) - 1;
    enum Phase { PAUSED, RUNNING, COMPLETE }

    final long recordedMs;
    final long startedAt;
    final int boot;
    final int steps;
    final Phase phase;
    final boolean interrupted;

    JobTimer(long recordedMs, long startedAt, int boot, int steps, Phase phase, boolean interrupted) {
        if (recordedMs < 0 || startedAt < 0 || (steps & ~ALL_STEPS) != 0
                || (phase == Phase.RUNNING ? boot < 0 : startedAt != 0 || boot != -1)) {
            throw new IllegalArgumentException("Invalid job state");
        }
        this.recordedMs = recordedMs;
        this.startedAt = startedAt;
        this.boot = boot;
        this.steps = steps;
        this.phase = phase;
        this.interrupted = interrupted;
    }

    static JobTimer empty() { return new JobTimer(0, 0, -1, 0, Phase.PAUSED, false); }

    JobTimer detachClock() {
        return phase == Phase.RUNNING
                ? new JobTimer(recordedMs, 0, -1, steps, Phase.PAUSED, true) : this;
    }

    JobTimer reconcile(long now, int thisBoot) {
        return phase == Phase.RUNNING && (boot != thisBoot || now < startedAt)
                ? detachClock() : this;
    }

    long elapsed(long now, int thisBoot) {
        if (phase != Phase.RUNNING || boot != thisBoot || now < startedAt) return recordedMs;
        final long additional = now - startedAt;
        return Long.MAX_VALUE - recordedMs < additional ? Long.MAX_VALUE : recordedMs + additional;
    }

    JobTimer start(long now, int thisBoot) {
        if (phase != Phase.PAUSED || thisBoot < 0 || now < 0) return this;
        return new JobTimer(recordedMs, now, thisBoot, steps, Phase.RUNNING, interrupted);
    }

    JobTimer checkpoint(long now, int thisBoot) {
        JobTimer current = reconcile(now, thisBoot);
        if (current != this || phase != Phase.RUNNING) return current;
        return new JobTimer(elapsed(now, thisBoot), now, boot, steps, phase, interrupted);
    }

    JobTimer pause(long now, int thisBoot) {
        JobTimer current = reconcile(now, thisBoot);
        if (current.phase != Phase.RUNNING) return current;
        return new JobTimer(elapsed(now, thisBoot), 0, -1, steps, Phase.PAUSED, interrupted);
    }

    JobTimer complete(long now, int thisBoot) {
        JobTimer stopped = pause(now, thisBoot);
        return new JobTimer(stopped.recordedMs, 0, -1, steps, Phase.COMPLETE, stopped.interrupted);
    }

    JobTimer check(int index, boolean checked) {
        if (phase == Phase.COMPLETE || index < 0 || index >= STEPS) return this;
        int bit = 1 << index;
        return new JobTimer(recordedMs, startedAt, boot, checked ? steps | bit : steps & ~bit, phase, interrupted);
    }
}
