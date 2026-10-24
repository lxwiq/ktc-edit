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
	<path d="M3.5 20.5 19 5" />
	<path d="M19 5C14 1.5 7 2.5 3 8c4-2.5 9-2.5 14-1" />
	<path d="M10 14l2.5 2.5" />
</svg>
