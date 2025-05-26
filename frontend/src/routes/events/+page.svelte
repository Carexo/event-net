<script lang="ts">
    import type {PageProps} from './$types';
    import {Heading} from "flowbite-svelte";
    import EventCard from "$lib/components/EventCard.svelte";

    let {data}: PageProps = $props();
</script>

<div class="text-center">
    <Heading tag="h1" class="py-5">All events</Heading>

    {#if data.error}
        <div class="text-red-500 p-8">
            <p>Error: {data.error}</p>
        </div>
    {:else if data.events.length === 0}
        <div class="p-8">
            <p>No events found.</p>
        </div>
    {:else}
        <section class="p-5 flex gap-8 flex-wrap justify-center">
            {#each data.events as event}
                <EventCard
                        id={event.id}
                        title={event.name}
                        keywords={event.keywords}
                />
            {/each}
        </section>
    {/if}
</div>