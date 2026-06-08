<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Character } from "$lib/api";
  import { t } from "$lib/i18n";

  interface Props {
    bookId: string;
  }

  let { bookId }: Props = $props();
  let characters = $state<Character[]>([]);
  let selectedId = $state<string | null>(null);
  let isEditing = $state(false);
  let newName = $state("");

  const selected = $derived(characters.find(c => c.id === selectedId) || null);

  onMount(async () => {
    characters = await api.listCharacters(bookId);
  });

  async function create() {
    if (!newName.trim()) return;
    const char = await api.createCharacter(bookId, newName);
    characters = [...characters, char];
    selectedId = char.id;
    newName = "";
    isEditing = true;
  }

  async function save() {
    if (!selected) return;
    await api.updateCharacter(bookId, selected.id, selected);
    isEditing = false;
    characters = await api.listCharacters(bookId);
  }

  async function remove(id: string) {
    await api.deleteCharacter(bookId, id);
    characters = characters.filter(c => c.id !== id);
    if (selectedId === id) selectedId = null;
  }
</script>

<div class="character-manager card">
  <div class="char-sidebar">
    <div class="sidebar-header">
      <h3>Characters</h3>
      <div class="add-char">
        <input bind:value={newName} placeholder="New character..." onkeydown={e => e.key === 'Enter' && create()} />
        <button class="btn btn-sm" onclick={create}>+</button>
      </div>
    </div>
    <ul class="char-list">
      {#each characters as char (char.id)}
        <li>
          <button class:selected={selectedId === char.id} onclick={() => { selectedId = char.id; isEditing = false; }}>
            {char.name}
            <span class="role">{char.role}</span>
          </button>
        </li>
      {/each}
    </ul>
  </div>

  <div class="char-main">
    {#if selected}
      <div class="char-detail">
        <header>
          {#if isEditing}
            <input class="title-input" bind:value={selected.name} />
          {:else}
            <h2>{selected.name}</h2>
          {/if}
          <div class="actions">
            {#if isEditing}
              <button class="btn btn-primary" onclick={save}>Save</button>
              <button class="btn" onclick={() => isEditing = false}>Cancel</button>
            {:else}
              <button class="btn" onclick={() => isEditing = true}>Edit</button>
              <button class="btn btn-danger" onclick={() => remove(selected.id)}>Delete</button>
            {/if}
          </div>
        </header>

        <div class="detail-grid">
          <label>
            <span>Role</span>
            {#if isEditing}
              <input bind:value={selected.role} placeholder="Protagonist, Antagonist, etc." />
            {:else}
              <p>{selected.role || "No role set"}</p>
            {/if}
          </label>

          <label>
            <span>Description</span>
            {#if isEditing}
              <textarea bind:value={selected.description} placeholder="Physical appearance, traits..."></textarea>
            {:else}
              <p class="pre-wrap">{selected.description || "No description set"}</p>
            {/if}
          </label>

          <label>
            <span>Notes</span>
            {#if isEditing}
              <textarea bind:value={selected.notes} placeholder="Backstory, motivations, etc."></textarea>
            {:else}
              <p class="pre-wrap">{selected.notes || "No notes set"}</p>
            {/if}
          </label>
        </div>
      </div>
    {:else}
      <div class="empty-state">
        <p>Select or create a character to view details</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .character-manager {
    display: grid;
    grid-template-columns: 240px 1fr;
    height: 600px;
    padding: 0;
    overflow: hidden;
  }

  .char-sidebar {
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    background: var(--bg-elevated);
  }

  .sidebar-header {
    padding: 1rem;
    border-bottom: 1px solid var(--border);
  }

  .sidebar-header h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .add-char {
    display: flex;
    gap: 0.5rem;
  }

  .add-char input {
    flex: 1;
    padding: 0.3rem 0.5rem;
    font-size: 0.85rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg);
    color: var(--text);
  }

  .char-list {
    list-style: none;
    padding: 0.5rem;
    margin: 0;
    overflow-y: auto;
  }

  .char-list button {
    width: 100%;
    text-align: left;
    padding: 0.75rem;
    border: none;
    background: transparent;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
  }

  .char-list button:hover {
    background: var(--bg-hover);
  }

  .char-list button.selected {
    background: var(--accent);
    color: white;
  }

  .char-list button.selected .role {
    color: rgba(255,255,255,0.8);
  }

  .role {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 0.2rem;
  }

  .char-main {
    padding: 2rem;
    overflow-y: auto;
    background: var(--bg);
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  h2 {
    margin: 0;
  }

  .title-input {
    font-size: 1.5rem;
    font-weight: 700;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-elevated);
    color: var(--text);
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .detail-grid {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label span {
    font-weight: 600;
    font-size: 0.85rem;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  input, textarea {
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-elevated);
    color: var(--text);
    font-family: inherit;
  }

  textarea {
    min-height: 100px;
    resize: vertical;
  }

  .pre-wrap {
    white-space: pre-wrap;
    line-height: 1.6;
  }

  .btn-danger {
    background: #ef4444;
    color: white;
    border: none;
  }

  .btn-danger:hover {
    background: #dc2626;
  }
</style>
