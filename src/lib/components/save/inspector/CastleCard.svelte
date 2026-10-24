<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Separator } from "$lib/components/ui/separator";
  import StatTile from "$lib/components/common/StatTile.svelte";
  import { CastleIcon, CrownIcon, IronWallIcon, TreeIcon, WallIcon } from "$lib/components/icons";
  import { upgradeIslandCastle, type IslandOverview } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import { specialPointIcon } from "./icons";

  let { overview }: { overview: IslandOverview } = $props();

  const editor = saveEditor;
  const t = $derived(editor.t);

  function handleUpgradeCastle(targetLevel: number = 7) {
    editor.edit(
      (doc) => upgradeIslandCastle(doc, { ...editor.islandTarget, targetLevel }),
      t.status.castleUpdated
    );
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><CastleIcon class="size-5" />{t.inspector.castleInfo}</Card.Title>
    <Card.Description>{overview.castleName} · {t.inspector.level} {overview.castleLevel}</Card.Description>
  </Card.Header>
  <Card.Content class="flex flex-col gap-4">
    <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
      <StatTile
        label={t.inspector.castlePosition}
        value={`X = ${overview.castleX}`}
        hint={`${overview.castleName} (${t.inspector.level} ${overview.castleLevel})`}
        icon={CastleIcon}
      />
      {#each overview.players as player (player.name)}
        <StatTile
          label={`${player.name} · ${t.inspector.playerPosition}`}
          value={`X = ${player.x}`}
          hint={`Y = ${player.y} · ${t.inspector.distance} ${Math.round(Math.abs(player.x - overview.castleX) * 100) / 100}m`}
          icon={CrownIcon}
        />
      {/each}
      <StatTile
        label={t.inspector.totalWalls}
        value={overview.walls.length}
        hint={`${overview.walls.filter((w) => w.level > 0).length} ${t.inspector.built} / ${overview.walls.filter((w) => w.level === 0).length} ${t.inspector.mounds}`}
        icon={WallIcon}
      />
      <StatTile
        label={t.inspector.totalTrees}
        value={overview.trees.length}
        hint={`${overview.trees.filter((tr) => tr.marked).length} ${t.inspector.markedToCut}`}
        icon={TreeIcon}
      />
    </div>

    {#if overview.specialPoints.length > 0}
      <Separator />
      <div class="flex flex-col gap-2">
        <span class="text-muted-foreground text-xs font-medium tracking-wide uppercase">{t.inspector.specialPoints}</span>
        <div class="flex flex-wrap gap-2">
          {#each overview.specialPoints as point, index (index)}
            {@const PointIcon = specialPointIcon(point.type)}
            <Badge variant="outline">
              <PointIcon data-icon="inline-start" />
              {point.name}
              <span class="text-muted-foreground font-mono">X = {point.x}</span>
            </Badge>
          {/each}
        </div>
      </div>
    {/if}
  </Card.Content>
  <Card.Footer class="flex-wrap gap-2">
    <Button variant="outline" disabled={overview.castleLevel >= 4} onclick={() => handleUpgradeCastle(4)}>
      <WallIcon data-icon="inline-start" />
      {t.inspector.upgradeCastleStone}
    </Button>
    <Button disabled={overview.castleLevel >= 7} onclick={() => handleUpgradeCastle(7)}>
      <IronWallIcon data-icon="inline-start" />
      {t.inspector.upgradeCastleIron}
    </Button>
  </Card.Footer>
</Card.Root>
