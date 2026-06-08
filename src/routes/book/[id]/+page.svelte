<script lang="ts">
  import { page } from "$app/stores";
  import { api, type BookMeta, type CompileOrderEntry, type PageBody, type PageMeta, type Character } from "$lib/api";
  import TipTapEditor from "$lib/components/TipTapEditor.svelte";
  import CharacterManager from "$lib/components/CharacterManager.svelte";
  import NoteSidebar from "$lib/components/NoteSidebar.svelte";
  import PageOverview from "$lib/components/PageOverview.svelte";
  import ExportModal from "$lib/components/ExportModal.svelte";
  import ExportDocxModal from "$lib/components/ExportDocxModal.svelte";
  import { t } from "$lib/i18n";
  import { setWordCounts } from "$lib/stores/app.svelte";
  import { onMount } from "svelte";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

  const bookId = $derived($page.params.id ?? "");

  let meta = $state<BookMeta | null>(null);
  let pages = $state<PageMeta[]>([]);
  let compileOrder = $state<CompileOrderEntry[]>([]);
  let characters = $state<Character[]>([]);
  let activeTab = $state<"hub" | "plan" | "write" | "read" | "characters">("hub");
  let selectedPageId = $state<string | null>(null);
  let pageBody = $state<PageBody | null>(null);
  let showExport = $state(false);
  let showExportDocx = $state(false);
  let showNotes = $state(false);
  let showGridView = $state(false);
  let searchQuery = $state("");

  const writePages = $derived(pages.filter((p) => p.kind === "write"));
  const planPages = $derived(pages.filter((p) => p.kind === "plan"));
  const selectedPage = $derived(pages.find((p) => p.id === selectedPageId) ?? null);

  let searchResults = $state<PageMeta[]>([]);

  $effect(() => {
    if (searchQuery.trim().length > 2) {
      api.searchPages(bookId, searchQuery).then(res => {
        searchResults = res;
      });
    } else {
      searchResults = [];
    }
  });

  const filteredWritePages = $derived(
    searchQuery.trim().length > 2
      ? searchResults.filter(p => p.kind === "write")
      : writePages.filter((p) => p.title.toLowerCase().includes(searchQuery.toLowerCase()))
  );
  const filteredPlanPages = $derived(
    searchQuery.trim().length > 2
      ? searchResults.filter(p => p.kind === "plan")
      : planPages.filter((p) => p.title.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  onMount(() => load());

  async function load() {
    if (!bookId) return;
    meta = await api.getBookMeta(bookId);
    pages = await api.listPages(bookId);
    compileOrder = await api.getCompileOrder(bookId);
    characters = await api.listCharacters(bookId);
    setWordCounts(0, meta ? writePages.reduce((s, p) => s + p.word_count, 0) : 0);

    if (selectedPageId) {
      pageBody = await api.loadPageBody(bookId, selectedPageId);
      const pg = pages.find((p) => p.id === selectedPageId);
      if (pg) setWordCounts(pg.word_count, writePages.reduce((s, p) => s + p.word_count, 0));
    }
  }

  async function selectPage(id: string, tab: "write" | "plan" | "read") {
    selectedPageId = id;
    activeTab = tab;
    pageBody = await api.loadPageBody(bookId, id);
    const pg = pages.find((p) => p.id === id);
    if (pg) setWordCounts(pg.word_count, writePages.reduce((s, p) => s + p.word_count, 0));
  }

  async function addPage(kind: "plan" | "write") {
    const title = kind === "plan" ? "New plan" : "New Chapter";
    const created = await api.createPage(bookId, kind, kind === "plan" ? "notes" : "chapter", title);
    await load();
    await selectPage(created.id, kind);
  }

  async function handleSave(doc: Record<string, unknown>, plain: string) {
    if (!selectedPageId) return;
    const saved = await api.savePageBody(bookId, selectedPageId, doc, plain);
    pageBody = saved;
    await load();
  }

  async function handleNotesSave(notes: string) {
    if (!selectedPageId) return;
    await api.updatePageMeta(bookId, selectedPageId, { notes });
    pages = await api.listPages(bookId);
  }

  async function movePage(id: string, direction: "up" | "down") {
    const list = activeTab === "plan" ? planPages : writePages;
    const idx = list.findIndex((p) => p.id === id);
    if (idx === -1) return;
    const otherIdx = direction === "up" ? idx - 1 : idx + 1;
    if (otherIdx < 0 || otherIdx >= list.length) return;

    const a = list[idx];
    const b = list[otherIdx];
    
    await api.updatePageMeta(bookId, a.id, { sort_key: b.sort_key });
    await api.updatePageMeta(bookId, b.id, { sort_key: a.sort_key });
    await load();
  }

  function popOut() {
    if (!selectedPageId) return;
    const label = `popout-${selectedPageId}`;
    new WebviewWindow(label, {
      url: `/popout/${bookId}/${selectedPageId}`,
      title: selectedPage?.title || "Anode Popout",
      width: 800,
      height: 900,
    });
  }
</script>

{#if meta}
  <div class="book-container">
    {#if activeTab === "hub"}
      <div class="hub">
        <header class="hub-header">
          <a href="/" class="back-link">← {t("home.title")}</a>
          <div class="title-row">
            <h1>{meta.title}</h1>
            <div class="hub-actions">
              <button class="btn" onclick={() => (activeTab = "characters")}>Characters</button>
              <button class="btn" onclick={() => (showExport = true)}>{t("book.export")}</button>
              <button class="btn btn-primary" onclick={() => (showExportDocx = true)}>{t("export.docx")}</button>
            </div>
          </div>
          <div class="hub-search">
            <input bind:value={searchQuery} placeholder="Search chapters and plans..." />
          </div>
        </header>

        <div class="hub-grid">
          <section class="hub-section">
            <div class="section-header">
              <h2>Chapters</h2>
              <div class="header-actions">
                <button class="btn btn-sm" onclick={() => (showGridView = !showGridView)}>
                  {showGridView ? "List View" : "Grid View"}
                </button>
                <button class="btn btn-sm" onclick={() => addPage("write")}>+ {t("book.addWritePage")}</button>
              </div>
            </div>
            {#if filteredWritePages.length === 0}
              <div class="empty-hub-state card" role="button" tabindex="0" onclick={() => addPage("write")} onkeydown={(e) => e.key === 'Enter' && addPage("write")}>
                <p>{searchQuery ? "No matching chapters" : "No chapters yet. Click to start writing."}</p>
              </div>
            {:else if showGridView}
              <PageOverview 
                pages={filteredWritePages} 
                onSelect={(id) => selectPage(id, "write")} 
                onReorder={movePage}
              />
            {:else}
              <div class="chapter-list">
                {#each filteredWritePages as pg, i (pg.id)}
                  <div class="chapter-card-wrapper">
                    <button class="chapter-card card" onclick={() => selectPage(pg.id, "write")}>
                      <span class="chapter-title">{pg.title}</span>
                      <span class="chapter-meta">{pg.word_count} words</span>
                    </button>
                    <div class="reorder-btns">
                      <button class="reorder-btn" disabled={i === 0} onclick={() => movePage(pg.id, "up")}>▲</button>
                      <button class="reorder-btn" disabled={i === filteredWritePages.length - 1} onclick={() => movePage(pg.id, "down")}>▼</button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </section>

          <section class="hub-section">
            <div class="section-header">
              <h2>{t("book.plan")}</h2>
              <button class="btn btn-sm" onclick={() => addPage("plan")}>+ {t("book.addPlanPage")}</button>
            </div>
            <div class="plan-cards">
              <button class="plan-mode-card card" onclick={() => {
                if (planPages.length > 0) {
                  selectPage(planPages[0].id, "plan");
                } else {
                  addPage("plan");
                }
              }}>
                <div class="plan-icon">📝</div>
                <div class="plan-info">
                  <strong>Overview Mode</strong>
                  <span>Organize your plot, characters, and world-building notes</span>
                </div>
              </button>
              
              <div class="plan-stats card">
                <div class="stat-item">
                  <span class="stat-value">{planPages.length}</span>
                  <span class="stat-label">Plan Pages</span>
                </div>
                <div class="stat-item">
                  <span class="stat-value">{characters.length}</span>
                  <span class="stat-label">Characters</span>
                </div>
              </div>

              {#if filteredPlanPages.length > 0}
                <div class="mini-plan-list">
                  {#each filteredPlanPages as pg, i (pg.id)}
                    <div class="mini-card-wrapper">
                      <button class="mini-card" onclick={() => selectPage(pg.id, "plan")}>
                        <span class="dot"></span>
                        <span class="title">{pg.title}</span>
                      </button>
                      <div class="mini-reorder">
                        <button onclick={() => movePage(pg.id, "up")} disabled={i === 0}>▲</button>
                        <button onclick={() => movePage(pg.id, "down")} disabled={i === filteredPlanPages.length - 1}>▼</button>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          </section>
        </div>

        <section class="hub-section">
          <h2>Quick Actions</h2>
          <div class="action-grid">
            <button class="action-card card" onclick={() => {
              if (writePages.length > 0) {
                selectPage(writePages[0].id, "read");
              }
            }}>
              <strong>{t("book.read")}</strong>
              <span>Review your work in distraction-free mode</span>
            </button>
          </div>
        </section>
      </div>
    {:else if activeTab === "characters"}
      <div class="hub characters-tab">
        <header class="hub-header">
          <button class="btn-link" onclick={() => (activeTab = "hub")}>← Hub</button>
          <h1>Characters</h1>
        </header>
        <CharacterManager {bookId} />
      </div>
    {:else if activeTab === "read" && selectedPage && pageBody}
      <div class="read-mode">
        <button class="exit-read" onclick={() => (activeTab = "hub")}>← Exit Read Mode</button>
        <div class="read-content">
          <h1>{selectedPage.title}</h1>
          <TipTapEditor
            {bookId}
            pageId={selectedPage.id}
            initialDoc={pageBody.doc}
            editable={false}
            pagesMode={false}
            bookWordTotal={writePages.reduce((s, p) => s + p.word_count, 0)}
          />
        </div>
      </div>
    {:else}
      <div class="editor-view">
        <header class="editor-header">
          <button class="back-to-hub" onclick={() => (activeTab = "hub")}>← Hub</button>
          <div class="tabs">
            <button class:active={activeTab === "plan"} onclick={() => {
              activeTab = "plan";
              if (planPages.length > 0 && !planPages.find(p => p.id === selectedPageId)) {
                selectPage(planPages[0].id, "plan");
              }
            }}>{t("book.plan")}</button>
            <button class:active={activeTab === "write"} onclick={() => {
              activeTab = "write";
              if (writePages.length > 0 && !writePages.find(p => p.id === selectedPageId)) {
                selectPage(writePages[0].id, "write");
              }
            }}>{t("book.write")}</button>
          </div>
          <div class="editor-meta">
            <button class="btn btn-sm" onclick={popOut} title="Open in new window">⧉</button>
            <button class="btn btn-sm" onclick={() => (showNotes = !showNotes)}>
              {showNotes ? "Hide Notes" : "Quick Notes"}
            </button>
            <span class="sep">|</span>
            <strong>{meta.title}</strong>
            {#if selectedPage}
              <span class="sep">/</span>
              <span>{selectedPage.title}</span>
            {/if}
          </div>
        </header>

        <div class="editor-layout">
          <aside class="editor-sidebar">
            <div class="sidebar-header">
              <h3>{activeTab === "plan" ? "Plans" : "Chapters"}</h3>
              <button class="add-btn" onclick={() => addPage(activeTab === "plan" ? "plan" : "write")}>+</button>
            </div>
            <ul class="sidebar-list">
              {#each (activeTab === "plan" ? planPages : writePages) as pg (pg.id)}
                <li>
                  <button
                    class:selected={pg.id === selectedPageId}
                    onclick={() => selectPage(pg.id, activeTab as any)}
                  >
                    {pg.title}
                    <span class="wc">{pg.word_count}</span>
                  </button>
                </li>
              {/each}
            </ul>
          </aside>

          <main class="editor-main">
            <div class="editor-row">
              {#if selectedPage && pageBody}
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
                {#if showNotes}
                  <NoteSidebar
                    {bookId}
                    pageId={selectedPage.id}
                    initialNotes={selectedPage.notes}
                    onsave={handleNotesSave}
                  />
                {/if}
              {:else}
                <div class="empty-state">
                  <p>{activeTab === "plan" ? t("book.emptyPlan") : t("book.emptyWrite")}</p>
                  <button class="btn btn-primary" onclick={() => addPage(activeTab as "plan" | "write")}>
                    {activeTab === "plan" ? t("book.addPlanPage") : t("book.addWritePage")}
                  </button>
                </div>
              {/if}
            </div>
          </main>
        </div>
      </div>
    {/if}
  </div>
  
  <ExportModal
    bind:isOpen={showExport}
    {bookId}
    bookTitle={meta.title}
    onDone={() => (showExport = false)}
    onCancel={() => (showExport = false)}
  />
  <ExportDocxModal
    bind:isOpen={showExportDocx}
    {bookId}
    bookTitle={meta.title}
    onDone={() => (showExportDocx = false)}
    onCancel={() => (showExportDocx = false)}
  />
{/if}

<svelte:window onkeydown={(e) => {
  if (e.key === "Escape") {
    if (activeTab === "read") activeTab = "hub";
    else if (activeTab !== "hub") activeTab = "hub";
  }
}} />

<style>
  .book-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg);
  }

  /* Hub Styles */
  .hub {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
    overflow-y: auto;
  }

  .hub-header {
    margin-bottom: 2.5rem;
  }

  .back-link {
    text-decoration: none;
    color: var(--text-muted);
    font-size: 0.9rem;
    display: block;
    margin-bottom: 1rem;
  }

  .btn-link {
    background: transparent;
    border: none;
    padding: 0;
    color: var(--text-muted);
    cursor: pointer;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .hub-actions {
    display: flex;
    gap: 0.75rem;
  }

  .hub-search {
    margin-top: 1.5rem;
    max-width: 600px;
  }

  .hub-search input {
    width: 100%;
    padding: 0.6rem 1rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg-elevated);
    color: var(--text);
    font-size: 0.95rem;
  }

  .hub-grid {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2.5rem;
    margin-bottom: 2.5rem;
  }

  .hub-section h2 {
    font-size: 1.25rem;
    margin-bottom: 1rem;
    color: var(--text);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .chapter-list {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }

  .chapter-card-wrapper {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .chapter-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem 1.25rem;
    text-align: left;
    border: 1px solid var(--border);
    transition: transform 0.2s, border-color 0.2s;
  }

  .chapter-card:hover {
    border-color: var(--accent);
    background: var(--bg-elevated);
  }

  .chapter-title {
    font-weight: 600;
    margin-bottom: 0.25rem;
    display: block;
  }

  .chapter-meta {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .reorder-btns {
    display: flex;
    flex-direction: column;
    gap: 2px;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .chapter-card-wrapper:hover .reorder-btns {
    opacity: 1;
  }

  .reorder-btn {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 0.7rem;
    color: var(--text-muted);
    cursor: pointer;
  }

  .reorder-btn:hover:not(:disabled) {
    color: var(--accent);
    border-color: var(--accent);
  }

  .plan-cards {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .plan-mode-card {
    padding: 1.5rem;
    text-align: left;
    background: var(--bg-elevated);
    border: 1px dashed var(--border);
    display: flex;
    gap: 1rem;
    align-items: center;
    width: 100%;
    cursor: pointer;
  }

  .plan-icon {
    font-size: 2rem;
  }

  .plan-info strong {
    display: block;
    font-size: 1.1rem;
    margin-bottom: 0.25rem;
  }

  .plan-info span {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .plan-stats {
    display: flex;
    justify-content: space-around;
    padding: 1rem;
    background: var(--bg-elevated);
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-value {
    font-weight: 700;
    font-size: 1.25rem;
    color: var(--accent);
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .mini-plan-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .mini-card-wrapper {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .mini-card {
    flex: 1;
    padding: 0.6rem 0.75rem;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 0.85rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    text-align: left;
  }

  .mini-card .dot {
    width: 6px;
    height: 6px;
    background: var(--accent);
    border-radius: 50%;
    flex-shrink: 0;
  }

  .mini-reorder {
    display: flex;
    flex-direction: column;
    gap: 2px;
    opacity: 0;
  }

  .mini-card-wrapper:hover .mini-reorder {
    opacity: 1;
  }

  .mini-reorder button {
    background: transparent;
    border: none;
    font-size: 0.6rem;
    padding: 2px;
    color: var(--text-muted);
    cursor: pointer;
  }

  /* Editor View Styles */
  .editor-view {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .editor-header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
  }

  .back-to-hub {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 0.9rem;
  }

  .back-to-hub:hover {
    color: var(--text);
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
  }

  .tabs button {
    background: transparent;
    border: none;
    padding: 0.4rem 0.8rem;
    border-radius: 4px;
    font-weight: 500;
    color: var(--text-muted);
    cursor: pointer;
  }

  .tabs button.active {
    background: var(--bg);
    color: var(--accent);
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }

  .editor-meta {
    margin-left: auto;
    font-size: 0.85rem;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .editor-meta .sep {
    opacity: 0.5;
  }

  .editor-layout {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  .editor-sidebar {
    width: 240px;
    border-right: 1px solid var(--border);
    background: var(--bg-elevated);
    display: flex;
    flex-direction: column;
  }

  .sidebar-header {
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
  }

  .sidebar-header h3 {
    margin: 0;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .add-btn {
    background: var(--accent);
    color: white;
    border: none;
    width: 24px;
    height: 24px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.2rem;
    cursor: pointer;
  }

  .sidebar-list {
    list-style: none;
    padding: 0.5rem;
    margin: 0;
    overflow-y: auto;
  }

  .sidebar-list button {
    width: 100%;
    text-align: left;
    padding: 0.6rem 0.75rem;
    border: none;
    background: transparent;
    border-radius: 6px;
    margin-bottom: 0.25rem;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sidebar-list button:hover {
    background: var(--bg-hover);
  }

  .sidebar-list button.selected {
    background: var(--accent);
    color: white;
  }

  .editor-main {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .editor-row {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  /* Read Mode Styles */
  .read-mode {
    height: 100%;
    overflow-y: auto;
    background: var(--bg);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  .exit-read {
    position: fixed;
    top: 1rem;
    left: 1rem;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    padding: 0.5rem 1rem;
    border-radius: 20px;
    cursor: pointer;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    z-index: 100;
  }

  .read-content {
    max-width: 800px;
    width: 100%;
  }

  .read-content h1 {
    text-align: center;
    margin-bottom: 3rem;
    font-family: serif;
    font-size: 2.5rem;
  }

  .btn-sm {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
  }

  .empty-hub-state {
    padding: 3rem;
    text-align: center;
    border: 2px dashed var(--border);
    cursor: pointer;
  }
</style>
