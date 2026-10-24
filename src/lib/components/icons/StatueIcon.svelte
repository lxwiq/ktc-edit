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
	<circle cx="11" cy="4.5" r="2" />
	<path d="M8.5 17 9 10.5a2 2 0 0 1 4 0l.5 6.5" />
	<path d="M13 11l3-2" />
	<path d="M16 2.5V12" />
	<path d="M6 17h12v4H6Z" />
</svg>
