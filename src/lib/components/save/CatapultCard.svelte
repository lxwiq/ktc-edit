<script lang="ts" module>
  // Shared by the Inspector and Combat tabs: a value typed in one stays until it is applied.
  const custom = $state<{ left: number | null; right: number | null }>({ left: null, right: null });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Empty from "$lib/components/ui/empty";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import { CatapultIcon } from "$lib/components/icons";
  import { setCatapultOilBarrels } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  const leftCatapult = $derived(editor.islandOverview?.catapults.find(c => c.side === -1) ?? null);
  const rightCatapult = $derived(editor.islandOverview?.catapults.find(c => c.side === 1) ?? null);

  const sides = $derived([
    {
      side: -1 as const,
      key: "left" as const,
      title: t.combat.catapultLeft,
      catapult: leftCatapult,
      barrels: custom.left !== null ? custom.left : (leftCatapult?.oilBarrels ?? 30),
    },
    {
      side: 1 as const,
      key: "right" as const,
      title: t.combat.catapultRight,
      catapult: rightCatapult,
      barrels: custom.right !== null ? custom.right : (rightCatapult?.oilBarrels ?? 30),
    },
  ]);

  function handleSetCatapultBarrels(side: -1 | 1, barrels: number) {
    const applied = editor.edit(
      (doc) => setCatapultOilBarrels(doc, { ...editor.islandTarget, side, oilBarrels: barrels }),
      t.status.catapultOilUpdated
    );
    if (applied) {
      if (side === -1) custom.left = null;
      else custom.right = null;
    }
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><CatapultIcon class="size-5" />{t.combat.catapultAmmo}</Card.Title>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-4 md:grid-cols-2">
      {#each sides as entry (entry.key)}
        <div class="flex flex-col gap-3 rounded-lg border p-4">
          <div class="flex items-center justify-between gap-2">
            <span class="font-medium">{entry.title}</span>
            {#if entry.catapult}
              <Badge variant="secondary" class="font-mono">X = {entry.catapult.x}</Badge>
            {:else}
              <Badge variant="outline">{t.inspector.notBuilt}</Badge>
            {/if}
          </div>
          {#if entry.catapult}
            <div class="flex items-end gap-2">
              <NumberField
                label={t.combat.barrelsCount}
                min="0"
                max="999"
                bind:value={
                  () => entry.barrels,
                  (next) => (custom[entry.key] = Math.max(0, Number(next)))
                }
              />
              <Button variant="outline" onclick={() => handleSetCatapultBarrels(entry.side, entry.barrels)}>
                {t.combat.catapultApply}
              </Button>
            </div>
          {:else}
            <Empty.Root class="p-4">
              <Empty.Description>{t.combat.noCatapultOnIsland}</Empty.Description>
            </Empty.Root>
          {/if}
        </div>
      {/each}
    </div>
  </Card.Content>
</Card.Root>
