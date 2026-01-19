<script lang="ts">
    import ThemeSwitcher from "$lib/components/theme/theme-switcher.svelte";
    import * as Item from "$lib/components/ui/item/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
</script>

<div class="p-4 py-2">
    <Item.Root variant="muted">
        <Item.Content>
            <a class="flex items-center" href="/">
                <img class="invert dark:invert-0" src="src/lib/assets/favicon.svg" alt="The Long Dark Modding Logo" height="48" width="48"/>
                <h3 class="ms-4 scroll-m-20 text-2xl font-semibold tracking-tight">The Long Dark Modding</h3>
            </a>
        </Item.Content>
        <Item.Actions>
            <ThemeSwitcher />
        </Item.Actions>
    </Item.Root>
</div>