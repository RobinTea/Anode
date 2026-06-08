<script lang="ts">
  import { page } from "$app/stores";
  import { api, type BookMeta, type CompileOrderEntry, type PageBody, type PageMeta } from "$lib/api";
  import TipTapEditor from "$lib/components/TipTapEditor.svelte";
  import ExportModal from "$lib/components/ExportModal.svelte";
  import ExportDocxModal from "$lib/components/ExportDocxModal.svelte";
  import { t } from "$lib/i18n";
  import { setWordCounts } from "$lib/stores/app.svelte";
  import { onMount } from "svelte";

  const bookId = $derived($page.params.id ?? "");

  let meta = $state<BookMeta | null>(null);
  let pages = $state<PageMeta[]>([]);
  let compileOrder = $state<CompileOrderEntry[]>([]);
  let activeTab = $state<"plan" | "write" | "read">("write");
  let selectedPageId = $state<string | null>(null);
  let pageBody = $state<PageBody | null>(null);
  let searchQuery = $state("");
  let showCompile = $state(false);
  let showExport = $state(false);
  let showExportDocx = $state(false);

  const writePages = $derived(pages.filter((p) => p.kind === "write"));
  const planPages = $derived(pages.filter((p) => p.kind === "plan"));
  const selectedPage = $derived(pages.find((p) => p.id === selectedPageId) ?? null);

  onMount(() => load());

  async function load() {
    if (!bookId) return;
    meta = await api.getBookMeta(bookId);
    pages = await api.listPages(bookId);
    compileOrder = await api.getCompileOrder(bookId);
    setWordCounts(0, meta ? writePages.reduce((s, p) => s + p.word_count, 0) : 0);

    const pool = activeTab === "plan" ? planPages : writePages;
    if (!selectedPageId && pool.length > 0) {
      selectedPageId = pool[0].id;
    }
    if (selectedPageId) {
      pageBody = await api.loadPageBody(bookId, selectedPageId);
      const pg = pages.find((p) => p.id === selectedPageId);
      if (pg) setWordCounts(pg.word_count, writePages.reduce((s, p) => s + p.word_count, 0));
    }
  }

  async function selectPage(id: string) {
    selectedPageId = id;
    pageBody = await api.loadPageBody(bookId, id);
    const pg = pages.find((p) => p.id === id);
    if (pg) setWordCounts(pg.word_count, writePages.reduce((s, p) => s + p.word_count, 0));
  }

  async function addPage(kind: "plan" | "write") {
    const title = kind === "plan" ? "New plan" : "New page";
    const created = await api.createPage(bookId, kind, kind === "plan" ? "notes" : "chapter", title);
    await load();
    selectedPageId = created.id;
    activeTab = kind;
  }

  async function handleSave(doc: Record<string, unknown>, plain: string) {
    if (!selectedPageId) return;
    const saved = await api.savePageBody(bookId, selectedPageId, doc, plain);
    pageBody = saved;
    await load();
  }

  async function toggleCompileInclude(entry: CompileOrderEntry) {
    const next = compileOrder.map((e) =>
      e.page_id === entry.page_id ? { ...e, included: !e.included } : e,
    );
    await api.setCompileOrder(bookId, next);
    compileOrder = next;
  }
</script>

{#if meta}
  <div class="book-home">
    <header class="book-header">
      <a href="/" class="back">←</a>
      <h1>{meta.title}</h1>
      <div class="tabs">
        <button class:active={activeTab === "plan"} onclick={() => (activeTab = "plan")}>{t("book.plan")}</button>
        <button class:active={activeTab === "write"} onclick={() => (activeTab = "write")}>{t("book.write")}</button>
        <button class:active={activeTab === "read"} onclick={() => (activeTab = "read")}>{t("book.read")}</button>
      </div>
    </header>

    <div class="search-bar">
      <input bind:value={searchQuery} placeholder={t("book.search")} />
      <button class="btn" onclick={() => (showCompile = !showCompile)}>{t("book.compile")}</button>
      <button class="btn" onclick={() => (showExport = true)}>{t("book.export")}</button>
    </div>

    {#if showCompile}
      <section class="compile-panel card">
        <h3>Compile order</h3>
        <ul>
          {#each compileOrder as entry (entry.page_id)}
            <li>
              <label>
                <input
                  type="checkbox"
                  checked={entry.included}
                  onchange={() => toggleCompileInclude(entry)}
                />
                {entry.title}
              </label>
            </li>
          {/each}
        </ul>
        <button class="btn btn-primary" onclick={() => (showExportDocx = true)} style="width: 100%; margin-top: 1rem;">
          {t("export.docx")}
        </button>
      </section>
    {/if}

    <div class="book-body">
      <aside class="page-sidebar">
        <button class="btn" onclick={() => addPage(activeTab === "plan" ? "plan" : "write")}>
          {activeTab === "plan" ? t("book.addPlanPage") : t("book.addWritePage")}
        </button>
        <ul>
          {#each (activeTab === "plan" ? planPages : writePages) as pg (pg.id)}
            <li>
              <button
                class:selected={pg.id === selectedPageId}
                onclick={() => selectPage(pg.id)}
              >
                {pg.title}
                <span class="wc">{pg.word_count}</span>
              </button>
            </li>
          {/each}
        </ul>
      </aside>

      <section class="editor-pane">
        {#if activeTab === "read" && selectedPage && pageBody}
          <div class="read-view">
            <TipTapEditor
              {bookId}
              pageId={selectedPage.id}
              initialDoc={pageBody.doc}
              editable={false}
              pagesMode={true}
              bookWordTotal={writePages.reduce((s, p) => s + p.word_count, 0)}
            />
          </div>
        {:else if selectedPage && pageBody && activeTab !== "read"}
          <TipTapEditor
            {bookId}
            pageId={selectedPage.id}
            initialDoc={pageBody.doc}
            editable={true}
            pagesMode={activeTab === "write"}
            placeholder={activeTab === "plan" ? t("book.emptyPlan") : t("book.emptyWrite")}
            bookWordTotal={writePages.reduce((s, p) => s + p.word_count, 0)}
            onsave={handleSave}
          />
        {:else}
          <div class="empty-state">
            <p>{t("book.emptyWrite")}</p>
            <button class="btn btn-primary" onclick={() => addPage("write")}>{t("book.addWritePage")}</button>
          </div>
        {/if}
      </section>
    </div>
  </div>
  
  {#if meta}
    <ExportModal
      bind:isOpen={showExport}
      {bookId}
      bookTitle={meta.title}
      onDone={() => console.log("Export complete")}
      onCancel={() => console.log("Export cancelled")}
    />
    <ExportDocxModal
      bind:isOpen={showExportDocx}
      {bookId}
      bookTitle={meta.title}
      onDone={() => console.log("DOCX export complete")}
      onCancel={() => console.log("DOCX export cancelled")}
    />
  {/if}
{/if}

<style>
  .book-home {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .book-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border);
  }

  .back {
    text-decoration: none;
    font-size: 1.25rem;
  }

  h1 {
    flex: 1;
    margin: 0;
    font-size: 1.1rem;
  }

  .tabs button {
    border: none;
    background: transparent;
    padding: 0.35rem 0.75rem;
    color: var(--text-muted);
  }

  .tabs button.active {
    color: var(--accent);
    font-weight: 600;
    border-bottom: 2px solid var(--accent);
  }

  .search-bar {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid var(--border);
  }

  .search-bar input {
    flex: 1;
    padding: 0.4rem 0.6rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-elevated);
    color: var(--text);
  }

  .compile-panel {
    margin: 0.5rem 1rem;
    padding: 0.75rem 1rem;
  }

  .compile-panel ul {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0 0;
  }

  .book-body {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  .page-sidebar {
    width: 200px;
    border-right: 1px solid var(--border);
    padding: 0.5rem;
    overflow: auto;
    flex-shrink: 0;
  }

  .page-sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0 0;
  }

  .page-sidebar button {
    width: 100%;
    text-align: left;
    border: none;
    background: transparent;
    padding: 0.4rem 0.5rem;
    border-radius: 4px;
    color: var(--text);
    font-size: 0.85rem;
  }

  .page-sidebar button.selected {
    background: var(--accent);
    color: #fff;
  }

  .wc {
    float: right;
    opacity: 0.7;
    font-size: 0.75rem;
  }

  .editor-pane {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .read-view :global(.ProseMirror) {
    border: none !important;
    box-shadow: none !important;
  }
</style>
