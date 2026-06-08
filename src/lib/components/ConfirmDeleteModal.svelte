<script lang="ts">
  import { t } from "$lib/i18n";

  interface Props {
    isOpen: boolean;
    bookTitle: string;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let { isOpen = $bindable(), bookTitle, onConfirm, onCancel }: Props = $props();

  let confirmText = $state("");
  let isValid = $derived(confirmText === bookTitle);

  function handleConfirm() {
    if (isValid) {
      onConfirm();
      confirmText = "";
    }
  }

  function handleCancel() {
    confirmText = "";
    onCancel();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && isValid) {
      handleConfirm();
    } else if (e.key === "Escape") {
      handleCancel();
    }
  }
</script>

{#if isOpen}
  <div class="modal-backdrop" role="presentation" onclick={handleCancel} onkeydown={(e) => e.key === "Escape" && handleCancel()}>
    <div class="modal" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.stopPropagation()}>
      <h2>{t("delete.title")}</h2>
      <p>{t("delete.confirm")}</p>
      <div class="book-title-display">
        <code>{bookTitle}</code>
      </div>
      <input
        type="text"
        bind:value={confirmText}
        placeholder={t("delete.placeholder")}
        onkeydown={handleKeydown}
        autofocus
      />
      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={handleCancel}>{t("delete.cancel")}</button>
        <button class="btn btn-danger" disabled={!isValid} onclick={handleConfirm}>
          {t("delete.confirm_button")}
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
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
    color: var(--text);
  }

  p {
    margin: 0 0 1rem 0;
    color: var(--text-muted);
    font-size: 0.95rem;
  }

  .book-title-display {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0.75rem;
    margin: 0.75rem 0;
  }

  code {
    font-family: monospace;
    color: var(--accent);
    font-size: 0.9rem;
    word-break: break-all;
  }

  input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg);
    color: var(--text);
    font-size: 0.95rem;
    margin-bottom: 1rem;
    box-sizing: border-box;
  }

  input:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-light);
  }

  .modal-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  .btn-danger {
    background: #dc2626;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }

  .btn-danger:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
