<script lang="ts">
    import {
        Heading,
        Button,
        Label,
        Input,
        Datepicker
    } from "flowbite-svelte";
    import { CalendarWeekSolid, TagSolid } from "flowbite-svelte-icons";

    // Form data
    let eventData = {
        name: "",
        start_datetime: "",
        keywords: ""
    };

    // Function to handle datetime change
    function updateStartDatetime(event) {
        eventData.start_datetime = event.detail.formattedDate;
    }

    // Form submission handler
    async function handleSubmit() {
        // Format the data for submission
        const formattedData = {
            ...eventData,
            keywords: eventData.keywords.split(',').map(k => k.trim()).filter(k => k !== "")
        };

        console.log("Submitting event:", formattedData);

        try {
            const response = await fetch('/api/event', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(formattedData)
            });

            if (!response.ok) {
                throw new Error('Failed to create event');
            }

            // Reset form or redirect
            eventData = { name: "", start_datetime: "", keywords: "" };
            alert("Event created successfully!");
        } catch (error) {
            console.error("Error creating event:", error);
            alert("Failed to create event");
        }
    }
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900 py-8">
    <div class="container mx-auto px-4 max-w-3xl">
        <Heading tag="h1" class="text-3xl font-bold mb-8 text-center">Create New Event</Heading>

        <form on:submit|preventDefault={handleSubmit} class="space-y-6 bg-white dark:bg-gray-800 p-8 rounded-lg shadow">
            <!-- Event Name -->
            <div class="mb-4">
                <Label for="name" class="mb-2">Event Name*</Label>
                <Input id="name" required bind:value={eventData.name} placeholder="Enter event name" />
            </div>

            <!-- Datetime Selector -->
            <div class="mb-4">
                <Label class="mb-2 flex items-center">
                    <CalendarWeekSolid class="mr-2 h-5 w-5 text-blue-600" />
                    Start Date and Time*
                </Label>
                <Datepicker
                        timePicker={true}
                        on:change={updateStartDatetime}
                        required
                        class="w-full"
                />
            </div>

            <!-- Keywords -->
            <div class="mb-4">
                <Label for="keywords" class="mb-2 flex items-center">
                    <TagSolid class="mr-2 h-5 w-5 text-blue-600" />
                    Keywords
                </Label>
                <Input id="keywords" bind:value={eventData.keywords}
                       placeholder="Comma-separated keywords (e.g., tech, workshop, networking)" />
                <p class="text-sm text-gray-500 mt-1">These help people discover your event</p>
            </div>

            <!-- Submit Button -->
            <div class="flex justify-center mt-6">
                <Button type="submit" size="lg" color="blue" class="px-8">Create Event</Button>
            </div>
        </form>
    </div>
</div>