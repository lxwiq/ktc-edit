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
	<path d="M2 13c1 0 1.5 1 2 2.5l.5 1.5a4.5 4.5 0 0 0 4.3 3h6.4a4.5 4.5 0 0 0 4.3-3l.5-1.5c.5-1.5 1-2.5 2-2.5" />
	<path d="M4.5 17h15" />
	<path d="M12 2v2M12 13v4" />
	<path d="M5 4h14" />
	<path d="M6 4v7c4 2 8 2 12 0V4" />
</svg>
