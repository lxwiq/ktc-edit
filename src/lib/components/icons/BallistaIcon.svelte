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
	<path d="M3 10h18" />
	<path d="m19 8 2 2-2 2" />
	<path d="M14 3.5q3.5 6.5 0 13" />
	<path d="M14 3.5 9.5 10l4.5 6.5" />
	<path d="M3.5 21 7 10l3.5 11" />
</svg>
