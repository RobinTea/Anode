<script lang="ts">
  import { appStatus } from "$lib/stores/app.svelte";
  import { t } from "$lib/i18n";

  const saveLabel = $derived(
    appStatus.saveState === "saving"
      ? t("status.saving")
      : appStatus.savedAt
        ? `${t("status.saved")} ${appStatus.savedAt.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`
        : "",
  );
</script>

<footer class="status-bar" role="status">
  <span class="mode">{appStatus.editorMode === "write" ? t("status.write") : t("status.cmd")}</span>
  <span class="words">
    {appStatus.pageWords} {t("status.words.page")} ·
    {appStatus.bookWords} {t("status.words.book")} ·
    {appStatus.sessionWords} {t("status.words.session")}
  </span>
  <span class="save">{saveLabel}</span>
  <span class="clock">{appStatus.clock}</span>
</footer>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.35rem 0.75rem;
    font-size: 0.75rem;
    background: var(--status-bg);
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .mode {
    font-weight: 600;
    color: var(--accent);
    min-width: 3.5rem;
  }

  .words {
    flex: 1;
  }

  .clock {
    font-variant-numeric: tabular-nums;
    min-width: 2.5rem;
    text-align: right;
  }
</style>
