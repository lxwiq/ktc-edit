<script lang="ts" module>
  // Kept outside the component so typed values survive tab switches.
  const form = $state({
    archerPlayerIndex: 0,
    archerCount: 1,
    workerCount: 1,
    farmerCount: 1,
    pikemanCount: 1,
    knightCount: 1,
    knightSide: 1 as 1 | -1,
    knightWithArchers: true,
    comboArchers: 0,
    comboWorkers: 0,
    comboPikemen: 0,
  });

  type UnitKind = "archer" | "worker" | "farmer" | "pikeman";
</script>

<script lang="ts">
  import type { Component } from "svelte";
  import { toast } from "svelte-sonner";
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Switch } from "$lib/components/ui/switch";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import PlayerToggle from "$lib/components/common/PlayerToggle.svelte";
  import { BowIcon, CrownIcon, HammerIcon, KnightHelmIcon, PikeIcon, ScytheIcon } from "$lib/components/icons";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import UsersIcon from "@lucide/svelte/icons/users";
  import {
    spawnArchers,
    spawnFarmers,
    spawnKnights,
    spawnPikemen,
    spawnUnits,
    spawnWorkers,
  } from "$lib/saveEditActions";
  import {
    ARCHER_PREFABS,
    FARMER_PREFABS,
    PIKEMAN_PREFABS,
    WORKER_PREFABS,
    countKnightsBySide,
    countUnits,
    saveEditor,
  } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  const rallyPlayer = $derived(
    editor.islandOverview?.players.find(p => p.name === `Player ${form.archerPlayerIndex + 1}`) ?? null
  );

  function count(prefabs: string[]) {
    return countUnits(editor.data, editor.selectedCampaign, editor.selectedIsland, prefabs, editor.targetMode, editor.selectedChallenge) ?? 0;
  }

  const knightStats = $derived(
    countKnightsBySide(editor.data, editor.selectedCampaign, editor.selectedIsland, editor.targetMode, editor.selectedChallenge)
  );

  const units = $derived<{
    kind: UnitKind;
    title: string;
    icon: Component;
    current: number;
    countKey: "archerCount" | "workerCount" | "farmerCount" | "pikemanCount";
    spawn: typeof spawnArchers;
    success: string;
  }[]>([
    { kind: "archer", title: t.recruitment.archers, icon: BowIcon, current: count(ARCHER_PREFABS), countKey: "archerCount", spawn: spawnArchers, success: t.status.archersAdded },
    { kind: "worker", title: t.recruitment.workers, icon: HammerIcon, current: count(WORKER_PREFABS), countKey: "workerCount", spawn: spawnWorkers, success: t.status.workersAdded },
    { kind: "farmer", title: t.recruitment.farmers, icon: ScytheIcon, current: count(FARMER_PREFABS), countKey: "farmerCount", spawn: spawnFarmers, success: t.status.farmersAdded },
    { kind: "pikeman", title: t.recruitment.pikemen, icon: PikeIcon, current: count(PIKEMAN_PREFABS), countKey: "pikemanCount", spawn: spawnPikemen, success: t.status.pikemenAdded },
  ]);

  function handleSpawn(unit: (typeof units)[number]) {
    editor.edit(
      (doc) => unit.spawn(doc, { ...editor.selection, playerIndex: form.archerPlayerIndex, count: form[unit.countKey] }),
      unit.success
    );
  }

  function handleSpawnKnights() {
    editor.edit(
      (doc) => spawnKnights(doc, {
        ...editor.selection,
        playerIndex: form.archerPlayerIndex,
        count: form.knightCount,
        side: form.knightSide,
        withArchers: form.knightWithArchers,
      }),
      t.status.knightsAdded
    );
  }

  function handleSpawnCombo() {
    const archers = Math.max(0, form.comboArchers);
    const workers = Math.max(0, form.comboWorkers);
    const pikemen = Math.max(0, form.comboPikemen);

    if (archers === 0 && workers === 0 && pikemen === 0) {
      toast.error(t.status.addAtLeastOneUnit);
      return;
    }

    editor.edit(
      (doc) => spawnUnits(doc, { ...editor.selection, playerIndex: form.archerPlayerIndex, archers, workers, pikemen }),
      t.status.unitsAdded
    );
  }
</script>

<PageIntro title={t.recruitment.title} />

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><CrownIcon class="size-5" />{t.recruitment.rallyPoint}</Card.Title>
    {#if rallyPlayer}
      <Card.Action>
        <Badge variant="secondary" class="font-mono">X = {rallyPlayer.x} · Y = {rallyPlayer.y}</Badge>
      </Card.Action>
    {/if}
  </Card.Header>
  <Card.Content>
    <PlayerToggle bind:value={form.archerPlayerIndex} />
  </Card.Content>
</Card.Root>

<div class="grid gap-6 md:grid-cols-2">
  {#each units as unit (unit.kind)}
    <Card.Root>
      <Card.Header>
        <Card.Title class="flex items-center gap-2"><unit.icon class="size-5" />{unit.title}</Card.Title>
        <Card.Description>{t.recruitment.current} <span class="text-foreground font-medium tabular-nums">{unit.current}</span></Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="flex items-end gap-2">
          <NumberField label={t.recruitment.quantity} min="1" bind:value={form[unit.countKey]} />
          <Button onclick={() => handleSpawn(unit)}>
            <PlusIcon data-icon="inline-start" />
            {t.recruitment.add}
          </Button>
        </div>
      </Card.Content>
    </Card.Root>
  {/each}
</div>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><KnightHelmIcon class="size-5" />{t.recruitment.knights}</Card.Title>
    <Card.Description>
      {t.recruitment.current} <span class="text-foreground font-medium tabular-nums">{knightStats.total}</span>
    </Card.Description>
    <Card.Action class="flex gap-2">
      <Badge variant="outline"><ArrowLeftIcon data-icon="inline-start" />{knightStats.left}</Badge>
      <Badge variant="outline">{knightStats.right}<ArrowRightIcon data-icon="inline-end" /></Badge>
    </Card.Action>
  </Card.Header>
  <Card.Content>
    <Field.Group class="grid gap-4 md:grid-cols-3 md:items-end">
      <NumberField label={t.recruitment.quantity} min="1" bind:value={form.knightCount} />
      <Field.Field>
        <Field.Label id="knight-side">{t.recruitment.side}</Field.Label>
        <ToggleGroup.Root
          type="single"
          variant="outline"
          aria-labelledby="knight-side"
          bind:value={() => String(form.knightSide), (next) => { if (next) form.knightSide = Number(next) as 1 | -1; }}
        >
          <ToggleGroup.Item value="-1" class="flex-1">
            <ArrowLeftIcon data-icon="inline-start" />
            {t.recruitment.sideLeft}
          </ToggleGroup.Item>
          <ToggleGroup.Item value="1" class="flex-1">
            {t.recruitment.sideRight}
            <ArrowRightIcon data-icon="inline-end" />
          </ToggleGroup.Item>
        </ToggleGroup.Root>
      </Field.Field>
      <Field.Field orientation="horizontal" class="md:h-9">
        <Switch id="knight-with-archers" bind:checked={form.knightWithArchers} />
        <Field.Label for="knight-with-archers">{t.recruitment.withArchers}</Field.Label>
      </Field.Field>
    </Field.Group>
  </Card.Content>
  <Card.Footer>
    <Button onclick={handleSpawnKnights}>
      <PlusIcon data-icon="inline-start" />
      {t.recruitment.add}
    </Button>
  </Card.Footer>
</Card.Root>

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><UsersIcon class="size-5" />{t.recruitment.combinedRecruitment}</Card.Title>
  </Card.Header>
  <Card.Content>
    <Field.Group class="grid gap-4 md:grid-cols-3">
      <NumberField label={t.recruitment.archers} min="0" bind:value={form.comboArchers} />
      <NumberField label={t.recruitment.workers} min="0" bind:value={form.comboWorkers} />
      <NumberField label={t.recruitment.pikemen} min="0" bind:value={form.comboPikemen} />
    </Field.Group>
  </Card.Content>
  <Card.Footer>
    <Button onclick={handleSpawnCombo}>
      <PlusIcon data-icon="inline-start" />
      {t.recruitment.addUnits}
    </Button>
  </Card.Footer>
</Card.Root>
