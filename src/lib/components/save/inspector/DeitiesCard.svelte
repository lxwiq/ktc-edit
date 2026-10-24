<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Item from "$lib/components/ui/item";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import { Button } from "$lib/components/ui/button";
  import { ShrineIcon } from "$lib/components/icons";
  import LockIcon from "@lucide/svelte/icons/lock";
  import LockOpenIcon from "@lucide/svelte/icons/lock-open";
  import SparklesIcon from "@lucide/svelte/icons/sparkles";
  import { setDeityStatus, unlockAllDeities, type DeityInfo } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import { deityIcon } from "./icons";

  let { deities }: { deities: DeityInfo[] } = $props();

  const editor = saveEditor;
  const t = $derived(editor.t);

  const statuses = $derived([
    { value: 0, label: t.inspector.deityStatusLocked, icon: LockIcon },
    { value: 1, label: t.inspector.deityStatusUnlocked, icon: LockOpenIcon },
    { value: 2, label: t.inspector.deityStatusActive, icon: SparklesIcon },
  ]);

  function handleSetDeityStatus(deityIndex: number, status: number) {
    editor.edit(
      (doc) => setDeityStatus(doc, {
        targetMode: editor.targetMode,
        challengeIndex: editor.selectedChallenge,
        campaignIndex: editor.selectedCampaign,
        deityIndex,
        status,
      }),
      t.status.deityUpdated
    );
  }

  function handleUnlockAllDeities(status: number = 2) {
    editor.edit(
      (doc) => unlockAllDeities(doc, {
        targetMode: editor.targetMode,
        challengeIndex: editor.selectedChallenge,
        campaignIndex: editor.selectedCampaign,
        status,
      }),
      t.status.deitiesAllUpdated
    );
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><ShrineIcon class="size-5" />{t.inspector.deitiesTitle}</Card.Title>
  </Card.Header>
  {#if deities.length > 0}
    <Card.Content>
      <Item.Group class="grid gap-3 md:grid-cols-2">
        {#each deities as deity (deity.index)}
          {@const DeityIcon = deityIcon(deity.index)}
          <Item.Root variant="outline">
            <Item.Media variant="icon">
              <DeityIcon />
            </Item.Media>
            <Item.Content>
              <Item.Title>{deity.name}</Item.Title>
              <Item.Description>
                {editor.targetMode === "campaign" ? `${t.context.island} ${deity.island}` : t.inspector.globalBuff}
              </Item.Description>
            </Item.Content>
            <Item.Footer>
              <ToggleGroup.Root
                type="single"
                variant="outline"
                size="sm"
                class="w-full"
                aria-label={deity.name}
                bind:value={
                  () => String(deity.status),
                  (next) => { if (next) handleSetDeityStatus(deity.index, Number(next)); }
                }
              >
                {#each statuses as status (status.value)}
                  <ToggleGroup.Item value={String(status.value)} class="flex-1">
                    <status.icon data-icon="inline-start" />
                    {status.label}
                  </ToggleGroup.Item>
                {/each}
              </ToggleGroup.Root>
            </Item.Footer>
          </Item.Root>
        {/each}
      </Item.Group>
    </Card.Content>
  {/if}
  <Card.Footer class="flex-wrap gap-2">
    <Button onclick={() => handleUnlockAllDeities(2)}>
      <SparklesIcon data-icon="inline-start" />
      {t.inspector.activateAllDeities}
    </Button>
    <Button variant="outline" onclick={() => handleUnlockAllDeities(1)}>
      <LockOpenIcon data-icon="inline-start" />
      {t.inspector.unlockAllDeities}
    </Button>
    <Button variant="destructive" onclick={() => handleUnlockAllDeities(0)}>
      <LockIcon data-icon="inline-start" />
      {t.inspector.lockAllDeities}
    </Button>
  </Card.Footer>
</Card.Root>
