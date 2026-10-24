<script lang="ts" module>
  import type { SideFilterValue } from "./SideFilter.svelte";

  // Kept outside the component so the filter survives tab switches.
  const filter = $state<{ side: SideFilterValue }>({ side: "all" });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Item from "$lib/components/ui/item";
  import { Button } from "$lib/components/ui/button";
  import OptionSelect from "$lib/components/common/OptionSelect.svelte";
  import { ArcherTowerIcon, BowIcon } from "$lib/components/icons";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import { batchUpgradeTowers, upgradeSpecificTower, type TowerInfo, type TowerType } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import SideFilter from "./SideFilter.svelte";
  import { towerIcon } from "./icons";

  let { towers }: { towers: TowerInfo[] } = $props();

  const editor = saveEditor;
  const t = $derived(editor.t);

  const left = $derived(towers.filter((tw) => tw.side === "left").reverse());
  const right = $derived(towers.filter((tw) => tw.side === "right"));
  const columns = $derived([
    { side: "left" as const, title: t.inspector.leftSide, icon: ArrowLeftIcon, towers: left },
    { side: "right" as const, title: t.inspector.rightSide, icon: ArrowRightIcon, towers: right },
  ].filter((column) => filter.side === "all" || filter.side === column.side));

  const typeOptions = $derived<{ value: TowerType; label: string }[]>([
    { value: "tower6", label: t.inspector.upgradeToL6 },
    { value: "tower5", label: t.inspector.upgradeToL5Roof },
    { value: "ballista", label: t.inspector.upgradeToBallista },
    { value: "baker", label: t.inspector.upgradeToBaker },
    { value: "knight", label: t.inspector.upgradeToKnightTower },
    { value: "tower0", label: t.inspector.mound },
  ]);

  function handleUpgradeSingleTower(towerId: string, targetType: TowerType) {
    editor.edit(
      (doc) => upgradeSpecificTower(doc, { ...editor.islandTarget, towerId, targetType }),
      t.status.towerUpdated
    );
  }

  function handleBatchUpgradeTowers(mode: 'all' | 'built_only' | 'mounds_only', targetType: TowerType) {
    editor.edit(
      (doc) => batchUpgradeTowers(doc, { ...editor.islandTarget, mode, targetType }),
      t.status.towersBatchUpdated
    );
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><ArcherTowerIcon class="size-5" />{t.inspector.towers}</Card.Title>
    <Card.Action>
      <SideFilter
        bind:value={filter.side}
        counts={{ all: towers.length, left: left.length, right: right.length }}
      />
    </Card.Action>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-6 {columns.length > 1 ? 'lg:grid-cols-2' : ''}">
      {#each columns as column (column.side)}
        <div class="flex min-w-0 flex-col gap-3">
          <h3 class="text-muted-foreground flex items-center gap-2 text-sm font-medium">
            <column.icon class="size-4" />
            {column.title}
          </h3>
          <Item.Group class="gap-2">
            {#each column.towers as tower (tower.id)}
              {@const Icon = towerIcon(tower.type)}
              <Item.Root variant="outline" size="sm">
                <Item.Media variant="icon">
                  <Icon />
                </Item.Media>
                <Item.Content>
                  <Item.Title class="font-mono">X = {tower.x}</Item.Title>
                  <Item.Description>{t.inspector.distance} {tower.distanceToCastle}m</Item.Description>
                </Item.Content>
                <Item.Actions>
                  <OptionSelect
                    size="sm"
                    class="w-44"
                    aria-label={t.inspector.towers}
                    options={typeOptions}
                    placeholder={tower.typeLabel}
                    bind:value={
                      () => (typeOptions.some((option) => option.value === tower.type) ? tower.type : ""),
                      (next) => { if (next && next !== tower.type) handleUpgradeSingleTower(tower.id, next as TowerType); }
                    }
                  />
                </Item.Actions>
              </Item.Root>
            {/each}
          </Item.Group>
        </div>
      {/each}
    </div>
  </Card.Content>
  <Card.Footer class="flex-wrap gap-2">
    <Button variant="outline" onclick={() => handleBatchUpgradeTowers('built_only', 'tower5')}>
      <ArcherTowerIcon data-icon="inline-start" />
      {t.inspector.batchUpgradeTowersRoof}
    </Button>
    <Button onclick={() => handleBatchUpgradeTowers('built_only', 'tower6')}>
      <BowIcon data-icon="inline-start" />
      {t.inspector.batchUpgradeTowersL6}
    </Button>
  </Card.Footer>
</Card.Root>
