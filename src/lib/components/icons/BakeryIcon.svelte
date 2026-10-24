<script lang="ts">
	import type { SVGAttributes } from "svelte/elements";

	let { class: className, ...restProps }: SVGAttributes<SVGSVGElement> = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width="24"
	height="24"
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
	class={className}
	aria-hidden="true"
	{...restProps}
>
	<path d="M4 18h16a1 1 0 0 0 1-1v-2a8 8 0 0 0-8-8h-2a8 8 0 0 0-8 8v2a1 1 0 0 0 1 1z" />
	<path d="m9 10-1.5 3m5-3.5L11 13m5.5-2.5L15 13.5" />
</svg>
