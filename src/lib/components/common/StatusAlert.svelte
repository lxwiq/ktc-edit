<script lang="ts">
  import * as Alert from "$lib/components/ui/alert";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";

  let { status = null, error = null }: { status?: string | null; error?: string | null } = $props();
</script>

{#if status}
  <Alert.Root>
    <CircleCheckIcon />
    <Alert.Description class="break-words">{status}</Alert.Description>
  </Alert.Root>
{/if}
{#if error}
  <Alert.Root variant="destructive">
    <CircleAlertIcon />
    <Alert.Description class="break-words">{error}</Alert.Description>
  </Alert.Root>
{/if}
