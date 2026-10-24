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
	<path d="M12.5 3.5Q13.5 2.5 14.5 3.5L20.5 9.5Q21.5 10.5 20.5 11.5L17.5 14.5Q16.5 15.5 15.5 14.5L9.5 8.5Q8.5 7.5 9.5 6.5Z" />
	<path d="M3.5 20.5 13 11" />
</svg>
