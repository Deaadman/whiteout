<script lang="ts">
	import './layout.css';
    import { ModeWatcher } from "mode-watcher";
	import favicon from '$lib/assets/favicon.svg';
    import Navbar from "$lib/components/navigation/navbar.svelte";

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<ModeWatcher />
<Navbar />
{@render children()}
