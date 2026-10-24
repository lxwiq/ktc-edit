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
	<path d="M17.96 9.16A8 8 0 1 0 11.39 17.88" />
	<path d="M17.5 12.5c1.8 2.2 3 4 3 6.5a3 3 0 0 1-6 0c0-2.5 1.2-4.3 3-6.5z" />
	<circle cx="7.5" cy="8" r="2" />
	<circle cx="12" cy="13" r="1" />
</svg>
