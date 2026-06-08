<script lang="ts">
  import { api } from "$lib/api";
  import { t } from "$lib/i18n";
  import { save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  let libraryPath = $state("");
  let theme = $state("system");
  let isBackingUp = $state(false);
  let customBg = $state("");
  let customText = $state("");
  let customAccent = $state("");
  let useCustom = $state(false);

  async function backupLibrary() {
    isBackingUp = true;
    try {
      const path = await save({
        defaultPath: `anode-backup-${new Date().toISOString().slice(0, 10)}.anode`,
        filters: [{ name: "Anode Backup", extensions: ["anode"] }],
      });
      if (path) {
        await api.exportBook("library-backup", true, path);
      }
    } catch (err) {
      console.error("Backup failed:", err);
    } finally {
      isBackingUp = false;
    }
  }

  function applyCustomTheme() {
    if (theme !== "custom") setTheme("custom");
    document.documentElement.style.setProperty("--bg", customBg);
    document.documentElement.style.setProperty("--bg-elevated", adjustColor(customBg, 10));
    document.documentElement.style.setProperty("--text", customText);
    document.documentElement.style.setProperty("--accent", customAccent);
    document.documentElement.style.setProperty("--border", adjustColor(customBg, -15));
    document.documentElement.style.setProperty("--status-bg", adjustColor(customBg, -5));
    
    localStorage.setItem("anode-custom-bg", customBg);
    localStorage.setItem("anode-custom-text", customText);
    localStorage.setItem("anode-custom-accent", customAccent);
  }

  function adjustColor(hex: string, percent: number): string {
    const num = parseInt(hex.replace("#", ""), 16),
      amt = Math.round(2.55 * percent),
      R = (num >> 16) + amt,
      G = (num >> 8 & 0x00FF) + amt,
      B = (num & 0x0000FF) + amt;
    return "#" + (0x1000000 + (R < 255 ? R < 1 ? 0 : R : 255) * 0x10000 + (G < 255 ? G < 1 ? 0 : G : 255) * 0x100 + (B < 255 ? B < 1 ? 0 : B : 255)).toString(16).slice(1);
  }

  onMount(async () => {
    const config = await api.getConfig();
    libraryPath = config.library_path ?? (await api.defaultLibraryPath());
    
    customBg = localStorage.getItem("anode-custom-bg") || "#ffffff";
    customText = localStorage.getItem("anode-custom-text") || "#1a1a1a";
    customAccent = localStorage.getItem("anode-custom-accent") || "#3d5a80";
    
    theme = localStorage.getItem("anode-theme") ?? config.theme ?? "system";
    if (theme === "custom") {
      useCustom = true;
      applyCustomTheme();
    } else {
      setTheme(theme);
    }
  });

  function setTheme(value: string) {
    theme = value;
    localStorage.setItem("anode-theme", value);
    if (value === "system") {
      document.documentElement.removeAttribute("data-theme");
      document.documentElement.removeAttribute("style");
    } else if (value === "custom") {
      document.documentElement.setAttribute("data-theme", "custom");
      applyCustomTheme();
    } else {
      document.documentElement.setAttribute("data-theme", value);
      document.documentElement.removeAttribute("style");
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
        <button class="btn" class:active={theme === th} onclick={() => { setTheme(th); useCustom = false; }}>{th}</button>
      {/each}
    </div>
    <label class="custom-theme">
      <input type="checkbox" bind:checked={useCustom} onchange={() => { if (useCustom) setTheme("custom"); }} />
      {t("settings.customTheme")}
    </label>
    {#if useCustom}
      <div class="color-pickers">
        <div class="color-picker">
          <label for="bg-color">{t("settings.bgColor")}</label>
          <input id="bg-color" type="color" bind:value={customBg} onchange={() => applyCustomTheme()} />
        </div>
        <div class="color-picker">
          <label for="text-color">{t("settings.textColor")}</label>
          <input id="text-color" type="color" bind:value={customText} onchange={() => applyCustomTheme()} />
        </div>
        <div class="color-picker">
          <label for="accent-color">{t("settings.accentColor")}</label>
          <input id="accent-color" type="color" bind:value={customAccent} onchange={() => applyCustomTheme()} />
        </div>
      </div>
    {/if}
  </section>

  <section class="card">
    <button class="btn btn-primary" onclick={backupLibrary} disabled={isBackingUp}>
      {isBackingUp ? t("loading") : t("settings.backup")}
    </button>
  </section>

  <section class="card">
    <details>
      <summary>{t("settings.keybinds")}</summary>
      <table class="keybinds-table">
        <tbody>
          <tr><td><kbd>Ctrl</kbd>+<kbd>S</kbd></td><td>{t("keybinds.save")}</td></tr>
          <tr><td><kbd>Esc</kbd></td><td>{t("keybinds.close")}</td></tr>
        </tbody>
      </table>
    </details>
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

  .custom-theme {
    display: block;
    margin-top: 0.75rem;
    font-size: 0.9rem;
  }

  .custom-theme input[type="checkbox"] {
    margin-right: 0.5rem;
    vertical-align: middle;
  }

  .color-pickers {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.75rem;
  }

  .color-picker {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .color-picker label {
    font-size: 0.85rem;
    white-space: nowrap;
  }

  .color-picker input[type="color"] {
    width: 40px;
    height: 32px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
  }

  section.card + section.card {
    margin-top: 1rem;
  }

  .keybinds-table {
    width: 100%;
    margin-top: 0.5rem;
  }

  .keybinds-table td {
    padding: 0.35rem 0;
    border-bottom: 1px solid var(--border);
  }

  .keybinds-table td:first-child {
    width: 40%;
    text-align: right;
    padding-right: 1rem;
  }

  .keybinds-table td:last-child {
    text-align: left;
  }

  kbd {
    display: inline-block;
    padding: 0.15rem 0.35rem;
    font-size: 0.8em;
    font-family: monospace;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 3px;
    margin: 0 0.15rem;
  }

  details summary {
    cursor: pointer;
    font-weight: 600;
    padding: 0.25rem 0;
  }

  details summary::marker {
    content: "▼ ";
  }

  details[open] summary::marker {
    content: "▲ ";
  }
</style>
