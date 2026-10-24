<script lang="ts" module>
  // Kept outside the component so typed values survive tab switches.
  const form = $state({ formationPlayerIndex: 0, formationPosition: 0, formationArchers: 5, formationPikemen: 5 });
</script>

<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import * as Field from "$lib/components/ui/field";
  import { Button } from "$lib/components/ui/button";
  import NumberField from "$lib/components/common/NumberField.svelte";
  import PageIntro from "$lib/components/common/PageIntro.svelte";
  import PlayerToggle from "$lib/components/common/PlayerToggle.svelte";
  import SwordsIcon from "@lucide/svelte/icons/swords";
  import { startFormationAssault } from "$lib/saveEditActions";
  import { saveEditor } from "$lib/state/save-editor.svelte";
  import CatapultCard from "./CatapultCard.svelte";

  const editor = saveEditor;
  const t = $derived(editor.t);

  function handleFormation() {
    editor.edit(
      (doc) => startFormationAssault(doc, {
        ...editor.campaignIsland,
        playerIndex: form.formationPlayerIndex,
        position: form.formationPosition,
        archers: form.formationArchers,
        pikemen: form.formationPikemen,
      }),
      t.status.formationDeployed
    );
  }
</script>

<PageIntro title={t.combat.title} />

<Card.Root>
  <Card.Header>
    <Card.Title class="flex items-center gap-2"><SwordsIcon class="size-5" />{t.combat.battleFormation}</Card.Title>
  </Card.Header>
  <Card.Content>
    <Field.Group class="grid gap-4 md:grid-cols-2">
      <PlayerToggle bind:value={form.formationPlayerIndex} />
      <NumberField label={t.combat.positionX} bind:value={form.formationPosition} />
      <NumberField label={t.navigation.archers} min="0" bind:value={form.formationArchers} />
      <NumberField label={t.navigation.pikemen} min="0" bind:value={form.formationPikemen} />
    </Field.Group>
  </Card.Content>
  <Card.Footer>
    <Button onclick={handleFormation}>{t.combat.deploy}</Button>
  </Card.Footer>
</Card.Root>

<CatapultCard />
