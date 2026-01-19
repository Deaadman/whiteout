<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import DownloadIcon from "@lucide/svelte/icons/download"
    import StarIcon from "@lucide/svelte/icons/star"
    import HeartIcon from "@lucide/svelte/icons/heart"
    import RefreshCCWIcon from "@lucide/svelte/icons/refresh-ccw"
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import {Button} from "$lib/components/ui/button";
</script>

<Card.Root>
    <Card.Header>
        <img src="https://raw.githubusercontent.com/Deaadman/AdaptiveArsenal/release/Images/Thumbnail.webp" alt="Test"/>
        <div class="grid grid-flow-col grid-cols-2">
            <div class="mt-2">
                <Card.Title>Adaptive Arsenal</Card.Title>
                <Card.Description>by <a class="underline" href="https://github.com/Deaadman">Deadman</a></Card.Description>
            </div>
            <div class="mt-2 justify-self-end text-end">
                <Card.Title>v1.0.9</Card.Title>
                <Card.Description>TLD v2.51 / ML v0.7.2</Card.Description>
            </div>
        </div>
    </Card.Header>
    <Card.Content>
        <Card.Description>Adaptive Arsenal is a modification that improves upon existing weapon mechanics within The Long Dark.</Card.Description>
    </Card.Content>
    <Card.Footer>
        <div class="grid grid-flow-col grid-rows-3 w-full">
            <div class="col-start-1 content-center">
                <Button variant="outline" size="icon">
                    <DownloadIcon />
                </Button>
                <Button variant="outline" size="icon">
                    <HeartIcon />
                </Button>
            </div>
            <div class="col-start-2 content-center justify-self-end">
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Badge variant="secondary" class="bg-blue-500 text-white dark:bg-blue-600">
                            <RefreshCCWIcon />
                            Updated Last Month
                        </Badge>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                        <p>17 January 2026, at 19:32</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </div>
            <div class="col-span-2 mt-2">
                <p>Dependencies</p>
                <Badge>ComplexLogger</Badge>
                <Badge>ModData</Badge>
                <Badge>ModComponent</Badge>
            </div>
            <div class="col-span-2 mt-2">
                <p>Categories</p>
                <Badge>Gameplay</Badge>
                <Badge>Overhaul</Badge>
            </div>
        </div>
    </Card.Footer>
</Card.Root>