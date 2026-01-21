<script lang="ts">
	import './layout.css';
    import { ModeWatcher } from "mode-watcher";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
	import favicon from '$lib/assets/favicon.svg';
    import Navbar from "$lib/components/navigation/navbar.svelte";

	let { children } = $props();
</script>

<svelte:head>
    <title>The Long Dark Modding</title>
    <link rel="icon" href={favicon} />
</svelte:head>
<ModeWatcher />
<Navbar />
<div class="p-4 py-2">
    <Tooltip.Provider>
        {@render children()}
    </Tooltip.Provider>
</div>
