<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Editor, Mark, mergeAttributes } from "@tiptap/core";
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
  let isTyping = $state(false);
  let typingTimer: ReturnType<typeof setTimeout> | undefined;

  const Comment = Mark.create({
    name: 'comment',
    addAttributes() {
      return {
        comment: {
          default: null,
          parseHTML: element => element.getAttribute('data-comment'),
          renderHTML: attributes => ({ 'data-comment': attributes.comment }),
        },
      }
    },
    parseHTML() {
      return [{ tag: 'span[data-comment]' }]
    },
    renderHTML({ HTMLAttributes }) {
      return ['span', mergeAttributes(HTMLAttributes, { class: 'inline-comment' }), 0]
    },
  });

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
        Comment,
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

        // Auto-minimize UI while typing
        isTyping = true;
        document.body.classList.add("is-typing");
        if (typingTimer) clearTimeout(typingTimer);
        typingTimer = setTimeout(() => {
          isTyping = false;
          document.body.classList.remove("is-typing");
        }, 2000);

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
    if (typingTimer) clearTimeout(typingTimer);
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

<div class="editor-wrap" class:pages-mode={pagesMode} class:void-mode={!pagesMode} class:is-typing={isTyping}>
  <div class="editor-surface" bind:this={element}></div>
</div>

<style>
  .editor-wrap {
    height: 100%;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow-y: auto;
  }

  /* Page Mode: Fixed paper-like layout */
  .editor-wrap.pages-mode :global(.ProseMirror) {
    width: 100%;
    max-width: 816px;
    min-height: 1056px;
    margin: 2rem auto;
    padding: 72px 72px 96px;
    background: var(--bg-elevated);
    box-shadow: 0 4px 15px var(--page-shadow);
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

  /* Void Mode: Continuous vertical writing */
  .editor-wrap.void-mode :global(.ProseMirror) {
    max-width: 720px;
    width: 100%;
    margin: 4rem auto;
    min-height: 100vh;
    outline: none;
    font-family: var(--font-prose);
    font-size: 1.15rem;
    line-height: 1.8;
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

  :global(.inline-comment) {
    background-color: #fef08a;
    border-bottom: 2px solid #eab308;
    cursor: help;
  }

  :global(.inline-comment:hover::after) {
    content: attr(data-comment);
    position: absolute;
    background: #1a1a1a;
    color: white;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    z-index: 1000;
    max-width: 200px;
    white-space: normal;
    margin-top: 1.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }
</style>
