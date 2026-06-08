<script lang="ts">
  import { goto } from "$app/navigation";
  import { api, type BookMeta, type PageMeta } from "$lib/api";
  import RecoveryModal from "$lib/components/RecoveryModal.svelte";
  import { onMount } from "svelte";

  let book = $state<BookMeta | null>(null);
  let pages = $state<PageMeta[]>([]);
  let showRecovery = $state(true);

  onMount(async () => {
    const params = new URLSearchParams(window.location.search);
    const bookId = params.get("bookId");

    if (!bookId) {
      await goto("/");
      return;
    }

    try {
      book = await api.getBookMeta(bookId);
      pages = await api.listPages(bookId);
    } catch (err) {
      console.error("Failed to load book:", err);
      await goto("/");
    }
  });

  async function handleKeepLatest() {
    await goto("/");
  }

  async function handleCancel() {
    await goto("/");
  }
</script>

{#if book && pages}
  <RecoveryModal
    bind:isOpen={showRecovery}
    bookId={book.id}
    pages={pages}
    onKeepLatest={handleKeepLatest}
    onCancel={handleCancel}
  />
{/if}
