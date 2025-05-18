<script lang="ts">
    import {Heading} from "flowbite-svelte";
    import EventCard from "$lib/components/EventCard.svelte";

    const featuredEvents = [
        {
            id: "1",
            title: "Tech Conference 2025",
            description: "Join the biggest tech conference in the region with keynotes from industry leaders."
        },
        {
            id: "2",
            title: "Summer Music Festival",
            description: "Three days of amazing performances from top artists across multiple stages."
        },
        {
            id: "3",
            title: "Food & Wine Expo",
            description: "Taste exquisite cuisine and wines from around the world at this premier culinary event."
        }
    ];
</script>

<div class="text-center">
    <Heading tag="h1" class="py-5">All events</Heading>
    <section class="p-5 flex gap-8 flex-wrap justify-center">
        {#each featuredEvents as event}
            <EventCard
                    id={event.id}
                    img="/event-placeholder.png"
                    title={event.title}
                    description={event.description} />
        {/each}

    </section>
</div>