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
	<path d="M3 4c1 8 7 13 16.5 12.5" />
	<path d="M5 4c1.5 3 6 3.5 14.5 2.5" />
	<ellipse cx="19.5" cy="11.5" rx="1.5" ry="5" />
	<path d="M3 4h2" />
	<path d="M14.5 7.2c-1 2-1 5.5 0 8.8" />
</svg>
