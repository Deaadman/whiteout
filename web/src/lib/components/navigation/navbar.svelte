<script lang="ts">
    import ThemeSwitcher from "$lib/components/theme/theme-switcher.svelte";
</script>

<div class="p-4 bg-blue-500">
    <div class="grid grid-flow-col grid-cols-3 gap-4 items-center">
        <div class="justify-self-start">
            <a class="flex items-center" href="/">
                <img src="src/lib/assets/favicon.svg" alt="The Long Dark Modding Logo" height="48" width="48"/>
                <h3 class="ms-4 scroll-m-20 text-2xl font-semibold tracking-tight">The Long Dark Modding</h3>
            </a>
        </div>
        <div class="justify-self-center">
<!--            <p>Hello</p>-->
        </div>
        <div class="justify-self-end">
            <ThemeSwitcher />
        </div>
    </div>
</div>