<script lang="ts">
  import { goto } from "$app/navigation";
  import { api, type BookSummary } from "$lib/api";
  import FirstRunWizard from "$lib/components/FirstRunWizard.svelte";
  import ConfirmDeleteModal from "$lib/components/ConfirmDeleteModal.svelte";
  import { t } from "$lib/i18n";
  import { onMount } from "svelte";

  let books = $state<BookSummary[]>([]);
  let firstRun = $state(true);
  let loading = $state(true);
  let newTitle = $state("");
  let deleteModal = $state({ isOpen: false, bookId: "", bookTitle: "" });

  onMount(async () => {
    firstRun = await api.isFirstRun();
    if (!firstRun) {
      await refresh();
    }
    loading = false;
  });

  async function refresh() {
    books = await api.listBooks();
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
</script>

{#if loading}
  <p class="loading">…</p>
{:else if firstRun}
  <FirstRunWizard ondone={onFirstRunDone} />
{:else}
  <div class="home">
    <header class="home-header">
      <h1>{t("home.title")}</h1>
      <nav>
        <a href="/settings" class="btn">{t("settings.title")}</a>
      </nav>
    </header>

    {#if books.length === 0}
      <div class="empty-state card">
        <p>{t("home.empty")}</p>
        <p>{t("home.emptyHint")}</p>
        <button class="btn btn-primary" onclick={createBook}>{t("home.createBook")}</button>
      </div>
    {:else}
      <ul class="book-grid">
        {#each books as book (book.id)}
          <li>
            <div class="book-item card">
              <a href="/book/{book.id}" class="book-card">
                <strong>{book.title}</strong>
                <span class="meta">{book.total_words} words · {book.write_page_count} pages</span>
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
    padding: 1.5rem;
    max-width: 960px;
    margin: 0 auto;
  }

  .home-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .book-grid {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.75rem;
  }

  .book-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0;
    overflow: hidden;
  }

  .book-card {
    display: block;
    text-decoration: none;
    color: inherit;
    flex: 1;
    padding: 1rem;
  }

  .book-card:hover {
    background: var(--bg-hover);
  }

  .btn-delete {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.75rem 1rem;
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
    margin-top: 0.25rem;
  }

  .create-row {
    display: flex;
    gap: 0.5rem;
    margin-top: 1.5rem;
  }

  .create-row input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-elevated);
    color: var(--text);
  }

  .loading {
    text-align: center;
    margin-top: 4rem;
  }
</style>
