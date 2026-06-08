<script lang="ts">
  import { goto } from "$app/navigation";
  import { api, type BookSummary } from "$lib/api";
  import FirstRunWizard from "$lib/components/FirstRunWizard.svelte";
  import ConfirmDeleteModal from "$lib/components/ConfirmDeleteModal.svelte";
  import TodoList from "$lib/components/TodoList.svelte";
  import QuestPanel from "$lib/components/QuestPanel.svelte";
  import StreakHeatmap from "$lib/components/StreakHeatmap.svelte";
  import { t } from "$lib/i18n";
  import { open } from "@tauri-apps/plugin-dialog";
  import { setWordCounts } from "$lib/stores/app.svelte";
  import { onMount } from "svelte";

  let books = $state<BookSummary[]>([]);
  let firstRun = $state(true);
  let loading = $state(true);
  let newTitle = $state("");
  let deleteModal = $state({ isOpen: false, bookId: "", bookTitle: "" });
  let searchQuery = $state("");

  const filteredBooks = $derived(
    books.filter((b) => 
      b.title.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  onMount(async () => {
    firstRun = await api.isFirstRun();
    if (!firstRun) {
      await refresh();
    }
    loading = false;
  });

  async function refresh() {
    books = await api.listBooks();
    const totalWords = books.reduce((sum, b) => sum + b.total_words, 0);
    setWordCounts(0, totalWords);
  }

  async function onFirstRunDone() {
    firstRun = false;
    await refresh();
  }

  async function createBook() {
    const title = newTitle.trim() || "Untitled";
    const book = await api.createBook(title);
    newTitle = "";
    await refresh();
    goto(`/book/${book.id}`);
  }

  function openDeleteModal(book: BookSummary) {
    deleteModal = { isOpen: true, bookId: book.id, bookTitle: book.title };
  }

  function closeDeleteModal() {
    deleteModal = { isOpen: false, bookId: "", bookTitle: "" };
  }

  async function confirmDelete() {
    const bookId = deleteModal.bookId;
    closeDeleteModal();
    await api.deleteBook(bookId);
    await refresh();
  }

  async function importBook() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Anode Files", extensions: ["anode"] }],
    });
    if (selected) {
      await api.importBook(selected as string);
      await refresh();
    }
  }
</script>

{#if loading}
  <p class="loading">…</p>
{:else if firstRun}
  <FirstRunWizard ondone={onFirstRunDone} />
{:else}
  <div class="home">
    <header class="home-header full-width">
      <h1>{t("home.title")}</h1>
      <nav>
        <a href="/settings" class="btn">{t("settings.title")}</a>
        <button class="btn" onclick={importBook}>{t("book.importShared")}</button>
      </nav>
    </header>

    <div class="home-search full-width">
      <input bind:value={searchQuery} placeholder={t("home.search")} />
    </div>

    <main class="main-content">
      {#if filteredBooks.length === 0}
        <div class="empty-state card">
          <p>{books.length === 0 ? t("home.empty") : "No matching books"}</p>
          {#if books.length === 0}
            <p>{t("home.emptyHint")}</p>
          {/if}
          <button class="btn btn-primary" onclick={createBook}>{t("home.createBook")}</button>
        </div>
      {:else}
        <ul class="book-grid">
          {#each filteredBooks as book (book.id)}
            <li>
              <div class="book-item card">
                <a href="/book/{book.id}" class="book-card">
                  <strong>{book.title}</strong>
                  <span class="meta">{book.total_words} words · {book.write_page_count} chapters</span>
                </a>
                <button
                  class="btn-delete"
                  title="Delete book"
                  onclick={() => openDeleteModal(book)}
                  aria-label="Delete {book.title}"
                >
                  ✕
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}

      <form class="create-row" onsubmit={(e) => { e.preventDefault(); createBook(); }}>
        <input bind:value={newTitle} placeholder="New book title…" />
        <button type="submit" class="btn btn-primary">{t("home.createBook")}</button>
      </form>
    </main>

    <aside class="sidebar">
      <StreakHeatmap />
      <QuestPanel />
      <TodoList />
    </aside>
  </div>
  
  <ConfirmDeleteModal
    bind:isOpen={deleteModal.isOpen}
    bookTitle={deleteModal.bookTitle}
    onConfirm={confirmDelete}
    onCancel={closeDeleteModal}
  />
{/if}

<style>
  .home {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 2rem;
    align-items: start;
  }

  .full-width {
    grid-column: 1 / -1;
  }

  .home-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .main-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .book-grid {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }

  .book-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: 8px;
    transition: transform 0.2s;
  }

  .book-item:hover {
    transform: translateX(4px);
    border-color: var(--accent);
  }

  .book-card {
    display: block;
    text-decoration: none;
    color: inherit;
    flex: 1;
    padding: 1.25rem;
  }

  .book-card strong {
    font-size: 1.1rem;
    display: block;
    margin-bottom: 0.25rem;
  }

  .book-card:hover {
    background: var(--bg-elevated);
  }

  .btn-delete {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 1rem;
    font-size: 1.2rem;
    transition: color 0.2s;
    flex-shrink: 0;
  }

  .btn-delete:hover {
    color: #dc2626;
  }

  .meta {
    display: block;
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .create-row {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .create-row input {
    flex: 1;
    padding: 0.6rem 1rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg-elevated);
    color: var(--text);
  }

  .loading {
    text-align: center;
    margin-top: 4rem;
  }

  .home-search {
    margin-bottom: 1rem;
  }

  .home-search input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--border);
    border-radius: 10px;
    background: var(--bg-elevated);
    color: var(--text);
    font-size: 1rem;
    box-shadow: 0 2px 10px rgba(0,0,0,0.05);
  }

  .sidebar {
    position: sticky;
    top: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  @media (max-width: 900px) {
    .home {
      grid-template-columns: 1fr;
    }
    .sidebar {
      position: static;
    }
  }
</style>
