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
	<path
		d="M12 2c1.4 0 2.5 1.1 2.5 2.5 0 .8-.5 1.5-1 2 .6 1 1 2.5 1 4 0 2.5-1 4.5-2.5 6-1.5-1.5-2.5-3.5-2.5-6 0-1.5.4-3 1-4-.5-.5-1-1.2-1-2C9.5 3.1 10.6 2 12 2z"
	/>
	<path d="M12 16.5c0 2.5 1.5 4 4 4.5 1.5.3 3-.5 3-2" />
	<path d="M9.5 8.5H7L6 6" />
	<path d="M14.5 8.5H17l1-2.5" />
	<path d="M10 13 7 14l-1 2.5" />
	<path d="m14 13 3 1 1 2.5" />
</svg>
