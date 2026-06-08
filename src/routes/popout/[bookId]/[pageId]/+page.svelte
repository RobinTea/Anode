<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { api, type PageBody, type PageMeta } from "$lib/api";
  import TipTapEditor from "$lib/components/TipTapEditor.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";

  const bookId = $derived($page.params.bookId);
  const pageId = $derived($page.params.pageId);

  let pageMeta = $state<PageMeta | null>(null);
  let pageBody = $state<PageBody | null>(null);

  onMount(async () => {
    if (!bookId || !pageId) return;
    const pages = await api.listPages(bookId);
    pageMeta = pages.find(p => p.id === pageId) || null;
    pageBody = await api.loadPageBody(bookId, pageId);
  });

  async function handleSave(doc: Record<string, unknown>, plain: string) {
    if (!bookId || !pageId) return;
    await api.savePageBody(bookId, pageId, doc, plain);
  }
</script>

<div class="popout-container">
  {#if bookId && pageId && pageMeta && pageBody}
    <header class="popout-header">
      <h1>{pageMeta.title}</h1>
    </header>
    <main class="popout-main">
      <TipTapEditor
        {bookId}
        pageId={pageId}
        initialDoc={pageBody.doc}
        editable={true}
        pagesMode={pageMeta.kind === "write"}
        onsave={handleSave}
      />
    </main>
    <StatusBar />
  {:else}
    <div class="loading">Loading...</div>
  {/if}
</div>

<style>
  .popout-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg);
  }

  .popout-header {
    padding: 0.5rem 1rem;
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
  }

  .popout-header h1 {
    font-size: 1rem;
    margin: 0;
  }

  .popout-main {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .loading {
    display: flex;
    height: 100%;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }
</style>
