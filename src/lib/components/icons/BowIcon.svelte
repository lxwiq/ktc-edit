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
	<path d="M5 5a11 11 0 0 1 14 14" />
	<path d="M5 5l4 10 10 4" />
	<path d="M3.5 20.5 20 4" />
	<path d="M16 4h4v4" />
	<path d="M3 18h3v3" />
</svg>
