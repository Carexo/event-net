<script lang="ts">
    import {onMount} from 'svelte';
    import {selectedUser} from "$lib/stores/userStore";
    import {getApiUrl} from "$lib/utils/api";
    import type {EventCard} from "$lib/types/event";
    import EventsGrid from "$lib/components/EventsGrid.svelte";
    import ToastNotification from "$lib/components/ToastNotification.svelte";

    let loading = $state(true);
    let events: EventCard[] = $state([]);
    let error: string | null = $state(null);

    let toast: ToastNotification;

    $effect(() => {
        if ($selectedUser) {
            fetchUserEvents();
        } else {
            events = [];
            error = "Please select a user to view your events";
            loading = false;
        }
    });

    async function fetchUserEvents() {
        loading = true;
        error = null;

        try {
            const response = await fetch(getApiUrl(`/user/${$selectedUser}/events`));

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.message || 'Failed to fetch your events');
            }

            const data = await response.json();
            events = data.data || [];

            if (events.length === 0) {
                error = "You're not signed up for any events yet";
            }
        } catch (err) {
            error = err instanceof Error ? err.message : 'An error occurred while fetching events';
            toast.showToast(error, "red");
        } finally {
            loading = false;
        }
    }
</script>

<EventsGrid
        title="My Events"
        {events}
        {loading}
        {error}
/>

<ToastNotification bind:this={toast}/>