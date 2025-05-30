<script lang="ts">
    import { Heading } from "flowbite-svelte";
    import { goto } from '$app/navigation';
    import EventForm from "$lib/components/EventForm.svelte";

    function handleSuccess(eventId: string) {
        setTimeout(() => {
            goto(`/event/${eventId}`);
        }, 1500);
    }
</script>

<div class="py-8">
    <div class="container mx-auto px-4 max-w-3xl">
        <Heading tag="h1" class="text-3xl font-bold mb-8 text-center">Create New Event</Heading>

        <EventForm
                submitLabel="Create Event"
                onSuccess={handleSuccess}
        />
    </div>
</div>