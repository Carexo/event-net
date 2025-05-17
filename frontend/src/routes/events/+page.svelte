<script lang="ts">
    import {Heading} from "flowbite-svelte";
    import EventCard from "$lib/components/EventCard.svelte";
</script>

<div class="text-center">
    <Heading tag="h1" class="py-5">All events</Heading>
    <section class="p-5 flex gap-8 flex-wrap justify-center">
        <EventCard />
        <EventCard />
        <EventCard />
    </section>
</div>