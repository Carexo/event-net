<script lang="ts">
    import { Heading } from "flowbite-svelte";
    import EventCard from "$lib/components/EventCard.svelte";
    import type { EventCard as EventCardType } from "$lib/types/event";

    export let title: string;
    export let events: EventCardType[] = [];
    export let loading: boolean = false;
    export let error: string | null = null;
</script>

<div class="container mx-auto px-4 py-8">
    <Heading tag="h1" class="text-3xl font-bold mb-8 text-center">{title}</Heading>

    {#if loading}
        <div class="flex justify-center my-20">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
        </div>
    {:else if error && events.length === 0}
        <div class="text-center my-20">
            <p class="text-lg text-gray-600 dark:text-gray-400">{error}</p>
        </div>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 justify-self-center lg:grid-cols-3 gap-6">
            {#each events as event}
                <EventCard
                        id={event.id}
                        title={event.name}
                        keywords={event.keywords}
                />
            {/each}
        </div>
    {/if}
</div>