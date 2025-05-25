<script lang="ts">
    import type {PageProps} from './$types';
    import {Heading, Button, Card, Img, Carousel} from "flowbite-svelte";
    import {ArrowRightOutline, CalendarMonthOutline, MapPinOutline, UsersOutline} from "flowbite-svelte-icons";
    import EventCard from "$lib/components/EventCard.svelte";

    let {data}: PageProps = $props();
</script>

<!-- Hero Section -->
<section class="section">
    <div class="flex flex-col md:flex-row items-center gap-10">
        <div class="w-full md:w-1/2">
            <Heading tag="h1" class="text-4xl md:text-5xl font-bold mb-4">Find Your Next Amazing Experience</Heading>
            <p class="text-lg text-gray-700 dark:text-gray-300 mb-8">
                Discover and join events that match your interests and connect with like-minded people.
            </p>
            <div class="flex gap-4">
                <Button href="/events" color="primary" size="xl">Browse Events
                    <ArrowRightOutline class="ml-2 h-5 w-5"/>
                </Button>
                <Button href="/users" outline color="light" size="xl">Connect with Others</Button>
            </div>
        </div>
        <div class="w-full md:w-1/2">
            <Img src="/hero-events.png" alt="People enjoying an event"
                 class="rounded-lg shadow-lg h-auto max-h-96 w-full object-cover"/>
        </div>
    </div>
</section>

<!-- Featured Events -->
<section class="section">
    <Heading tag="h2" class="text-3xl font-bold mb-8 text-center">Featured Events</Heading>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 justify-items-center">
        {#if data.error}
            <div class="text-red-500 p-8 text-center">
                <p>Error: {data.error}</p>
            </div>
        {:else if data.events.length === 0}
            <div class="p-8 text-center">
                <p>No featured events available at the moment.</p>
            </div>
        {/if}
        {#if data.events.length > 0}
            {#each data.events as event}
                <EventCard
                        id={event.id}
                        title={event.name}
                        />
            {/each}
        {/if}

    </div>
    <div class="flex justify-center mt-8">
        <Button href="/events" color="light" size="lg">See All Events
            <ArrowRightOutline class="ml-2 h-5 w-5"/>
        </Button>
    </div>
</section>

<!-- How It Works -->
<section class="bg-gray-50 dark:bg-gray-800 rounded-lg my-10 section">
    <Heading tag="h2" class="text-3xl font-bold mb-8 text-center">How It Works</Heading>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 px-4">
        <div class="flex flex-col items-center text-center">
            <div class="bg-blue-100 dark:bg-blue-900 p-4 rounded-full mb-4">
                <CalendarMonthOutline class="h-10 w-10 text-blue-600 dark:text-blue-300"/>
            </div>
            <h3 class="text-xl font-bold mb-2">Discover Events</h3>
            <p class="text-gray-600 dark:text-gray-400">Browse through our curated selection of events in various
                categories.</p>
        </div>
        <div class="flex flex-col items-center text-center">
            <div class="bg-blue-100 dark:bg-blue-900 p-4 rounded-full mb-4">
                <MapPinOutline class="h-10 w-10 text-blue-600 dark:text-blue-300"/>
            </div>
            <h3 class="text-xl font-bold mb-2">Find Nearby</h3>
            <p class="text-gray-600 dark:text-gray-400">Locate events happening near you with our location-based
                search.</p>
        </div>
        <div class="flex flex-col items-center text-center">
            <div class="bg-blue-100 dark:bg-blue-900 p-4 rounded-full mb-4">
                <UsersOutline class="h-10 w-10 text-blue-600 dark:text-blue-300"/>
            </div>
            <h3 class="text-xl font-bold mb-2">Connect & Join</h3>
            <p class="text-gray-600 dark:text-gray-400">RSVP to events and connect with other attendees who share your
                interests.</p>
        </div>
    </div>
</section>

<!-- Call to Action -->
<section class="py-10 text-center">
    <Card class="max-w-4xl mx-auto py-8 px-12">
        <Heading tag="h2" class="text-3xl font-bold mb-4">Ready to Discover Your Next Event?</Heading>
        <p class="mb-6 text-lg">Join event-net today and never miss out on exciting experiences around you.</p>
        <Button href="/events" gradient color="blue" size="xl">Get Started</Button>
    </Card>
</section>