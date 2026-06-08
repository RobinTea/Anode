<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Link from "@tiptap/extension-link";
  import Placeholder from "@tiptap/extension-placeholder";
  import FormattingToolbar from "./FormattingToolbar.svelte";
  import { setSaveState, setWordCounts } from "$lib/stores/app.svelte";
  import { undoStore, undoRestoreData } from "$lib/stores/undo.svelte";

  interface Props {
    bookId: string;
    pageId: string;
    initialDoc?: Record<string, unknown>;
    editable?: boolean;
    pagesMode?: boolean;
    placeholder?: string;
    bookWordTotal?: number;
    onsave?: (doc: Record<string, unknown>, plain: string) => void;
  }

  let {
    bookId,
    pageId,
    initialDoc = { type: "doc", content: [] },
    editable = true,
    pagesMode = true,
    placeholder = "Start writing…",
    bookWordTotal = 0,
    onsave,
  }: Props = $props();

  let element: HTMLDivElement;
  let editor: Editor | undefined;
  let saveTimer: ReturnType<typeof setTimeout> | undefined;
  let lastDoc: Record<string, unknown> = initialDoc;
  let isRestoringUndo = false;

  onMount(() => {
    editor = new Editor({
      element,
      extensions: [
        StarterKit,
        Link.configure({
          openOnClick: true,
          autolink: true,
        }),
        Placeholder.configure({ placeholder }),
      ],
      content: initialDoc,
      editable,
      editorProps: {
        attributes: {
          spellcheck: editable ? "true" : "false",
          class: pagesMode ? "prose pages-mode" : "prose void-mode",
        },
      },
      onUpdate: ({ editor: ed }) => {
        if (isRestoringUndo) return;
        const json = ed.getJSON() as Record<string, unknown>;
        const plain = ed.getText();
        const words = plain.split(/\s+/).filter(Boolean).length;
        setWordCounts(words, bookWordTotal);
        
        // Push to undo stack
        undoStore.push({
          pageId,
          before: lastDoc,
          after: json,
          timestamp: Date.now(),
          plainText: plain,
        });
        
        lastDoc = JSON.parse(JSON.stringify(json));
        scheduleSave(json, plain);
      },
    });
    lastDoc = JSON.parse(JSON.stringify(initialDoc));

    // Listen for undo/redo restores
    const unsubscribe = undoRestoreData.subscribe((entry) => {
      if (entry && entry.pageId === pageId && editor) {
        isRestoringUndo = true;
        const doc = entry.after;
        editor.commands.setContent(doc);
        lastDoc = JSON.parse(JSON.stringify(doc));
        isRestoringUndo = false;
        undoRestoreData.set(null);
      }
    });

    return () => unsubscribe();
  });

  onDestroy(() => {
    if (saveTimer) clearTimeout(saveTimer);
    editor?.destroy();
  });

  function scheduleSave(doc: Record<string, unknown>, plain: string) {
    if (!onsave) return;
    setSaveState("saving");
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => {
      onsave(doc, plain);
      setSaveState("saved", new Date());
    }, 2000);
  }

  export function getEditor() {
    return editor;
  }

  export function setContent(doc: Record<string, unknown>) {
    if (editor) {
      isRestoringUndo = true;
      editor.commands.setContent(doc);
      lastDoc = JSON.parse(JSON.stringify(doc));
      isRestoringUndo = false;
    }
  }
</script>

{#if editable}
  <FormattingToolbar {editor} />
{/if}

<div class="editor-wrap" class:pages-mode={pagesMode} class:void-mode={!pagesMode}>
  <div class="editor-surface" bind:this={element}></div>
</div>

<style>
  .editor-wrap {
    height: 100%;
    overflow: auto;
    padding: 1rem;
  }

  .editor-wrap.pages-mode :global(.ProseMirror) {
    max-width: 816px;
    min-height: 1056px;
    margin: 0 auto 1.5rem;
    padding: 72px 72px 96px;
    background: var(--bg-elevated);
    box-shadow: 0 2px 12px var(--page-shadow);
    border: 1px solid var(--border);
    font-family: var(--font-prose);
    font-size: 12pt;
    line-height: 1.6;
    outline: none;
  }

  .editor-wrap :global(.ProseMirror p.is-editor-empty:first-child::before) {
    color: var(--text-muted);
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
  }

  .editor-wrap.void-mode :global(.ProseMirror) {
    max-width: 720px;
    margin: 0 auto;
    min-height: 100%;
    outline: none;
    font-family: var(--font-prose);
    line-height: 1.7;
  }

  :global(.ProseMirror a) {
    color: var(--accent);
    cursor: pointer;
    text-decoration: underline;
  }

  :global(.ProseMirror a:hover) {
    color: var(--accent-hover);
  }

  :global(.ProseMirror code) {
    background: var(--status-bg);
    border-radius: 3px;
    padding: 0 0.25rem;
    font-family: "Courier New", monospace;
  }

  :global(.ProseMirror pre) {
    background: var(--status-bg);
    border-radius: 6px;
    padding: 1rem;
    overflow-x: auto;
  }

  :global(.ProseMirror blockquote) {
    border-left: 3px solid var(--accent);
    padding-left: 1rem;
    margin-left: 0;
    font-style: italic;
    color: var(--text-muted);
  }
</style>
