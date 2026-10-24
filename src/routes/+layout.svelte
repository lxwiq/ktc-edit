<script lang="ts">
	import '../app.css';
	import '../kingdom-theme.css';
	import UpdateBanner from '$lib/components/UpdateBanner.svelte';
	
	let { children } = $props();
</script>

<UpdateBanner />
{@render children()}
