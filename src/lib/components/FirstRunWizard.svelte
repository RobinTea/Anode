<script lang="ts">
  import { api } from "$lib/api";
  import { t } from "$lib/i18n";
  interface Props {
    ondone?: () => void;
  }

  let { ondone }: Props = $props();

  let libraryPath = $state("");
  let loading = $state(true);

  $effect(() => {
    api.defaultLibraryPath().then((p) => {
      libraryPath = p;
      loading = false;
    });
  });

  async function continueSetup() {
    await api.initLibrary(libraryPath);
    ondone?.();
  }
</script>

<div class="wizard card">
  <h1>{t("firstRun.title")}</h1>
  <p>{t("firstRun.library")}</p>
  {#if !loading}
    <input type="text" bind:value={libraryPath} class="path-input" />
  {/if}
  <p class="hint">{t("firstRun.keybinds")}</p>
  <button class="btn btn-primary" onclick={continueSetup} disabled={loading || !libraryPath}>
    {t("firstRun.continue")}
  </button>
</div>

<style>
  .wizard {
    max-width: 480px;
    margin: 4rem auto;
    padding: 2rem;
  }

  .path-input {
    width: 100%;
    padding: 0.5rem;
    margin: 0.75rem 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg);
    color: var(--text);
  }

  .hint {
    font-size: 0.85rem;
    color: var(--text-muted);
  }
</style>
