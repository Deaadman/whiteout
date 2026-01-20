<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import DownloadIcon from "@lucide/svelte/icons/download"
    import StarIcon from "@lucide/svelte/icons/star"
    import HeartIcon from "@lucide/svelte/icons/heart"
    import GitHubIcon from "@lucide/svelte/icons/github"
    import RefreshCCWIcon from "@lucide/svelte/icons/refresh-ccw"
    import BadgeAlertIcon from "@lucide/svelte/icons/badge-alert"
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { Skeleton } from "$lib/components/ui/skeleton/index.js";
    import {Button} from "$lib/components/ui/button";
    import {AspectRatio} from "$lib/components/ui/aspect-ratio/index.js";
</script>

<Card.Root>
    <Card.Header>
        <AspectRatio ratio={16 / 9}>
            <Skeleton class="w-full h-full" />
        </AspectRatio>
        <div class="grid grid-flow-col grid-cols-2">
            <div class="mt-2 space-y-2">
                <Skeleton class="h-4 w-40" />
                <Skeleton class="h-4 w-20" />
            </div>
            <div class="mt-2 grid gap-1 col-start-2 content-center justify-self-end justify-items-end">
                <Skeleton class="h-4 w-20" />
                <Skeleton class="h-4 w-40" />
            </div>
        </div>
    </Card.Header>
    <Card.Content>
        <Skeleton class="h-10" />
    </Card.Content>
    <Card.Footer>
        <div class="grid grid-flow-col grid-rows-3 w-full">
            <div class="col-start-1 flex gap-1 content-center">
                <Skeleton class="h-10 w-10" />
                <Skeleton class="h-10 w-10" />
                <Skeleton class="h-10 w-10" />
            </div>
            <div class="grid gap-1 col-start-2 content-center justify-self-end justify-items-end">
                <Skeleton class="h-4 w-30" />
                <Skeleton class="h-4 w-20" />
            </div>
            <div class="col-span-2 mt-2 space-y-2">
                <Skeleton class="h-4 w-20" />
                <div class="flex gap-1">
                    <Skeleton class="h-6 w-20" />
                    <Skeleton class="h-6 w-20" />
                    <Skeleton class="h-6 w-20" />
                </div>
            </div>
            <div class="col-span-2 mt-4 space-y-2">
                <Skeleton class="h-4 w-20" />
                <div class="flex gap-1">
                    <Skeleton class="h-6 w-20" />
                    <Skeleton class="h-6 w-20" />
                    <Skeleton class="h-6 w-20" />
                </div>
            </div>
        </div>
    </Card.Footer>
</Card.Root>