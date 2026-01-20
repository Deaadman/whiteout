<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import DownloadIcon from "@lucide/svelte/icons/download"
    import StarIcon from "@lucide/svelte/icons/star"
    import HeartIcon from "@lucide/svelte/icons/heart"
    import GitHubIcon from "@lucide/svelte/icons/github"
    import RefreshCCWIcon from "@lucide/svelte/icons/refresh-ccw"
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import {Button} from "$lib/components/ui/button";
    import {AspectRatio} from "$lib/components/ui/aspect-ratio/index.js";

    let { mod } = $props();

    const updatedDate = new Date(mod.Updated);
    const currentDate = new Date();

    const diffTime= currentDate - updatedDate;
    const diffDays = diffTime / (1000 * 3600 * 24);
    const diffYears = diffDays / 365.2425;
    const diffMonths = diffDays / 30.44;
    const diffWeeks = diffDays / 7;

    function dateDifference() : String {
        if (diffYears >= 2) {
            return `Updated ${Math.floor(diffYears)} years ago`
        } else if (diffYears >= 1) {
            return `Updated last year`;
        }

        if (diffMonths >= 2) {
            return `Updated ${Math.floor(diffMonths)} months ago`
        } else if (diffMonths >= 1) {
            return `Updated last month`;
        }

        if (diffWeeks >= 2) {
            return `Updated ${Math.floor(diffWeeks)} weeks ago`;
        } else if (diffWeeks >= 1) {
            return `Updated last week`;
        }

        if (diffDays >= 2) {
            return `Updated ${Math.floor(diffDays)} days ago`;
        } else if (diffDays >= 1) {
            return `Updated yesterday`;
        } else {
            return `Updated today`;
        }
    }

    function dateDifferenceClass() : String {
        if (diffYears >= 1) {
            return `bg-red-500 text-white dark:bg-red-600`
        }

        if (diffMonths >= 1) {
            return `bg-yellow-500 text-white dark:bg-yellow-600`
        }

        if (diffDays < 30) {
            return `bg-green-500 text-white dark:bg-green-600`
        }

        return ``;
    }
</script>

<Card.Root>
    <Card.Header>
        <AspectRatio ratio={16 / 9}>
            {#if mod.Images[0]}
                <img src="{mod.Images[0]}" alt="{mod.DisplayName}" class="w-full h-full" />
            {:else}
                <img src="src/lib/assets/unavailable-dark.png" alt="{mod.DisplayName}" class="w-full h-full hidden dark:block" />
                <img src="src/lib/assets/unavailable-light.png" alt="{mod.DisplayName}" class="w-full h-full block dark:hidden" />
            {/if}
        </AspectRatio>
        <div class="grid grid-flow-col grid-cols-2">
            <div class="mt-2">
                <Card.Title>{mod.DisplayName}</Card.Title>
                <Card.Description>by <a class="underline" href="{mod.AuthorUrl}" target="_blank">{mod.Author}</a></Card.Description>
            </div>
            <div class="mt-2 justify-self-end text-end">
                <Card.Title>v{mod.Version}</Card.Title>
                <Card.Description>TLD v{mod.TestedOn.tld} / ML v{mod.TestedOn.ml}</Card.Description>
            </div>
        </div>
    </Card.Header>
    <Card.Content class="h-10 overflow-auto">
        <Card.Description>
            {mod.Description}
        </Card.Description>
    </Card.Content>
    <Card.Footer>
        <div class="grid grid-flow-col grid-rows-3 w-full">
            <div class="col-start-1 content-center">
                <Button variant="outline" size="icon" href={mod.ModUrl} target="_blank">
                    <GitHubIcon />
                </Button>
                <Button variant="outline" size="icon" href={mod.Download} target="_blank">
                    <DownloadIcon />
                </Button>
                {#if mod.SupportUrl}
                    <Button variant="outline" size="icon" href={mod.SupportUrl} target="_blank">
                        <HeartIcon />
                    </Button>
                {/if}
            </div>
            <div class="col-start-2 content-center justify-self-end">
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Badge variant="secondary" class={dateDifferenceClass()}>
                            <RefreshCCWIcon />
                            {dateDifference()}
                        </Badge>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                        <p>{new Date(mod.Updated).toDateString()}</p>
                    </Tooltip.Content>
                </Tooltip.Root>
            </div>
            {#if mod.Dependencies.length > 0}
                <div class="col-span-2 mt-2">
                    <p>Dependencies</p>
                    {#each mod.Dependencies as dependency}
                        <Badge class="me-1">{dependency}</Badge>
                    {/each}
                </div>
            {/if}
            {#if mod.Categories.length > 0}
                <div class="col-span-2 mt-2">
                    <p>Categories</p>
                    {#each mod.Categories as category}
                        <Badge class="me-1">{category}</Badge>
                    {/each}
                </div>
            {/if}
        </div>
    </Card.Footer>
</Card.Root>