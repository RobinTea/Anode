<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib/api";

  interface Props {
    bookId: string;
    pageId: string;
    initialNotes?: string;
    onsave: (notes: string) => void;
  }

  let { bookId, pageId, initialNotes = "", onsave }: Props = $props();
  let notes = $state(initialNotes);
  let saveTimer: ReturnType<typeof setTimeout> | undefined;

  function handleInput() {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => {
      onsave(notes);
    }, 1000);
  }

  onMount(() => {
    notes = initialNotes;
  });
</script>

<div class="note-sidebar">
  <header>
    <h3>Quick Notes</h3>
    <span class="hint">Auto-saves locally</span>
  </header>
  <textarea
    bind:value={notes}
    oninput={handleInput}
    placeholder="Type quick notes for this chapter here..."
  ></textarea>
</div>

<style>
  .note-sidebar {
    width: 280px;
    height: 100%;
    border-left: 1px solid var(--border);
    background: var(--bg-elevated);
    display: flex;
    flex-direction: column;
    padding: 1rem;
    box-sizing: border-box;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 1rem;
  }

  h3 {
    margin: 0;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    font-style: italic;
  }

  textarea {
    flex: 1;
    background: transparent;
    border: none;
    resize: none;
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.5;
    color: var(--text);
    outline: none;
    padding: 0;
  }

  textarea::placeholder {
    color: var(--text-muted);
    opacity: 0.5;
  }
</style>
