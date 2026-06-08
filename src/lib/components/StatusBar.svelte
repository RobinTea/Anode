<script lang="ts">
  import { appStatus } from "$lib/stores/app.svelte";
  import { t } from "$lib/i18n";
  import PomodoroTimer from "./PomodoroTimer.svelte";

  const saveLabel = $derived(
    appStatus.saveState === "saving"
      ? t("status.saving")
      : appStatus.saveState === "saved"
        ? `${t("status.saved")} ${appStatus.savedAt?.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`
        : t("status.idle"),
  );
</script>

<footer class="status-bar" role="status">
  <span class="mode">{t("status.write")}</span>
  <div class="word-stats">
    <span class="stat">
      <strong>{appStatus.pageWords}</strong> page
    </span>
    <span class="sep">|</span>
    <span class="stat">
      <strong>{appStatus.bookWords}</strong> book
    </span>
    <span class="sep">|</span>
    <span class="stat">
      <strong>{appStatus.sessionWords}</strong> session
    </span>
  </div>
  <PomodoroTimer />
  <span class="save">{saveLabel}</span>
  <span class="clock">{appStatus.clock}</span>
</footer>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.35rem 1rem;
    font-size: 0.75rem;
    background: var(--status-bg);
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .mode {
    font-weight: 700;
    color: var(--accent);
    letter-spacing: 0.05em;
  }

  .word-stats {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .stat strong {
    color: var(--text);
  }

  .sep {
    opacity: 0.3;
  }

  .save {
    font-style: italic;
  }

  .clock {
    font-variant-numeric: tabular-nums;
    text-align: right;
  }
</style>
