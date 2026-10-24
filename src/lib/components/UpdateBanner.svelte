<script lang="ts">
  import { onMount } from "svelte";
  import { check, type Update } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { detectLanguage, getTranslations } from "$lib/i18n";

  const t = getTranslations(detectLanguage()).updater;

  let update = $state<Update | null>(null);
  let dismissed = $state(false);
  let installing = $state(false);
  let downloaded = $state(0);
  let total = $state<number | null>(null);
  let error = $state<string | null>(null);
  let percent = $derived(total ? Math.min(100, Math.round((downloaded / total) * 100)) : null);

  onMount(async () => {
    try {
      update = await check();
    } catch (err) {
      // Offline, rate-limited or no published release yet: never block the editor.
      console.warn("Update check failed", err);
    }
  });

  async function install() {
    if (!update || installing) return;
    installing = true;
    error = null;
    downloaded = 0;
    total = null;
    try {
      await update.downloadAndInstall((event) => {
        if (event.event === "Started") total = event.data.contentLength ?? null;
        else if (event.event === "Progress") downloaded += event.data.chunkLength;
      });
      await relaunch();
    } catch (err) {
      error = String(err);
      installing = false;
    }
  }
</script>

{#if update && !dismissed}
  <div class="update-banner" role="status">
    <span class="message">
      <strong>{t.available.replace("{version}", update.version)}</strong>
      {t.saveFirst}
    </span>
    {#if installing}
      <span class="progress">{t.downloading}{percent === null ? "" : ` ${percent}%`}</span>
    {:else}
      <span class="actions">
        <button type="button" onclick={install}>{t.install}</button>
        <button type="button" class="secondary" onclick={() => (dismissed = true)}>{t.later}</button>
      </span>
    {/if}
    {#if error}<span class="error">{t.failed} {error}</span>{/if}
  </div>
{/if}

<style>
  .update-banner {
    position: sticky;
    top: 0;
    z-index: 1000;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 1rem;
    padding: 0.6rem 1rem;
    background: var(--color-bg-card);
    border-bottom: 2px solid var(--color-gold);
    color: var(--color-text-primary);
    font-size: 0.9rem;
  }
  .message {
    flex: 1 1 18rem;
  }
  .message strong {
    color: var(--color-gold-light);
    margin-right: 0.35rem;
  }
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  button {
    padding: 0.35rem 0.8rem;
    border: 1px solid var(--color-gold-dark);
    border-radius: 4px;
    background: var(--color-gold);
    color: var(--color-bg-primary);
    font-weight: 600;
    cursor: pointer;
  }
  button.secondary {
    background: transparent;
    color: var(--color-text-secondary);
    border-color: var(--color-border-light);
  }
  .progress {
    color: var(--color-text-secondary);
  }
  .error {
    flex-basis: 100%;
    color: var(--color-warning);
  }
</style>
