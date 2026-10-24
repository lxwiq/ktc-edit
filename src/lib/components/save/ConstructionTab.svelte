<script lang="ts" module>
  // Kept outside the component so typed values survive tab switches.
  const form = $state({ pimpCoins: 69, pimpSpawnCount: 10 });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import { Button } from "$lib/components/ui/button";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import { CastleIcon } from "$lib/components/icons";
  import { pimpIsland } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  function handlePimp() {
    editor.edit(
      (doc) => pimpIsland(doc, { ...editor.campaignIsland, coins: form.pimpCoins, spawnCount: form.pimpSpawnCount }),
      t.status.islandUpgraded
    );
  }
</script>

<PageIntro title={t.construction.title} />

<Card.Root class="max-w-2xl">
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><CastleIcon class="size-5" />{t.construction.pimpIsland}</Card.Title>
  </Card.Header>
  <Card.Content>
    <Field.Group class="grid gap-4 sm:grid-cols-2">
      <NumberField label={t.navigation.playerCoins} min="0" bind:value={form.pimpCoins} />
      <NumberField label={t.construction.unitsToRecruit} min="0" bind:value={form.pimpSpawnCount} />
    </Field.Group>
  </Card.Content>
  <Card.Footer>
    <Button onclick={handlePimp}>{t.construction.upgrade}</Button>
  </Card.Footer>
</Card.Root>
