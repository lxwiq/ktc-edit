<script lang="ts" module>
  import type { SideFilterValue } from "./SideFilter.svelte";

  // Kept outside the component so the filter survives tab switches.
  const filter = $state<{ side: SideFilterValue }>({ side: "all" });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Item from "$lib/components/ui/item";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import { Button } from "$lib/components/ui/button";
  import { HornIcon, IronWallIcon, MoundIcon, WallIcon } from "$lib/components/icons";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import { batchUpgradeWalls, upgradeSpecificWall, type WallInfo } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import SideFilter from "./SideFilter.svelte";
  import { wallIcon } from "./icons";

  let { walls }: { walls: WallInfo[] } = $props();

  const editor = saveEditor;
  const t = $derived(editor.t);

  const left = $derived(walls.filter((w) => w.side === "left").reverse());
  const right = $derived(walls.filter((w) => w.side === "right"));
  const columns = $derived([
    { side: "left" as const, title: t.inspector.leftSide, icon: ArrowLeftIcon, walls: left },
    { side: "right" as const, title: t.inspector.rightSide, icon: ArrowRightIcon, walls: right },
  ].filter((column) => filter.side === "all" || filter.side === column.side));

  function levelLabel(wall: WallInfo) {
    if (wall.level === 5 && wall.hasHorn) return t.inspector.wallLevelHorn;
    if (wall.level === 5) return t.inspector.wallLevelIron;
    if (wall.level > 0) return `${t.inspector.level} ${wall.level}`;
    return t.inspector.mound;
  }

  /** Current wall state as one of the toggle values; levels 1–4 match none of them. */
  function wallValue(wall: WallInfo) {
    if (wall.level === 5) return wall.hasHorn ? "l5h" : "l5";
    if (wall.level === 0) return "l0";
    return "";
  }

  function handleUpgradeSingleWall(wallId: string, targetLevel: number, addHorn: boolean = false) {
    editor.edit(
      (doc) => upgradeSpecificWall(doc, { ...editor.islandTarget, wallId, targetLevel, addHorn }),
      t.status.wallUpdated
    );
  }

  function handleBatchUpgradeWalls(mode: 'all' | 'built_only' | 'mounds_only', targetLevel: number = 5, addHorn: boolean = false) {
    editor.edit(
      (doc) => batchUpgradeWalls(doc, { ...editor.islandTarget, mode, targetLevel, addHorn }),
      t.status.wallsBatchUpdated
    );
  }

  function setWall(wall: WallInfo, value: string) {
    if (value === "l0") handleUpgradeSingleWall(wall.id, 0, false);
    else if (value === "l5") handleUpgradeSingleWall(wall.id, 5, false);
    else if (value === "l5h") handleUpgradeSingleWall(wall.id, 5, true);
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><WallIcon class="size-5" />{t.inspector.walls}</Card.Title>
    <Card.Description>
      {walls.filter((w) => w.level > 0).length} {t.inspector.built} · {walls.filter((w) => w.level === 0).length} {t.inspector.mounds}
    </Card.Description>
    <Card.Action>
      <SideFilter
        bind:value={filter.side}
        counts={{ all: walls.length, left: left.length, right: right.length }}
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
            {#each column.walls as wall (wall.id)}
              {@const Icon = wallIcon(wall)}
              <Item.Root variant="outline" size="sm">
                <Item.Media variant="icon">
                  <Icon />
                </Item.Media>
                <Item.Content>
                  <Item.Title class="font-mono">X = {wall.x}</Item.Title>
                  <Item.Description>{levelLabel(wall)} · {t.inspector.distance} {wall.distanceToCastle}m</Item.Description>
                </Item.Content>
                <Item.Actions>
                  <ToggleGroup.Root
                    type="single"
                    variant="outline"
                    size="sm"
                    aria-label={t.inspector.level}
                    bind:value={() => wallValue(wall), (next) => { if (next) setWall(wall, next); }}
                  >
                    <ToggleGroup.Item value="l0" aria-label={t.inspector.demoteToMound} title={t.inspector.demoteToMound}>
                      <MoundIcon />
                    </ToggleGroup.Item>
                    <ToggleGroup.Item value="l5" aria-label={t.inspector.upgradeToL5} title={t.inspector.upgradeToL5}>
                      <IronWallIcon />
                    </ToggleGroup.Item>
                    <ToggleGroup.Item value="l5h" aria-label={t.inspector.upgradeToL5Horn} title={t.inspector.upgradeToL5Horn}>
                      <HornIcon />
                    </ToggleGroup.Item>
                  </ToggleGroup.Root>
                </Item.Actions>
              </Item.Root>
            {/each}
          </Item.Group>
        </div>
      {/each}
    </div>
  </Card.Content>
  <Card.Footer class="flex-wrap gap-2">
    <Button onclick={() => handleBatchUpgradeWalls('built_only', 5, false)}>
      <IronWallIcon data-icon="inline-start" />
      {t.inspector.batchUpgradeBuilt}
    </Button>
    <Button variant="outline" onclick={() => handleBatchUpgradeWalls('all', 5, false)}>
      <IronWallIcon data-icon="inline-start" />
      {t.inspector.batchUpgradeAll}
    </Button>
  </Card.Footer>
</Card.Root>
