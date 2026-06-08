<script lang="ts">
  import { t } from "$lib/i18n";
  import type { Editor } from "@tiptap/core";
  import { onMount } from "svelte";

  interface Props {
    editor: Editor | undefined;
  }

  let { editor }: Props = $props();

  onMount(() => {
    // Force re-render when editor is ready
  });
</script>

{#if editor}
  <div class="toolbar-container">
    <div class="toolbar card">
      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          class:active={editor.isActive("bold")}
          onclick={() => editor.chain().focus().toggleBold().run()}
          title="Bold"
        >
          <strong>B</strong>
        </button>
        <button
          class="toolbar-btn"
          class:active={editor.isActive("italic")}
          onclick={() => editor.chain().focus().toggleItalic().run()}
          title="Italic"
        >
          <em>I</em>
        </button>
        <button
          class="toolbar-btn"
          class:active={editor.isActive("strike")}
          onclick={() => editor.chain().focus().toggleStrike().run()}
          title="Strike"
        >
          <s>S</s>
        </button>
      </div>

      <div class="toolbar-sep"></div>

      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          class:active={editor.isActive("heading", { level: 1 })}
          onclick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
        >
          H1
        </button>
        <button
          class="toolbar-btn"
          class:active={editor.isActive("heading", { level: 2 })}
          onclick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
        >
          H2
        </button>
        <button
          class="toolbar-btn"
          class:active={editor.isActive("blockquote")}
          onclick={() => editor.chain().focus().toggleBlockquote().run()}
        >
          “”
        </button>
      </div>

      <div class="toolbar-sep"></div>

      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          class:active={editor.isActive("bulletList")}
          onclick={() => editor.chain().focus().toggleBulletList().run()}
        >
          • List
        </button>
        <button
          class="toolbar-btn"
          class:active={editor.isActive("orderedList")}
          onclick={() => editor.chain().focus().toggleOrderedList().run()}
        >
          1. List
        </button>
      </div>

      <div class="toolbar-sep"></div>

      <div class="toolbar-group">
        <button
          class="toolbar-btn"
          onclick={() => {
            const comment = prompt("Enter comment:");
            if (comment) {
              editor.chain().focus().setMark("comment", { comment }).run();
            }
          }}
          title="Add Comment"
        >
          💬
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .toolbar-container {
    position: sticky;
    top: 0;
    z-index: 50;
    padding: 0.5rem;
    display: flex;
    justify-content: center;
    background: linear-gradient(to bottom, var(--bg) 0%, transparent 100%);
    pointer-events: none;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    pointer-events: auto;
  }

  .toolbar-group {
    display: flex;
    gap: 0.15rem;
  }

  .toolbar-sep {
    width: 1px;
    height: 1.5rem;
    background: var(--border);
    margin: 0 0.25rem;
  }

  .toolbar-btn {
    background: transparent;
    border: none;
    padding: 0.35rem 0.6rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    color: var(--text);
    min-width: 2rem;
    transition: all 0.15s ease;
  }

  .toolbar-btn:hover {
    background: var(--bg-hover);
  }

  .toolbar-btn.active {
    background: var(--accent);
    color: white;
  }
</style>
