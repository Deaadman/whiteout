<script lang="ts">
    import type { PageProps } from './$types';
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import SearchIcon from "@lucide/svelte/icons/search"
    import ModCard from "$lib/components/mods/mod-card.svelte";
    import ModCardSkeleton from "$lib/components/mods/mod-card-skeleton.svelte";
    import type {FormEventHandler} from "svelte/elements";
    import type {SvelteComponent} from "svelte";
    import {onMount} from 'svelte';

    import {InputGroup, InputGroupAddon, InputGroupInput,} from "$lib/components/ui/input-group"

    import {_mods} from "./+page";

    let modCount = _mods.length;
    let refs: SvelteComponent[] = [];

    const tabsOnChange = (value: string) => {
        switch (value) {
            case "all":
                modCount = _mods.length;
                break;
            default:
                modCount = _mods.length;
        }
    }

    function searchOnChange(event: FormEventHandler<HTMLInputElement>) {
        updateSearch(event.srcElement.value)
    }

    function updateSearch(value: string) {

        const url = new URL(window.location.href);
        if (value !== "") {
            url.searchParams.set("q", value);
            history.pushState(null, "", url);
        } else {
            url.searchParams.delete("q");
            history.pushState(null, "", url);
        }

        let i;
        for (i = 0; i < _mods.length; i++) {
            if (_mods[i].DisplayName.toUpperCase().indexOf(value.toUpperCase()) > -1) {
                refs[i].show();
            } else {
                refs[i].hide();
            }
        }

        modCount = 0;

        let j;
        for (j = 0; j < _mods.length; j++) {
            if (refs[j].isVisible()) {
                modCount++;
            }
        }
    }

    onMount(() => {
        const searchInput = document.getElementById('searchInput');
        if (searchInput === null) {
            return;
        }

        const url = new URL(window.location.href);
        const query = url.searchParams.get("q") || "";
        searchInput.value = query;
        updateSearch(query);
    });
</script>

<div>
    <Tabs.Root value="all" onValueChange={tabsOnChange}>
        <div class="grid grid-flow-col max-md:items-end">
            <Tabs.List>
                <Tabs.Trigger value="all">All</Tabs.Trigger>
                <Tabs.Trigger value="mods">Mods</Tabs.Trigger>
                <Tabs.Trigger value="plugins">Plugins</Tabs.Trigger>
            </Tabs.List>
            <div class="max-md:grid flex gap-2 justify-self-end">
                <Select.Root type="multiple">
                    <Select.Trigger class="min-w-45">Categories</Select.Trigger>
                    <Select.Content>
                        <Select.Item value="audio">Audio</Select.Item>
                        <Select.Item value="base-building">Base Building</Select.Item>
                        <Select.Item value="cheats">Cheats</Select.Item>
                        <Select.Item value="clothing">Clothing</Select.Item>
                        <Select.Item value="food">Food</Select.Item>
                        <Select.Item value="gameplay">Gameplay</Select.Item>
                        <Select.Item value="gear-items">Gear Items</Select.Item>
                        <Select.Item value="graphics">Graphics</Select.Item>
                        <Select.Item value="misc">Misc</Select.Item>
                        <Select.Item value="overhaul">Overhaul</Select.Item>
                        <Select.Item value="quality-of-life">Quality of Life</Select.Item>
                        <Select.Item value="utility">Utility</Select.Item>
                    </Select.Content>
                </Select.Root>
                <Select.Root type="single" value="alphabetical">
                    <Select.Trigger class="min-w-45">Sort by</Select.Trigger>
                    <Select.Content>
                        <Select.Item value="alphabetical">Alphabetical</Select.Item>
                        <Select.Item value="updated">Date Updated</Select.Item>
                    </Select.Content>
                </Select.Root>
                <InputGroup class="max-lg:w-45">
                    <InputGroupInput id="searchInput" oninput={searchOnChange} placeholder="Search..." />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputGroupAddon class="max-lg:hidden" align="inline-end">
                        {modCount}
                        {#if modCount > 1}
                            results
                        {:else}
                            result
                        {/if}
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </div>
        <Tabs.Content value="all" class="grid grid-cols-5 max-[1920px]:grid-cols-4 max-2xl:grid-cols-3 max-lg:grid-cols-2 max-sm:grid-cols-1 gap-4">
            {#if _mods.length === 0}
                {#each Array.from({length: 20}) as _}
                    <ModCardSkeleton />
                {/each}
            {:else}
                {#each _mods as mod, i}
                    <ModCard bind:this={refs[i]} mod="{mod}" />
                {/each}
            {/if}
        </Tabs.Content>
        <Tabs.Content value="mods" class="grid grid-cols-3 max-sm:grid-cols-1 gap-4">
            <!--{#each _modsOnly as mod}-->
            <!--    <ModCard mod="{mod}" />-->
            <!--{/each}-->
        </Tabs.Content>
        <Tabs.Content value="plugins" class="grid grid-cols-3 max-sm:grid-cols-1 gap-4">
            <!--{#each _pluginsOnly as mod}-->
            <!--    <ModCard mod="{mod}" />-->
            <!--{/each}-->
        </Tabs.Content>
    </Tabs.Root>
</div>