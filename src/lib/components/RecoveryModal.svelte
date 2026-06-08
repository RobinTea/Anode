<script lang="ts">
  import { t } from "$lib/i18n";
  import { api, type PageMeta, type SnapshotInfo } from "$lib/api";

  interface Props {
    isOpen: boolean;
    bookId: string;
    pages: PageMeta[];
    onKeepLatest: () => void;
    onCancel: () => void;
  }

  let { isOpen = $bindable(), bookId, pages, onKeepLatest, onCancel }: Props = $props();

  let selectedPageId = $state("");
  let snapshots = $state<SnapshotInfo[]>([]);
  let loadingSnapshots = $state(false);
  let selectedSnapshot = $state("");
  let showPicker = $state(false);

  async function loadSnapshots(pageId: string) {
    loadingSnapshots = true;
    selectedPageId = pageId;
    try {
      snapshots = await api.listSnapshots(bookId, pageId);
      if (snapshots.length > 0) {
        selectedSnapshot = snapshots[0].filename;
      }
    } catch (err) {
      console.error("Failed to load snapshots:", err);
      snapshots = [];
    }
    loadingSnapshots = false;
  }

  async function restoreAndClose() {
    if (!selectedPageId || !selectedSnapshot) return;
    try {
      await api.restoreSnapshot(bookId, selectedPageId, selectedSnapshot);
      isOpen = false;
      onKeepLatest();
    } catch (err) {
      console.error("Failed to restore snapshot:", err);
    }
  }

  function handleCancel() {
    showPicker = false;
    selectedPageId = "";
    snapshots = [];
    selectedSnapshot = "";
    onCancel();
  }
</script>

{#if isOpen}
  <div class="modal-backdrop" role="presentation" onclick={handleCancel} onkeydown={(e) => e.key === "Escape" && handleCancel()}>
    <div class="modal" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.stopPropagation()}>
      <h2>{t("recovery.title")}</h2>
      <p>{t("recovery.description")}</p>

      <div class="recovery-actions">
        <button class="btn btn-primary" onclick={() => { isOpen = false; onKeepLatest(); }}>
          {t("recovery.keepLatest")}
        </button>
        <button class="btn btn-secondary" onclick={() => (showPicker = !showPicker)}>
          {t("recovery.revertOlder")}
        </button>
      </div>

      {#if showPicker}
        <div class="recovery-picker">
          <label>
            {t("recovery.selectPage")}
            <select bind:value={selectedPageId} onchange={(e) => loadSnapshots(e.currentTarget.value)}>
              <option value="">{t("recovery.selectPagePlaceholder")}</option>
              {#each pages as page (page.id)}
                <option value={page.id}>
                  {page.title} ({page.kind})
                </option>
              {/each}
            </select>
          </label>

          {#if loadingSnapshots}
            <p class="loading">Loading snapshots…</p>
          {/if}

          {#if snapshots.length > 0}
            <label>
              {t("recovery.selectSnapshot")}
              <select bind:value={selectedSnapshot}>
                {#each snapshots as snapshot (snapshot.filename)}
                  <option value={snapshot.filename}>
                    {new Date(snapshot.timestamp).toLocaleString()} ({snapshot.size_bytes} bytes)
                  </option>
                {/each}
              </select>
            </label>
            <button class="btn btn-primary" onclick={restoreAndClose} disabled={!selectedSnapshot}>
              {t("recovery.restore")}
            </button>
          {:else if selectedPageId && !loadingSnapshots}
            <p class="no-snapshots">{t("recovery.noSnapshots")}</p>
          {/if}
        </div>
      {/if}

      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={handleCancel}>
          {t("recovery.cancel")}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.5rem;
    max-width: 500px;
    width: 90%;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  }

  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
    color: var(--text);
  }

  p {
    margin: 0 0 1rem 0;
    color: var(--text-muted);
    font-size: 0.95rem;
  }

  .recovery-actions {
    display: flex;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .recovery-picker {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.75rem;
    font-size: 0.9rem;
    color: var(--text);
  }

  select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-elevated);
    color: var(--text);
    margin-top: 0.25rem;
  }

  select:focus {
    outline: none;
    border-color: var(--accent);
  }

  .loading,
  .no-snapshots {
    padding: 0.5rem 0;
    font-size: 0.9rem;
    color: var(--text-muted);
  }

  .modal-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }
</style>
