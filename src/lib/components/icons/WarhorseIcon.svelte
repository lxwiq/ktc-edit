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
	<path
		d="M4 21C3.5 14 6 8 10 4.5L11.5 2 13 4.5l6.5 5c2 1.5 2 4.5 0 5.5-1.5.5-3 .2-4.5-.5-1.2-.6-2.5-1-2.5-1.5 0 3 1 5.5 3 8z"
	/>
	<path d="M18.5 9 16 13.5" />
	<path d="M6 10c2-.5 4-.5 5.5.5" />
	<path d="M4.5 15c3-1 6-1.2 8.5-.5" />
	<path d="M14.5 8h.01" />
</svg>
