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
	<path d="M2 20h20" />
	<path d="M3 20 9 9l6 11" />
	<path d="M9 9 7.5 6.5M9 9l1.5-2.5" />
	<path d="M7 20l2-4 2 4" />
	<path d="M19 18.5a2 2 0 0 1-2-2c0-1.5 2-3.5 2-5.5 0 2 2 4 2 5.5a2 2 0 0 1-2 2Z" />
</svg>
