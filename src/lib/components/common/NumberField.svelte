<script lang="ts">
  import * as Field from "$lib/components/ui/field";
  import { Input } from "$lib/components/ui/input";
  import type { HTMLInputAttributes } from "svelte/elements";

  let {
    label,
    value = $bindable(0),
    description,
    emptyValue = 0,
    id,
    disabled,
    class: className,
    ...restProps
  }: Omit<HTMLInputAttributes, "value" | "type" | "children" | "files"> & {
    label: string;
    value?: number | null;
    description?: string;
    /** Value stored when the input is cleared. */
    emptyValue?: number | null;
  } = $props();

  const uid = $props.id();
  const inputId = $derived(id ?? `number-${uid}`);
</script>

<Field.Field data-disabled={disabled || undefined} class={className}>
  <Field.Label for={inputId}>{label}</Field.Label>
  <Input
    id={inputId}
    type="number"
    {disabled}
    bind:value={
      () => value,
      (next) => (value = next === null || next === undefined ? emptyValue : Number(next))
    }
    {...restProps}
  />
  {#if description}
    <Field.Description>{description}</Field.Description>
  {/if}
</Field.Field>
