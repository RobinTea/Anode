<script lang="ts">
  import { api, type PageMeta } from "$lib/api";

  interface Props {
    pages: PageMeta[];
    onSelect: (id: string) => void;
    onReorder: (id: string, direction: "up" | "down") => void;
  }

  let { pages, onSelect, onReorder }: Props = $props();
</script>

<div class="page-overview">
  <div class="grid">
    {#each pages as page, i (page.id)}
      <div class="page-card" role="button" tabindex="0" onclick={() => onSelect(page.id)} onkeydown={e => e.key === 'Enter' && onSelect(page.id)}>
        <div class="preview">
          <div class="content-skeleton">
            <div class="line" style="width: 80%"></div>
            <div class="line" style="width: 95%"></div>
            <div class="line" style="width: 60%"></div>
            <div class="line" style="width: 85%"></div>
          </div>
        </div>
        <div class="info">
          <span class="index">{i + 1}</span>
          <span class="title">{page.title}</span>
          <div class="actions" onclick={e => e.stopPropagation()}>
            <button disabled={i === 0} onclick={() => onReorder(page.id, "up")}>▲</button>
            <button disabled={i === pages.length - 1} onclick={() => onReorder(page.id, "down")}>▼</button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .page-overview {
    padding: 1rem 0;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1.5rem;
  }

  .page-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    aspect-ratio: 8.5 / 11; /* Paper aspect ratio */
  }

  .page-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 10px 20px rgba(0,0,0,0.1);
    border-color: var(--accent);
  }

  .preview {
    flex: 1;
    padding: 1.5rem;
    background: white;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .content-skeleton {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    opacity: 0.2;
  }

  .line {
    height: 4px;
    background: var(--text);
    border-radius: 2px;
  }

  .info {
    padding: 0.75rem;
    background: var(--bg-elevated);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .index {
    font-size: 0.7rem;
    font-weight: 700;
    color: var(--text-muted);
    background: var(--bg);
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }

  .title {
    flex: 1;
    font-size: 0.85rem;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions {
    display: flex;
    gap: 2px;
  }

  .actions button {
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 0.6rem;
    padding: 2px;
    color: var(--text-muted);
  }

  .actions button:hover:not(:disabled) {
    color: var(--accent);
  }

  .actions button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
</style>
