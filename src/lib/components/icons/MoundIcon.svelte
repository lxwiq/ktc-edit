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
	<path d="M2 20h20" />
	<path d="M3 20c3-1 4-3 5.5-5.5C10 12 11 10.5 12.5 10.5s2.5 1.5 4 4c1.5 2.5 2.5 4.5 4.5 5.5" />
	<path d="M10 17h.01M14 16h.01M12.5 13.5h.01" />
</svg>
