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
	<path d="M9 21c-.5-4 0-8 1.5-11.5C12 6 14 4.5 17 4.5c3 0 4.5 2.5 4.5 5.5v1.5L20 10h-2.5c-.5 4 0 7.5 1.5 11z" />
	<path d="M10 10C7.5 8.5 5 6 3.5 2.5c-.5 3 0 5 1.5 6.5-1 .5-1.5 1-1.5 1.5 1 2 3 3.5 5.5 4" />
	<path d="m13 5.5-1-3 3 1.5" />
	<path d="M16.5 7.5h.01" />
</svg>
