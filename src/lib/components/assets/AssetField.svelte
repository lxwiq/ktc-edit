<script lang="ts">
  import NumberField from "$lib/components/common/NumberField.svelte";
  import { assetEditor, type AssetNumberKey } from "$lib/state/asset-editor.svelte";

  let {
    key,
    label,
    min,
    max,
    step,
  }: {
    key: AssetNumberKey;
    label: string;
    min?: string;
    max?: string;
    step?: string;
  } = $props();

  // Null means this game build's layout is not recognized for the field: it is shown disabled.
  const value = $derived(assetEditor.settings?.[key] ?? null);
</script>

<NumberField
  {label}
  {min}
  {max}
  {step}
  disabled={value === null}
  bind:value={() => value, (next) => assetEditor.setNumber(key, next ?? 0)}
/>
