<script lang="ts">
  import { api } from "$lib/api";
  import { t } from "$lib/i18n";
  import { onMount } from "svelte";

  let libraryPath = $state("");
  let theme = $state("system");

  onMount(async () => {
    const config = await api.getConfig();
    libraryPath = config.library_path ?? (await api.defaultLibraryPath());
    theme = localStorage.getItem("anode-theme") ?? config.theme ?? "system";
  });

  function setTheme(value: string) {
    theme = value;
    localStorage.setItem("anode-theme", value);
    if (value === "system") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      document.documentElement.setAttribute("data-theme", value);
    }
  }
</script>

<div class="settings">
  <header>
    <a href="/">←</a>
    <h1>{t("settings.title")}</h1>
  </header>

  <section class="card">
    <label>
      {t("settings.libraryPath")}
      <input type="text" value={libraryPath} readonly />
    </label>
  </section>

  <section class="card">
    <p>{t("settings.theme")}</p>
    <div class="theme-row">
      {#each ["system", "light", "dark", "sepia"] as th}
        <button class="btn" class:active={theme === th} onclick={() => setTheme(th)}>{th}</button>
      {/each}
    </div>
  </section>
</div>

<style>
  .settings {
    padding: 1.5rem;
    max-width: 560px;
    margin: 0 auto;
  }

  header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  label {
    display: block;
    font-size: 0.9rem;
  }

  input {
    width: 100%;
    margin-top: 0.35rem;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg);
    color: var(--text);
  }

  section {
    margin-bottom: 1rem;
  }

  .theme-row {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin-top: 0.5rem;
  }

  .theme-row .active {
    border-color: var(--accent);
    color: var(--accent);
  }
</style>
