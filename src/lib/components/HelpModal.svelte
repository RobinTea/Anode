<script lang="ts">
  import { t } from "$lib/i18n";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen = $bindable(), onClose }: Props = $props();

  const shortcuts = [
    { key: "Ctrl + S", action: "Force Save" },
    { key: "Esc", action: "Return to Hub / Close Modal" },
    { key: "Ctrl + B", action: "Bold" },
    { key: "Ctrl + I", action: "Italic" },
    { key: "Alt + P", action: "Toggle Pomodoro (coming soon)" },
  ];
</script>

{#if isOpen}
  <div class="modal-backdrop" onclick={onClose}>
    <div class="modal card" onclick={e => e.stopPropagation()}>
      <header>
        <h2>Keyboard Shortcuts</h2>
        <button class="close-btn" onclick={onClose}>✕</button>
      </header>
      <div class="shortcut-list">
        {#each shortcuts as s}
          <div class="shortcut-item">
            <span class="key">{s.key}</span>
            <span class="action">{s.action}</span>
          </div>
        {/each}
      </div>
      <div class="footer">
        <p>Anode is designed to be keyboard-first. More shortcuts coming in v3!</p>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    backdrop-filter: blur(4px);
  }

  .modal {
    width: 100%;
    max-width: 450px;
    padding: 1.5rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  h2 { margin: 0; font-size: 1.25rem; }

  .close-btn {
    background: transparent;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: var(--text-muted);
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .shortcut-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border);
  }

  .key {
    font-family: monospace;
    background: var(--bg);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: 700;
    font-size: 0.9rem;
    border: 1px solid var(--border);
  }

  .action {
    color: var(--text-muted);
    font-size: 0.95rem;
  }

  .footer {
    margin-top: 1.5rem;
    font-size: 0.8rem;
    color: var(--text-muted);
    text-align: center;
  }
</style>
