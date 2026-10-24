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
	<path d="M2 20q2.5-1.5 5 0t5 0 5 0 5 0" />
	<path d="M3 16.5c1.5-2.5 4.5-3.5 9-3.5s7.5 1 9 3.5" />
	<path d="M10 13V8.5h4V13" />
	<path d="M12 8.5V3" />
	<path d="M12 3h3.5l-1 1.25 1 1.25H12" />
</svg>
