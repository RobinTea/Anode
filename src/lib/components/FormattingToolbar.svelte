<script lang="ts">
  import { t } from "$lib/i18n";
  import type { Editor } from "@tiptap/core";
  import { onMount } from "svelte";

  interface Props {
    editor: Editor | undefined;
  }

  let { editor }: Props = $props();

  let isSticky = $state(false);
  let isMobile = $state(false);
  let isCollapsed = $state(false);

  onMount(() => {
    const checkMobile = () => {
      isMobile = window.innerWidth < 768;
      isCollapsed = isMobile;
    };
    checkMobile();
    window.addEventListener("resize", checkMobile);
    return () => window.removeEventListener("resize", checkMobile);
  });

  function toggleBold() {
    editor?.chain().focus().toggleBold().run();
  }

  function toggleItalic() {
    editor?.chain().focus().toggleItalic().run();
  }

  function toggleH1() {
    editor?.chain().focus().toggleHeading({ level: 1 }).run();
  }

  function toggleH2() {
    editor?.chain().focus().toggleHeading({ level: 2 }).run();
  }

  function toggleH3() {
    editor?.chain().focus().toggleHeading({ level: 3 }).run();
  }

  function toggleBulletList() {
    editor?.chain().focus().toggleBulletList().run();
  }

  function toggleOrderedList() {
    editor?.chain().focus().toggleOrderedList().run();
  }

  function toggleBlockquote() {
    editor?.chain().focus().toggleBlockquote().run();
  }

  function toggleCodeBlock() {
    editor?.chain().focus().toggleCodeBlock().run();
  }

  function insertLink() {
    const url = prompt(t("toolbar.link"));
    if (url) {
      editor?.chain().focus().extendMarkRange("link").setLink({ href: url }).run();
    }
  }

  function clearFormat() {
    editor?.chain().focus().clearNodes().unsetAllMarks().run();
  }

  function isBoldActive() {
    return editor?.isActive("bold") ?? false;
  }

  function isItalicActive() {
    return editor?.isActive("italic") ?? false;
  }

  function isH1Active() {
    return editor?.isActive("heading", { level: 1 }) ?? false;
  }

  function isH2Active() {
    return editor?.isActive("heading", { level: 2 }) ?? false;
  }

  function isH3Active() {
    return editor?.isActive("heading", { level: 3 }) ?? false;
  }

  function isBulletActive() {
    return editor?.isActive("bulletList") ?? false;
  }

  function isOrderedActive() {
    return editor?.isActive("orderedList") ?? false;
  }

  function isBlockquoteActive() {
    return editor?.isActive("blockquote") ?? false;
  }

  function isCodeBlockActive() {
    return editor?.isActive("codeBlock") ?? false;
  }
</script>

<div class="toolbar-container" class:sticky={isSticky} class:mobile={isMobile}>
  <div class="toolbar">
    {#if !isCollapsed || !isMobile}
      <div class="toolbar-group">
        <button
          title={t("toolbar.bold")}
          class:active={isBoldActive()}
          onclick={toggleBold}
          class="toolbar-btn"
        >
          <strong>B</strong>
        </button>
        <button
          title={t("toolbar.italic")}
          class:active={isItalicActive()}
          onclick={toggleItalic}
          class="toolbar-btn"
        >
          <em>I</em>
        </button>
      </div>

      <div class="toolbar-group">
        <button
          title={t("toolbar.h1")}
          class:active={isH1Active()}
          onclick={toggleH1}
          class="toolbar-btn"
        >
          H1
        </button>
        <button
          title={t("toolbar.h2")}
          class:active={isH2Active()}
          onclick={toggleH2}
          class="toolbar-btn"
        >
          H2
        </button>
        <button
          title={t("toolbar.h3")}
          class:active={isH3Active()}
          onclick={toggleH3}
          class="toolbar-btn"
        >
          H3
        </button>
      </div>

      <div class="toolbar-group">
        <button
          title={t("toolbar.bullet")}
          class:active={isBulletActive()}
          onclick={toggleBulletList}
          class="toolbar-btn"
        >
          • List
        </button>
        <button
          title={t("toolbar.numbered")}
          class:active={isOrderedActive()}
          onclick={toggleOrderedList}
          class="toolbar-btn"
        >
          1. List
        </button>
      </div>

      <div class="toolbar-group">
        <button
          title={t("toolbar.quote")}
          class:active={isBlockquoteActive()}
          onclick={toggleBlockquote}
          class="toolbar-btn"
        >
          " "
        </button>
        <button
          title={t("toolbar.code")}
          class:active={isCodeBlockActive()}
          onclick={toggleCodeBlock}
          class="toolbar-btn"
        >
          &lt;&gt;
        </button>
      </div>

      <div class="toolbar-group">
        <button title={t("toolbar.link")} onclick={insertLink} class="toolbar-btn">
          🔗
        </button>
        <button title={t("toolbar.clear")} onclick={clearFormat} class="toolbar-btn">
          ✕
        </button>
      </div>
    {/if}

    {#if isMobile}
      <button
        onclick={() => (isCollapsed = !isCollapsed)}
        class="toolbar-toggle"
        title={isCollapsed ? "Show toolbar" : "Hide toolbar"}
      >
        {isCollapsed ? "▼" : "▲"}
      </button>
    {/if}
  </div>
</div>

<style>
  .toolbar-container {
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--border);
    padding: 0.5rem;
    display: flex;
    justify-content: center;
    gap: 0.25rem;
    flex-wrap: wrap;
    z-index: 100;
  }

  .toolbar-container.sticky {
    position: sticky;
    top: 0;
    box-shadow: 0 2px 8px var(--page-shadow);
  }

  .toolbar {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    align-items: center;
    width: 100%;
  }

  .toolbar-group {
    display: flex;
    gap: 0.25rem;
    border-right: 1px solid var(--border);
    padding-right: 0.5rem;
  }

  .toolbar-group:last-child {
    border-right: none;
  }

  .toolbar-btn {
    min-width: 2rem;
    height: 2rem;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: all 0.15s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toolbar-btn:hover {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }

  .toolbar-btn.active {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
    font-weight: 600;
  }

  .toolbar-toggle {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    border-radius: 4px;
    cursor: pointer;
  }

  .toolbar-toggle:hover {
    background: var(--accent);
    color: white;
  }

  @media (max-width: 768px) {
    .toolbar-container {
      padding: 0.375rem;
    }

    .toolbar-btn {
      min-width: 1.75rem;
      height: 1.75rem;
      font-size: 0.75rem;
    }

    .toolbar-group {
      gap: 0.125rem;
    }

    .toolbar-container.mobile .toolbar {
      flex-direction: column;
      gap: 0.25rem;
    }
  }
</style>
