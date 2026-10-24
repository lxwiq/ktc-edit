<script lang="ts">
  import * as Field from "$lib/components/ui/field";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import CrownIcon from "$lib/components/icons/CrownIcon.svelte";
  import { saveEditor } from "$lib/state/save-editor.svelte";

  let { value = $bindable(0) }: { value?: number } = $props();

  const t = $derived(saveEditor.t);
  const uid = $props.id();
</script>

<Field.Field>
  <Field.Label id={`player-${uid}`}>{t.resources.player}</Field.Label>
  <ToggleGroup.Root
    type="single"
    variant="outline"
    aria-labelledby={`player-${uid}`}
    bind:value={() => String(value), (next) => { if (next) value = Number(next); }}
  >
    {#each [0, 1] as index (index)}
      <ToggleGroup.Item value={String(index)}>
        <CrownIcon data-icon="inline-start" />
        {t.resources.player} {index + 1}
      </ToggleGroup.Item>
    {/each}
  </ToggleGroup.Root>
</Field.Field>
