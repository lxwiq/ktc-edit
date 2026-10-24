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
	<path d="M4 21v-9c0-4.5 3-8 8-10 5 2 8 5.5 8 10v9" />
	<path d="M2 21h20" />
	<path d="M13.5 13.5a1.5 1.5 0 0 0-3 0 3 3 0 0 0 6 0 4.5 4.5 0 0 0-9 0" />
</svg>
