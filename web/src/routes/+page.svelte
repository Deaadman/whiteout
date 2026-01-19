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

    export let data;
</script>

<div>
    <Tabs.Root value="mods">
        <div class="grid grid-flow-col">
            <Tabs.List>
                <Tabs.Trigger value="mods">Mods</Tabs.Trigger>
                <Tabs.Trigger value="plugins">Plugins</Tabs.Trigger>
            </Tabs.List>
            <div class="flex gap-2 justify-self-end">
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
                <InputGroup>
                    <InputGroupInput placeholder="Search..." />
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputGroupAddon align="inline-end">{data.mods.length} results</InputGroupAddon>
                </InputGroup>
            </div>
        </div>
        <Tabs.Content value="mods" class="grid grid-cols-3 gap-4">
            {#each data.mods as mod}
                <ModCard mod="{mod}" />
            {/each}
        </Tabs.Content>
        <Tabs.Content value="plugins">
            Change your password here.</Tabs.Content>
    </Tabs.Root>
</div>