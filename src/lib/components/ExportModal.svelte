<script lang="ts">
  import { t } from "$lib/i18n";
  import { api } from "$lib/api";
  import { save as saveFile } from "@tauri-apps/plugin-dialog";

  interface Props {
    isOpen: boolean;
    bookId: string;
    bookTitle: string;
    onDone?: () => void;
    onCancel?: () => void;
  }

  let { isOpen = $bindable(), bookId, bookTitle, onDone, onCancel }: Props = $props();

  let includeSnapshots = $state(localStorage.getItem("anode-export-include-snapshots") === "true");
  let isExporting = $state(false);
  let error = $state("");

  async function handleExport() {
    isExporting = true;
    error = "";
    
    try {
      // Use the save dialog from Tauri
      const path = await saveFile({
        defaultPath: `${bookTitle}.anode`,
        filters: [{ name: "Anode Books", extensions: ["anode"] }],
      });

      if (!path) return;

      await api.exportBook(bookId, includeSnapshots, path);
      
      // Remember the preference
      localStorage.setItem("anode-export-include-snapshots", includeSnapshots.toString());
      
      isOpen = false;
      onDone?.();
    } catch (err) {
      error = `Export failed: ${err}`;
      console.error(err);
    } finally {
      isExporting = false;
    }
  }

  function handleCancel() {
    isOpen = false;
    onCancel?.();
  }
</script>

{#if isOpen}
  <div class="modal-backdrop" role="presentation" onclick={handleCancel} onkeydown={(e) => e.key === "Escape" && handleCancel()}>
    <div class="modal" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.stopPropagation()}>
      <h2>{t("export.title")}</h2>
      
      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      <label class="checkbox-label">
        <input type="checkbox" bind:checked={includeSnapshots} />
        <span>{t("export.includeSnapshots")}</span>
      </label>

      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={handleCancel} disabled={isExporting}>
          {t("delete.cancel")}
        </button>
        <button class="btn btn-primary" onclick={handleExport} disabled={isExporting}>
          {isExporting ? "Exporting…" : t("export.button")}
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
    max-width: 400px;
    width: 90%;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  }

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
    color: var(--text);
  }

  .error-message {
    background: #fee2e2;
    border: 1px solid #fecaca;
    color: #991b1b;
    padding: 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    cursor: pointer;
  }

  input[type="checkbox"] {
    cursor: pointer;
  }

  span {
    color: var(--text);
    font-size: 0.95rem;
  }

  .modal-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }
</style>
