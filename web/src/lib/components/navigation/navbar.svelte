<script lang="ts">
    import ThemeSwitcher from "$lib/components/theme/theme-switcher.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import CircleQuestionMark from "@lucide/svelte/icons/circle-question-mark";
    import SiDiscord from "@icons-pack/svelte-simple-icons/icons/SiDiscord";
</script>

<div class="p-4 py-2">
    <Card.Root class="py-4">
        <Card.Content>
            <div class="grid grid-flow-col grid-cols-3 gap-4 items-center">
                <div class="col-span-2 justify-self-start">
                    <a class="flex items-center" href="/">
                        <img class="invert dark:invert-0" src="src/lib/assets/favicon.svg" alt="The Long Dark Modding Logo" height="38" width="38"/>
                        <h3 class="ms-4 scroll-m-20 text-2xl font-semibold tracking-tight max-sm:hidden">The Long Dark Modding</h3>
                    </a>
                </div>
                <div class="flex gap-1 justify-self-end">
                    <Button variant="outline" size="icon" href="https://discord.gg/EhBWKRx" target="_blank">
                        <SiDiscord />
                    </Button>
                    <Button variant="outline" size="icon">
                        <CircleQuestionMark />
                    </Button>
                    <ThemeSwitcher />
                </div>
            </div>
        </Card.Content>
    </Card.Root>
</div>