<script lang="ts">
  import type { Component } from "svelte";
  import * as Card from "$lib/components/ui/card";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import { CastleIcon, CatapultIcon, CrownIcon, IslandIcon, TreeIcon, WallIcon, ArcherTowerIcon } from "$lib/components/icons";
  import MapPinIcon from "@lucide/svelte/icons/map-pin";
  import type { IslandOverview } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import { specialPointIcon, towerIcon, wallIcon } from "./icons";

  let { overview }: { overview: IslandOverview } = $props();

  const t = $derived(saveEditor.t);

  interface Marker {
    key: string;
    x: number;
    label: string;
    icon: Component;
    tone: "primary" | "foreground" | "muted";
  }

  const bounds = $derived.by(() => {
    const xs = [
      overview.castleX,
      ...overview.walls.map((item) => item.x),
      ...overview.towers.map((item) => item.x),
      ...overview.catapults.map((item) => item.x),
      ...overview.trees.map((item) => item.x),
      ...overview.specialPoints.map((item) => item.x),
      ...overview.players.map((item) => item.x),
    ];
    const min = Math.min(...xs);
    const max = Math.max(...xs);
    return { min, max, span: max - min || 1 };
  });

  /** Horizontal position in percent, keeping a small margin at both ends. */
  function position(x: number) {
    return 2 + ((x - bounds.min) / bounds.span) * 96;
  }

  const ground = $derived<Marker[]>([
    ...overview.walls.map((wall) => ({
      key: `wall-${wall.id}`,
      x: wall.x,
      label: `${t.inspector.level} ${wall.level}${wall.hasHorn ? " + Horn" : ""}`,
      icon: wallIcon(wall),
      tone: "foreground" as const,
    })),
    ...overview.towers.map((tower) => ({
      key: `tower-${tower.id}`,
      x: tower.x,
      label: tower.typeLabel,
      icon: towerIcon(tower.type),
      tone: "foreground" as const,
    })),
    ...overview.catapults.map((catapult) => ({
      key: `catapult-${catapult.id}`,
      x: catapult.x,
      label: catapult.side === -1 ? t.combat.catapultLeft : t.combat.catapultRight,
      icon: CatapultIcon,
      tone: "foreground" as const,
    })),
    {
      key: "castle",
      x: overview.castleX,
      label: `${overview.castleName} · ${t.inspector.level} ${overview.castleLevel}`,
      icon: CastleIcon,
      tone: "primary" as const,
    },
  ]);

  const landmarks = $derived<Marker[]>([
    ...overview.specialPoints.map((point, index) => ({
      key: `point-${index}`,
      x: point.x,
      label: point.name,
      icon: specialPointIcon(point.type),
      tone: "muted" as const,
    })),
    ...overview.players.map((player) => ({
      key: `player-${player.name}`,
      x: player.x,
      label: player.name,
      icon: CrownIcon,
      tone: "primary" as const,
    })),
  ]);

  const toneClass = {
    primary: "text-primary",
    foreground: "text-foreground",
    muted: "text-muted-foreground",
  };

  const legend = $derived<{ icon: Component; label: string }[]>([
    { icon: CastleIcon, label: t.inspector.castleInfo },
    { icon: WallIcon, label: t.inspector.walls },
    { icon: ArcherTowerIcon, label: t.inspector.towers },
    { icon: MapPinIcon, label: t.inspector.specialPoints },
    { icon: CrownIcon, label: t.resources.player },
  ]);
</script>

{#snippet marker(item: Marker, row: string)}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        <button
          {...props}
          type="button"
          aria-label={`${item.label} · X = ${item.x}`}
          class="hover:bg-accent focus-visible:ring-ring/50 absolute flex size-6 -translate-x-1/2 items-center justify-center rounded-md outline-none focus-visible:ring-3 {row} {toneClass[item.tone]}"
          style:left="{position(item.x)}%"
        >
          <item.icon class="size-4" />
        </button>
      {/snippet}
    </Tooltip.Trigger>
    <Tooltip.Content>{item.label} · X = {item.x}</Tooltip.Content>
  </Tooltip.Root>
{/snippet}

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><IslandIcon class="size-5" />{t.inspector.mapTitle}</Card.Title>
    <Card.Description>{t.inspector.mapDescription}</Card.Description>
  </Card.Header>
  <Card.Content class="flex flex-col gap-3">
    <div class="bg-muted/30 relative h-32 overflow-hidden rounded-lg border">
      <div class="bg-border absolute inset-x-0 bottom-9 h-px"></div>
      {#each overview.trees as tree (tree.id)}
        <span
          class="absolute bottom-9 h-3 w-0.5 -translate-x-1/2 rounded-t-full {tree.marked ? 'bg-warning' : 'bg-success/60'}"
          style:left="{position(tree.x)}%"
        ></span>
      {/each}
      {#each ground as item (item.key)}
        {@render marker(item, "bottom-[2.6rem]")}
      {/each}
      {#each landmarks as item (item.key)}
        {@render marker(item, "top-2")}
      {/each}
      <div class="text-muted-foreground absolute inset-x-2 bottom-2 flex justify-between font-mono text-[10px] tabular-nums">
        <span>X = {Math.round(bounds.min)}</span>
        <span>X = {Math.round(bounds.max)}</span>
      </div>
    </div>
    <div class="text-muted-foreground flex flex-wrap gap-x-4 gap-y-1 text-xs">
      {#each legend as entry (entry.label)}
        <span class="flex items-center gap-1.5"><entry.icon class="size-3.5" />{entry.label}</span>
      {/each}
      <span class="flex items-center gap-1.5"><span class="bg-success/60 h-3 w-0.5 rounded-full"></span><TreeIcon class="size-3.5" />{t.inspector.standing}</span>
      <span class="flex items-center gap-1.5"><span class="bg-warning h-3 w-0.5 rounded-full"></span><TreeIcon class="size-3.5" />{t.inspector.markedToCut}</span>
    </div>
  </Card.Content>
</Card.Root>
