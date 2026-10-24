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
	<circle cx="12" cy="4.5" r="2.5" />
	<path d="M12 7v14" />
	<path d="M8 10h8" />
	<path d="M5 13.5a7 7 0 0 0 14 0" />
	<path d="M2.5 15 5 11.5 7.5 15Z" />
	<path d="M16.5 15 19 11.5 21.5 15Z" />
</svg>
