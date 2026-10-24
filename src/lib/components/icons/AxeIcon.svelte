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
	<path d="M3.5 20.5 14.5 9.5" />
	<path d="M12.5 7.5l2-2L17 8q2 1.5 4.5 2.5A5.5 5.5 0 0 1 16 16q0-3.5-1-6Z" />
</svg>
