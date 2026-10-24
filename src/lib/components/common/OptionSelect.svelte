<script lang="ts" module>
  export interface Option {
    value: string;
    label: string;
  }
</script>

<script lang="ts">
  import * as Select from "$lib/components/ui/select";

  let {
    value = $bindable(""),
    options,
    placeholder = "",
    id,
    disabled = false,
    size = "default",
    class: className,
    "aria-label": ariaLabel,
  }: {
    value?: string;
    options: Option[];
    placeholder?: string;
    id?: string;
    disabled?: boolean;
    size?: "sm" | "default";
    class?: string;
    "aria-label"?: string;
  } = $props();

  const selected = $derived(options.find((option) => option.value === value));
</script>

<Select.Root type="single" bind:value {disabled}>
  <Select.Trigger {id} {size} class={className} aria-label={ariaLabel}>
    {selected?.label ?? placeholder}
  </Select.Trigger>
  <Select.Content>
    <Select.Group>
      {#each options as option (option.value)}
        <Select.Item value={option.value} label={option.label} />
      {/each}
    </Select.Group>
  </Select.Content>
</Select.Root>
