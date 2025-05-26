<script lang="ts">
    import {Card} from "flowbite-svelte";
    import {ArrowRightOutline} from "flowbite-svelte-icons";
    import KeywordsList from "./KeywordsList.svelte";

    export let id: number;
    export let title: string;
    export let keywords: string[] = [];

    const img = `/events/${id % 10}.png`;
</script>

<Card {img} class="hover:shadow-xl transition-shadow">
    <div class="m-6 flex flex-col items-center justify-between gap-3 text-center h-full">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white flex-grow flex items-center">{title}</h5>


        <div class="flex flex-col items-center justify-between gap-3">
            {#if keywords.length > 0}
                <KeywordsList {keywords} size="small" showIcon={false}/>
            {/if}

            <a href={`event/${id}`} class="cursor-pointer flex mt-2">
                Read more
                <ArrowRightOutline class="ms-2 h-6 w-6"/>
            </a>
        </div>
    </div>
</Card>