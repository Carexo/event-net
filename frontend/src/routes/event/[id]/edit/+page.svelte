<script lang="ts">
    import { Heading } from "flowbite-svelte";
    import type {PageProps} from './$types';
    import { goto } from '$app/navigation';
    import EventForm from "$lib/components/EventForm.svelte";
    import ToastNotification from "$lib/components/ToastNotification.svelte";

    let {data}: PageProps = $props();

    let toast: ToastNotification;

    function handleSuccess(id: string) {
        setTimeout(() => goto(`/event/${id}`), 1500);
    }
</script>

<div class="py-8">
    <div class="container mx-auto px-4 max-w-3xl">
        <Heading tag="h1" class="text-3xl font-bold mb-8 text-center">Edit Event</Heading>

        {#if data.error}
            <div class="text-red-500 text-center py-8">{data.error}</div>
        {:else if data.event}
            <EventForm
                    eventId={data.event.id}
                    initialData={{
          name: data.event.name,
          date: data.event.date,
          time: data.event.time,
          keywords: data.event.keywords.join(", ")
        }}
                    submitLabel="Update Event"
                    onSuccess={handleSuccess}
            />
        {/if}
    </div>
</div>

<ToastNotification bind:this={toast}/>