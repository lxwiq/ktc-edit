<script lang="ts">
  import type { Component } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import * as Field from "$lib/components/ui/field";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import OptionSelect from "$lib/components/common/OptionSelect.svelte";
  import {
    BoatIcon,
    CastleIcon,
    CoinIcon,
    IslandIcon,
    KingdomEmblem,
    KnightHelmIcon,
  } from "$lib/components/icons";
  import SwordsIcon from "@lucide/svelte/icons/swords";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import FileIcon from "@lucide/svelte/icons/file";
  import ArchiveIcon from "@lucide/svelte/icons/archive";
  import { saveEditor, type SaveTab, type TargetMode } from "$lib/state/save-editor.svelte";
  import { assetEditor } from "$lib/state/asset-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  const tabs = $derived<{ id: SaveTab; label: string; icon: Component; campaignOnly?: boolean }[]>([
    { id: "inspector", label: t.tabs.inspector, icon: IslandIcon },
    { id: "resources", label: t.tabs.resources, icon: CoinIcon },
    { id: "navigation", label: t.tabs.navigation, icon: BoatIcon, campaignOnly: true },
    { id: "combat", label: t.tabs.combat, icon: SwordsIcon },
    { id: "construction", label: t.tabs.construction, icon: CastleIcon, campaignOnly: true },
    { id: "recruitment", label: t.tabs.recruitment, icon: KnightHelmIcon },
  ]);
  const visibleTabs = $derived(tabs.filter((tab) => !tab.campaignOnly || editor.targetMode === "campaign"));

  const campaignOptions = $derived(editor.campaigns.map((campaign, index) => ({ value: String(index), label: campaign.label })));
  const islandOptions = $derived(
    Array.from({ length: Math.max(0, editor.islands) }, (_, index) => ({
      value: String(index),
      label: `${t.context.island} ${index + 1}`,
    }))
  );
  const challengeOptions = $derived(editor.challenges.map((challenge, index) => ({ value: String(index), label: challenge.label })));

  function splitPath(path: string) {
    const index = Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\"));
    return { name: path.slice(index + 1), directory: path.slice(0, index) };
  }

  const file = $derived(editor.filePath ? splitPath(editor.filePath) : null);
  const backupName = $derived(editor.backupPath ? splitPath(editor.backupPath).name : null);

  function openTab(id: SaveTab) {
    assetEditor.close();
    editor.activeTab = id;
    window.scrollTo({ top: 0 });
  }

  function openAssets() {
    if (assetEditor.active || editor.isLoading || assetEditor.busy) return;
    assetEditor.open(false);
    window.scrollTo({ top: 0 });
  }

  async function openSave() {
    if (editor.isLoading) return;
    await editor.openSave();
    if (editor.data) assetEditor.close();
  }
</script>

<Sidebar.Root collapsible="icon">
  <Sidebar.Header>
    <div class="flex h-12 items-center gap-2 overflow-hidden px-1 group-data-[collapsible=icon]:px-0">
      <KingdomEmblem class="size-8 shrink-0" />
      <div class="grid flex-1 leading-tight group-data-[collapsible=icon]:hidden">
        <span class="font-heading truncate text-sm font-semibold tracking-wide">Kingdom Two Crowns</span>
        <span class="text-muted-foreground truncate text-xs">Save & Asset Editor</span>
      </div>
    </div>
  </Sidebar.Header>

  <Sidebar.Content>
    {#if editor.data}
      <Sidebar.Group class="group-data-[collapsible=icon]:hidden">
        <Sidebar.GroupLabel>{t.context.title}</Sidebar.GroupLabel>
        <Sidebar.GroupContent class="px-2">
          <Field.Group class="gap-3">
            <ToggleGroup.Root
              type="single"
              variant="outline"
              size="sm"
              class="w-full"
              aria-label={t.context.mode}
              bind:value={() => editor.targetMode, (next) => { if (next) editor.setTargetMode(next as TargetMode); }}
            >
              <ToggleGroup.Item value="campaign" class="flex-1">{t.context.campaignShort}</ToggleGroup.Item>
              <ToggleGroup.Item value="challenge" class="flex-1">{t.context.challengeShort}</ToggleGroup.Item>
            </ToggleGroup.Root>

            {#if editor.targetMode === "campaign"}
              <Field.Field class="gap-1.5">
                <Field.Label for="context-campaign">{t.context.campaign}</Field.Label>
                <OptionSelect
                  id="context-campaign"
                  size="sm"
                  class="w-full"
                  options={campaignOptions}
                  bind:value={() => String(editor.selectedCampaign), (next) => (editor.selectedCampaign = Number(next))}
                />
              </Field.Field>
              <Field.Field class="gap-1.5">
                <Field.Label for="context-island">{t.context.island}</Field.Label>
                <OptionSelect
                  id="context-island"
                  size="sm"
                  class="w-full"
                  options={islandOptions}
                  bind:value={() => String(editor.selectedIsland), (next) => (editor.selectedIsland = Number(next))}
                />
              </Field.Field>
            {:else}
              <Field.Field class="gap-1.5">
                <Field.Label for="context-challenge">{t.context.challenge}</Field.Label>
                <OptionSelect
                  id="context-challenge"
                  size="sm"
                  class="w-full"
                  options={challengeOptions}
                  bind:value={() => String(editor.selectedChallenge), (next) => (editor.selectedChallenge = Number(next))}
                />
              </Field.Field>
            {/if}
          </Field.Group>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    {/if}

    <Sidebar.Group>
      <Sidebar.GroupLabel>{t.nav.saveEditor}</Sidebar.GroupLabel>
      <Sidebar.Menu>
        {#if editor.data}
          {#each visibleTabs as tab (tab.id)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                isActive={!assetEditor.active && editor.activeTab === tab.id}
                tooltipContent={tab.label}
                onclick={() => openTab(tab.id)}
              >
                <tab.icon />
                <span>{tab.label}</span>
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        {:else}
          <Sidebar.MenuItem>
            <Sidebar.MenuButton tooltipContent={t.welcome.openFile} aria-disabled={editor.isLoading} onclick={openSave}>
              <FolderOpenIcon />
              <span>{t.welcome.openFile}</span>
            </Sidebar.MenuButton>
          </Sidebar.MenuItem>
        {/if}
      </Sidebar.Menu>
    </Sidebar.Group>

    <Sidebar.Group>
      <Sidebar.GroupLabel>{t.nav.game}</Sidebar.GroupLabel>
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <Sidebar.MenuButton
            isActive={assetEditor.active}
            tooltipContent={t.nav.gameAssets}
            aria-disabled={editor.isLoading || assetEditor.busy}
            onclick={openAssets}
          >
            <SlidersHorizontalIcon />
            <span>{t.nav.gameAssets}</span>
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Group>
  </Sidebar.Content>

  <Sidebar.Footer class="group-data-[collapsible=icon]:hidden">
    {#if file}
      <div class="flex flex-col gap-2 rounded-md border p-2.5 text-xs">
        <div class="flex items-center gap-2">
          <FileIcon class="text-muted-foreground size-4 shrink-0" />
          <span class="truncate font-medium">{file.name}</span>
          {#if editor.dirty}
            <span class="bg-warning ml-auto size-2 shrink-0 rounded-full" title={t.header.unsaved}></span>
          {/if}
        </div>
        <span class="text-muted-foreground truncate font-mono" title={editor.filePath}>{file.directory}</span>
        {#if backupName}
          <div class="text-muted-foreground flex items-center gap-2" title={editor.backupPath}>
            <ArchiveIcon class="size-3.5 shrink-0" />
            <span class="truncate">{backupName}</span>
          </div>
        {/if}
      </div>
    {:else}
      <p class="text-muted-foreground px-2 text-xs">{t.nav.noSave}</p>
    {/if}
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
