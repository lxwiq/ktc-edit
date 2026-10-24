<script lang="ts" module>
  // Kept outside the component so the selected players survive tab switches.
  const form = $state({ coinsPlayerIndex: 0, gemsPlayerIndex: 0 });
</script>

<script lang="ts">
  import type { Component } from "svelte";
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import { Button } from "$lib/components/ui/button";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import PlayerToggle from "$lib/components/common/PlayerToggle.svelte";
  import { CoinIcon, GemIcon } from "$lib/components/icons";
  import { getPlayerCoins, getPlayerGems, setPlayerCoins, setPlayerGems } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  const currentCoins = $derived(getPlayerCoins(editor.data, { ...editor.selection, playerIndex: form.coinsPlayerIndex }) ?? 0);
  const currentGems = $derived(getPlayerGems(editor.data, { ...editor.selection, playerIndex: form.gemsPlayerIndex }) ?? 0);

  // Follow the save whenever it or the selection changes; typing overrides until then.
  let coinsAmount = $derived(currentCoins);
  let gemsAmount = $derived(currentGems);

  function handleCoinsUpdate() {
    editor.edit(
      (doc) => setPlayerCoins(doc, { ...editor.selection, playerIndex: form.coinsPlayerIndex, coins: coinsAmount }),
      t.status.coinsUpdated
    );
  }

  function handleGemsUpdate() {
    editor.edit(
      (doc) => setPlayerGems(doc, { ...editor.selection, playerIndex: form.gemsPlayerIndex, gems: gemsAmount }),
      t.status.gemsUpdated
    );
  }
</script>

{#snippet resourceCard(
  title: string,
  Icon: Component<{ class?: string }>,
  current: number,
  amountLabel: string,
  onApply: () => void,
  kind: "coins" | "gems"
)}
  <Card.Root>
    <Card.Header>
      <Card.Title class="flex items-center gap-2"><Icon class="size-5" />{title}</Card.Title>
      <Card.Description>{t.resources.current} <span class="text-foreground font-medium tabular-nums">{current}</span></Card.Description>
    </Card.Header>
    <Card.Content>
      <Field.Group>
        {#if kind === "coins"}
          <PlayerToggle bind:value={form.coinsPlayerIndex} />
          <NumberField label={amountLabel} min="0" bind:value={coinsAmount} />
        {:else}
          <PlayerToggle bind:value={form.gemsPlayerIndex} />
          <NumberField label={amountLabel} min="0" bind:value={gemsAmount} />
        {/if}
      </Field.Group>
    </Card.Content>
    <Card.Footer>
      <Button onclick={onApply}>{t.resources.apply}</Button>
    </Card.Footer>
  </Card.Root>
{/snippet}

<PageIntro title={t.resources.title} />

<div class="grid gap-6 lg:grid-cols-2">
  {@render resourceCard(t.resources.playerCoins, CoinIcon, currentCoins, t.resources.coins, handleCoinsUpdate, "coins")}
  {@render resourceCard(t.resources.playerGems, GemIcon, currentGems, t.resources.gems, handleGemsUpdate, "gems")}
</div>
