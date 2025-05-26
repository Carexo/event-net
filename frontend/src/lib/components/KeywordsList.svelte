<script lang="ts">
    import {Badge} from "flowbite-svelte";
    import {TagSolid} from "flowbite-svelte-icons";

    export let keywords: string[] = [];
    export let showIcon: boolean = true;
    export let size: "small" | "large" = "large";
</script>

<div class="flex items-center gap-2">
    {#if showIcon}
        <TagSolid class="w-5 h-5 text-blue-600 flex-shrink-0" />
    {/if}
    <div class="flex flex-wrap gap-1">
        {#each keywords as keyword}
            <Badge color="blue" {size}>{keyword.trim()}</Badge>
        {/each}
    </div>
</div>