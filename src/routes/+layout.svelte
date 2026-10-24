<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { Toaster } from '$lib/components/ui/sonner';
	import { updater } from '$lib/state/updater.svelte';

	let { children } = $props();

	onMount(() => {
		updater.check();
	});
</script>

<Toaster theme="dark" position="bottom-right" closeButton />
{@render children()}
