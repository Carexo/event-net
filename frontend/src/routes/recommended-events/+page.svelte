<script lang="ts">
    import {onMount, onDestroy} from 'svelte';
    import {Toast} from "flowbite-svelte";
    import {BellOutline} from "flowbite-svelte-icons";
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
            fetchRecommendedEvents();
        } else {
            events = [];
            error = "Please select a user to view recommended events";
            loading = false;
        }
    });

    async function fetchRecommendedEvents() {
        loading = true;
        error = null;

        try {
            const response = await fetch(getApiUrl(`/user/${$selectedUser}/recommendations`));

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.message || 'Failed to fetch recommended events');
            }

            const data = await response.json();
            events = data.data || [];

            if (events.length === 0) {
                error = "No recommendations available for you at this time";
            }
        } catch (err) {
            error = err instanceof Error ? err.message : 'An error occurred while fetching recommendations';
            toast.showToast(error, "red");
        } finally {
            loading = false;
        }
    }
</script>

<EventsGrid
        title="Recommended Events"
        {events}
        {loading}
        {error}
/>

<ToastNotification bind:this={toast}/>