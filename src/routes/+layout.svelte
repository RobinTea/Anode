<script lang="ts">
  import "../app.css";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import HelpModal from "$lib/components/HelpModal.svelte";
  import { tickClock } from "$lib/stores/app.svelte";
  import { undoStore, undoRestoreData } from "$lib/stores/undo.svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  let { children } = $props();
  let helpOpen = $state(false);

  onMount(() => {
    const theme = localStorage.getItem("anode-theme") ?? "system";
    applyTheme(theme);
    const interval = setInterval(tickClock, 30_000);
    tickClock();
    
    // Handle keyboard shortcuts
    const handleKeydown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "h") {
        e.preventDefault();
        helpOpen = !helpOpen;
      }
      if (e.key === "F1") {
        e.preventDefault();
        helpOpen = !helpOpen;
      }
      if ((e.ctrlKey || e.metaKey) && e.key === "z" && !e.shiftKey) {
        e.preventDefault();
        handleUndo();
      } else if ((e.ctrlKey || e.metaKey) && (e.key === "y" || (e.key === "z" && e.shiftKey))) {
        e.preventDefault();
        handleRedo();
      }
    };

    window.addEventListener("keydown", handleKeydown);
    return () => {
      clearInterval(interval);
      window.removeEventListener("keydown", handleKeydown);
    };
  });

  function applyTheme(theme: string) {
    if (theme === "system") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      document.documentElement.setAttribute("data-theme", theme);
    }
  }

  async function handleUndo() {
    const entry = undoStore.undo();
    if (entry) {
      // Trigger restore
      undoRestoreData.set(entry);
    }
  }

  async function handleRedo() {
    const entry = undoStore.redo();
    if (entry) {
      // Trigger restore
      undoRestoreData.set(entry);
    }
  }
</script>

<div class="app-shell">
  <main class="app-main">
    {@render children()}
  </main>
  <StatusBar />
</div>

<HelpModal bind:isOpen={helpOpen} onClose={() => helpOpen = false} />
