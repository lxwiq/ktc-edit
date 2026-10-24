<script lang="ts">
  import { onMount, type Component } from "svelte";
  import * as Card from "$lib/components/ui/card";
  import * as Item from "$lib/components/ui/item";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Spinner } from "$lib/components/ui/spinner";
  import UpdateBanner from "$lib/components/UpdateBanner.svelte";
  import {
    BoatIcon,
    CastleIcon,
    CoinIcon,
    KingdomEmblem,
    KingdomHorizon,
    KnightHelmIcon,
  } from "$lib/components/icons";
  import SwordsIcon from "@lucide/svelte/icons/swords";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import MonitorIcon from "@lucide/svelte/icons/monitor";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import FileSearchIcon from "@lucide/svelte/icons/file-search";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import { assetEditor } from "$lib/state/asset-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  function getDefaultSavePath(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      return `~/Library/Application Support/nl.noio.kingdom-two-crowns/Release/global-v35`;
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return `C:\\Users\\USERNAME\\AppData\\LocalLow\\noio\\KingdomTwoCrowns\\Release\\global-v35`;
    } else {
      return `~/.config/unity3d/noio/KingdomTwoCrowns/Release/global-v35`;
    }
  }

  function getOSName(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      return 'macOS';
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return 'Windows';
    } else if (platform.includes('linux') || userAgent.includes('linux')) {
      return 'Linux';
    } else {
      return 'Unknown system';
    }
  }

  const detectedOS = getOSName();
  const defaultPath = getDefaultSavePath();
  const savePath = $derived(editor.detectedSavePath ?? defaultPath);

  onMount(() => {
    editor.detectSave();
  });

  const features = $derived<{ icon: Component; title: string; desc: string }[]>([
    { icon: CoinIcon, ...t.welcome.features.resources },
    { icon: BoatIcon, ...t.welcome.features.navigation },
    { icon: SwordsIcon, ...t.welcome.features.combat },
    { icon: CastleIcon, ...t.welcome.features.construction },
    { icon: KnightHelmIcon, ...t.welcome.features.recruitment },
  ]);
</script>

<div class="relative flex min-h-svh flex-col">
  <div class="pointer-events-none absolute inset-x-0 top-0 h-72 overflow-hidden md:h-80">
    <KingdomHorizon class="size-full" />
  </div>

  <main class="relative mx-auto flex w-full max-w-4xl flex-1 flex-col items-center gap-8 px-4 pt-40 pb-12 md:pt-48">
    <UpdateBanner class="w-full" />

    <div class="flex flex-col items-center gap-4 text-center">
      <KingdomEmblem class="size-20 drop-shadow-lg" />
      <div class="flex flex-col gap-2">
        <h1 class="font-heading text-primary text-3xl font-bold tracking-wide text-balance md:text-4xl">
          {t.welcome.title}
        </h1>
        <p class="text-muted-foreground text-lg">{t.welcome.subtitle}</p>
      </div>
    </div>

    <Card.Root class="w-full max-w-2xl">
      <Card.Header>
        <Card.Title>{editor.detectedSavePath ? t.welcome.detected : t.welcome.defaultPath}</Card.Title>
        <Card.Description>{editor.detectedSavePath ? t.welcome.detectedHint : t.welcome.notFound}</Card.Description>
        <Card.Action>
          {#if editor.detectedSavePath}
            <Badge>
              <CircleCheckIcon data-icon="inline-start" />
              {detectedOS}
            </Badge>
          {:else}
            <Badge variant="secondary">
              <MonitorIcon data-icon="inline-start" />
              {detectedOS}
            </Badge>
          {/if}
        </Card.Action>
      </Card.Header>
      <Card.Content>
        <code class="bg-muted block rounded-md px-3 py-2 font-mono text-sm break-all">{savePath}</code>
      </Card.Content>
      <Card.Footer class="flex-wrap gap-2">
        <Button size="lg" onclick={() => editor.openSave()} disabled={editor.isLoading}>
          {#if editor.isLoading}
            <Spinner data-icon="inline-start" />
            {t.welcome.loading}
          {:else}
            <FolderOpenIcon data-icon="inline-start" />
            {t.welcome.openFile}
          {/if}
        </Button>
        {#if editor.detectedSavePath}
          <Button size="lg" variant="outline" onclick={() => editor.selectFile()} disabled={editor.isLoading}>
            <FileSearchIcon data-icon="inline-start" />
            {t.welcome.chooseOther}
          </Button>
        {/if}
        <Button size="lg" variant="outline" onclick={() => assetEditor.open(false)} disabled={assetEditor.busy}>
          <SlidersHorizontalIcon data-icon="inline-start" />
          {t.welcome.editAssets}
        </Button>
      </Card.Footer>
    </Card.Root>

    <Item.Group class="grid w-full grid-cols-[repeat(auto-fit,minmax(10rem,1fr))] gap-3">
      {#each features as feature (feature.title)}
        <Item.Root variant="outline" class="flex-col items-start">
          <Item.Media variant="icon">
            <feature.icon />
          </Item.Media>
          <Item.Content>
            <Item.Title>{feature.title}</Item.Title>
            <Item.Description class="line-clamp-none">{feature.desc}</Item.Description>
          </Item.Content>
        </Item.Root>
      {/each}
    </Item.Group>
  </main>
</div>
