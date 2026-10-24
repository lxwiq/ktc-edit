<script lang="ts" module>
  // Kept outside the component so typed values survive tab switches.
  const form = $state({
    gotoCoins: 69,
    gotoGems: 3,
    gotoPikemen: 4,
    gotoFarmers: 4,
    gotoBoats: 4,
    takeOverCoins: 69,
    takeOverArchers: 1,
    takeOverWorkers: 1,
    takeOverOffset: 3,
    treesPlayerIndex: 0,
    treesCoins: 69,
    teleportPlayerIndex: 0,
    teleportTargetX: 0,
  });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import PlayerToggle from "$lib/components/common/PlayerToggle.svelte";
  import { AxeIcon, BoatIcon, CastleIcon, CrownIcon, PortalIcon } from "$lib/components/icons";
  import FlagIcon from "@lucide/svelte/icons/flag";
  import SkullIcon from "@lucide/svelte/icons/skull";
  import ZapIcon from "@lucide/svelte/icons/zap";
  import {
    destroyPortals,
    exterminateEnemies,
    gotoIsland,
    markTreesForRemoval,
    takeOverIsland,
    teleportPlayer,
  } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  const teleportPlayerInfo = $derived(
    editor.islandOverview?.players.find(p => p.name === `Player ${form.teleportPlayerIndex + 1}`) ?? null
  );

  function handleGoto() {
    editor.edit(
      (doc) => gotoIsland(doc, {
        ...editor.campaignIsland,
        coins: form.gotoCoins,
        gems: form.gotoGems,
        pikemen: form.gotoPikemen,
        farmers: form.gotoFarmers,
        boats: form.gotoBoats,
      }),
      t.status.destinationUpdated
    );
  }

  function handleTakeOver() {
    editor.edit(
      (doc) => takeOverIsland(doc, {
        ...editor.campaignIsland,
        coins: form.takeOverCoins,
        archerCount: form.takeOverArchers,
        workerCount: form.takeOverWorkers,
        formationOffset: form.takeOverOffset,
      }),
      t.status.islandSecured
    );
  }

  function handleDestroyPortals() {
    editor.edit((doc) => destroyPortals(doc, editor.campaignIsland), t.status.portalsDestroyed);
  }

  function handleExterminate() {
    editor.edit((doc) => exterminateEnemies(doc, editor.campaignIsland), t.status.enemiesRemoved);
  }

  function handleMarkTrees() {
    editor.edit(
      (doc) => markTreesForRemoval(doc, {
        ...editor.campaignIsland,
        playerIndex: form.treesPlayerIndex,
        coins: form.treesCoins,
      }),
      t.status.treesMarked
    );
  }

  function handleTeleportPlayer(targetX?: number) {
    const x = targetX !== undefined ? targetX : form.teleportTargetX;
    const applied = editor.edit(
      (doc) => teleportPlayer(doc, { ...editor.campaignIsland, playerIndex: form.teleportPlayerIndex, x }),
      t.status.playerTeleported
    );
    if (applied) form.teleportTargetX = x;
  }
</script>

<PageIntro title={t.navigation.title} />

<div class="grid gap-6 xl:grid-cols-2">
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><BoatIcon class="size-5" />{t.navigation.fastTravel}</Card.Title>
    </Card.Header>
    <Card.Content>
      <Field.Group class="grid gap-4 sm:grid-cols-2">
        <NumberField label={t.resources.coins} min="0" bind:value={form.gotoCoins} />
        <NumberField label={t.resources.gems} min="0" bind:value={form.gotoGems} />
        <NumberField label={t.navigation.pikemen} min="0" bind:value={form.gotoPikemen} />
        <NumberField label={t.navigation.farmers} min="0" bind:value={form.gotoFarmers} />
        <NumberField label={t.navigation.boats} min="0" bind:value={form.gotoBoats} />
      </Field.Group>
    </Card.Content>
    <Card.Footer>
      <Button onclick={handleGoto}>{t.navigation.updateDestination}</Button>
    </Card.Footer>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><FlagIcon class="size-5" />{t.navigation.takeOver}</Card.Title>
    </Card.Header>
    <Card.Content>
      <Field.Group class="grid gap-4 sm:grid-cols-2">
        <NumberField label={t.navigation.playerCoins} min="0" bind:value={form.takeOverCoins} />
        <NumberField label={t.navigation.archers} min="0" bind:value={form.takeOverArchers} />
        <NumberField label={t.navigation.workers} min="0" bind:value={form.takeOverWorkers} />
        <NumberField label={t.navigation.formationOffset} bind:value={form.takeOverOffset} />
      </Field.Group>
    </Card.Content>
    <Card.Footer>
      <Button onclick={handleTakeOver}>{t.navigation.cleanIsland}</Button>
    </Card.Footer>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><PortalIcon class="size-5" />{t.navigation.threats}</Card.Title>
    </Card.Header>
    <Card.Footer class="flex-wrap gap-2">
      <Button variant="destructive" onclick={handleDestroyPortals}>
        <PortalIcon data-icon="inline-start" />
        {t.navigation.destroy}
      </Button>
      <Button variant="destructive" onclick={handleExterminate}>
        <SkullIcon data-icon="inline-start" />
        {t.navigation.exterminateEnemies}
      </Button>
    </Card.Footer>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><AxeIcon class="size-5" />{t.navigation.markTrees}</Card.Title>
    </Card.Header>
    <Card.Content>
      <Field.Group>
        <PlayerToggle bind:value={form.treesPlayerIndex} />
        <NumberField label={t.resources.coins} min="0" bind:value={form.treesCoins} />
      </Field.Group>
    </Card.Content>
    <Card.Footer>
      <Button onclick={handleMarkTrees}>{t.navigation.mark}</Button>
    </Card.Footer>
  </Card.Root>

  <Card.Root class="xl:col-span-2">
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><CrownIcon class="size-5" />{t.navigation.teleportTitle}</Card.Title>
      {#if teleportPlayerInfo}
        <Card.Action>
          <Badge variant="secondary" class="font-mono">{t.resources.current} X = {teleportPlayerInfo.x}</Badge>
        </Card.Action>
      {/if}
    </Card.Header>
    <Card.Content>
      <Field.Group class="grid gap-4 sm:grid-cols-2">
        <PlayerToggle bind:value={form.teleportPlayerIndex} />
        <NumberField label={t.navigation.teleportX} step="0.5" bind:value={form.teleportTargetX} />
      </Field.Group>
    </Card.Content>
    <Card.Footer class="flex-wrap gap-2">
      <Button onclick={() => handleTeleportPlayer()}>
        <ZapIcon data-icon="inline-start" />
        {t.navigation.teleportButton}
      </Button>
      {#if editor.islandOverview}
        {@const castleX = editor.islandOverview.castleX}
        <Button variant="outline" onclick={() => handleTeleportPlayer(castleX)}>
          <CastleIcon data-icon="inline-start" />
          {t.navigation.teleportToCastle} ({castleX})
        </Button>
      {/if}
    </Card.Footer>
  </Card.Root>
</div>
