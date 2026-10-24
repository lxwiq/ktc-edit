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
	<path d="M3 17h18" />
	<circle cx="7" cy="19.5" r="1.5" />
	<circle cx="17" cy="19.5" r="1.5" />
	<path d="m10 17 3-6 3 6" />
	<path d="M18 15 8.5 7" />
	<circle cx="6.5" cy="5.5" r="2.5" />
</svg>
