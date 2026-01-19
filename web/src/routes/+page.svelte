<script lang="ts">
    import type { PageProps } from './$types';
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import SearchIcon from "@lucide/svelte/icons/search"
    import ModCard from "$lib/components/mods/mod-card.svelte";

    import {
        InputGroup,
        InputGroupAddon,
        InputGroupInput,
    } from "$lib/components/ui/input-group"

    import {
        _mods,
        _modsOnly,
        _pluginsOnly
    } from "./+page";
    import type { FormEventHandler } from "svelte/elements";
    import type {SvelteComponent} from "svelte";

    let modCount: Number = _mods.length;
    let refs: SvelteComponent[] = [];

    const tabsOnChange = (value: string) => {
        switch (value) {
            case "all":
                modCount = _mods.length;
                break;
            case "mods":
                modCount = _modsOnly.length;
                break;
            case "plugins":
                modCount = _pluginsOnly.length;
                break;
            default:
                modCount = _mods.length;
        }
    }

    function searchOnChange(event: FormEventHandler<HTMLInputElement>) {
        const searchValue = event.srcElement.value.toUpperCase();
        console.log(searchValue);

        let i;
        for (i = 0; i < _mods.length; i++) {
            if (_mods[i].DisplayName.toUpperCase().indexOf(searchValue) > -1) {
                // console.log(refs[i]);
                // refs[i].style.display = "";
            } else {
                // refs[i].style.display = "none";
            }
        }

        modCount = refs.length;
    }
</script>

<div>
    <Tabs.Root value="all" onValueChange={tabsOnChange}>
        <div class="grid grid-flow-col">
            <Tabs.List>
                <Tabs.Trigger value="all">All</Tabs.Trigger>
                <Tabs.Trigger value="mods">Mods</Tabs.Trigger>
                <Tabs.Trigger value="plugins">Plugins</Tabs.Trigger>
            </Tabs.List>
            <div class="flex gap-2 justify-self-end">
                <span class="max-lg:hidden inline-flex gap-2">
                    <Select.Root type="multiple">
                        <Select.Trigger class="w-45">Categories</Select.Trigger>
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
                        <Select.Trigger class="w-45">Sort by</Select.Trigger>
                        <Select.Content>
                            <Select.Item value="alphabetical">Alphabetical</Select.Item>
                            <Select.Item value="updated">Date Updated</Select.Item>
                            <Select.Item value="stars">Stars</Select.Item>
                            <Select.Item value="downloads">Downloads</Select.Item>
                        </Select.Content>
                    </Select.Root>
                </span>
                <InputGroup>
                    <InputGroupInput oninput={searchOnChange} placeholder="Search..." />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputGroupAddon class="max-sm:hidden" align="inline-end">
                        {modCount} results
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </div>
        <Tabs.Content value="all" class="grid grid-cols-5 max-[1920px]:grid-cols-4 max-2xl:grid-cols-3 max-lg:grid-cols-2 max-sm:grid-cols-1 gap-4">
            {#each _mods as mod, i}
                <ModCard bind:this={refs[i]} mod="{mod}" />
            {/each}
        </Tabs.Content>
        <Tabs.Content value="mods" class="grid grid-cols-3 max-sm:grid-cols-1 gap-4">
            {#each _modsOnly as mod}
                <ModCard mod="{mod}" />
            {/each}
        </Tabs.Content>
        <Tabs.Content value="plugins" class="grid grid-cols-3 max-sm:grid-cols-1 gap-4">
            {#each _pluginsOnly as mod}
                <ModCard mod="{mod}" />
            {/each}
        </Tabs.Content>
    </Tabs.Root>
</div>