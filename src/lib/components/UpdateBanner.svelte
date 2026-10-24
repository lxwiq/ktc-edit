<script lang="ts">
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Progress } from "$lib/components/ui/progress";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import { detectLanguage, getTranslations } from "$lib/i18n";
  import { updater } from "$lib/state/updater.svelte";

  let { class: className }: { class?: string } = $props();

  const t = getTranslations(detectLanguage()).updater;
</script>

{#if updater.visible && updater.update}
  <Alert.Root class={className}>
    <DownloadIcon />
    <Alert.Title>{t.available.replace("{version}", updater.update.version)}</Alert.Title>
    <Alert.Description class="flex flex-col gap-3">
      <p>{t.saveFirst}</p>
      {#if updater.installing}
        <div class="flex w-full items-center gap-3">
          <Progress value={updater.percent} class="flex-1" />
          <span class="shrink-0 tabular-nums">{t.downloading}{updater.percent === null ? "" : ` ${updater.percent}%`}</span>
        </div>
      {:else}
        <div class="flex gap-2">
          <Button size="sm" onclick={() => updater.install()}>{t.install}</Button>
          <Button size="sm" variant="ghost" onclick={() => (updater.dismissed = true)}>{t.later}</Button>
        </div>
      {/if}
      {#if updater.error}
        <p class="text-destructive">{t.failed} {updater.error}</p>
      {/if}
    </Alert.Description>
  </Alert.Root>
{/if}
