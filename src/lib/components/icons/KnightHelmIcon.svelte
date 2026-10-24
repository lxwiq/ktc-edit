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
	<path d="M4 21l1.5-2.5V9.5c0-3 3-5 6.5-5s6.5 2 6.5 5v9L20 21Z" />
	<path d="M8 12h8" />
	<path d="M12 12v6" />
	<path d="M12 4.5c.5-2 2.5-3 5-2.5-1.5.5-2.5 1.5-3 3" />
</svg>
