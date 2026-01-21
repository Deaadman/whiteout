<script lang="ts">
    import ThemeSwitcher from "$lib/components/theme/theme-switcher.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import CircleQuestionMark from "@lucide/svelte/icons/circle-question-mark";
    import SiDiscord from "@icons-pack/svelte-simple-icons/icons/SiDiscord";
    import {Badge} from "$lib/components/ui/badge/index.js";
    import {onMount} from "svelte";

    interface SiteData {
        currentVersion: String,
        lastBigPatch: String,
        latestGameBreaking: String,
        melonloaderVersion: String,
        melonNightly: Boolean,
        announcement: String,
        lists: String[],
        oldlists: String[],
        overrides: String[]
    }

    let json: SiteData | null = null;

    async function querySiteData() {
        const response = await fetch("https://raw.githubusercontent.com/TLD-Mods/ModLists/master/SiteData.json");
        json = await response.json()
    }

    onMount(() => {
        querySiteData();
    });
</script>

<div class="p-4 py-2">
    <Card.Root class="py-4">
        <Card.Content>
            <div class="grid grid-flow-col grid-cols-3 gap-4 items-center">
                <div class="justify-self-start">
                    <a class="flex items-center" href="/">
                        <img class="invert dark:invert-0" src="src/lib/assets/favicon.svg" alt="The Long Dark Modding Logo" height="38" width="38"/>
                        <h3 class="ms-4 scroll-m-20 text-2xl font-semibold tracking-tight max-md:hidden">The Long Dark Modding</h3>
                    </a>
                </div>
                <div class="flex gap-2 justify-center max-sm:col-span-3">
                    <Button variant="outline" href="https://discord.gg/EhBWKRx" target="_blank">
                        <SiDiscord /> Discord
                    </Button>
                    <Button variant="outline" href="/help">
                        <CircleQuestionMark /> Help
                    </Button>
                    <ThemeSwitcher/>
                </div>
                <div class="grid gap-1 justify-items-end justify-self-end max-sm:hidden">
                    <Badge>
                        The Long Dark Version: {json?.currentVersion}
                    </Badge>
                    <Badge class="bg-yellow-500 text-white dark:bg-yellow-600">
                        Latest Mod-Breaking: {json?.latestGameBreaking}
                    </Badge>
                    <Badge>
                        MelonLoader Version: {json?.melonloaderVersion}
                    </Badge>
                </div>
            </div>
        </Card.Content>
    </Card.Root>
</div>