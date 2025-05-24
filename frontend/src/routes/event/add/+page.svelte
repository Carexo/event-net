<script lang="ts">
    import {
        Heading,
        Card,
        Button,
        Label,
        Input,
        Textarea,
        Checkbox,
        Select
    } from "flowbite-svelte";
    import { CalendarWeekSolid, ClockSolid, TagSolid, MapPinSolid } from "flowbite-svelte-icons";

    // Form data
    let eventData = {
        name: "",
        description: "",
        location: "",
        start_date: "",
        start_time: "",
        end_date: "",
        end_time: "",
        keywords: "",
        max_participants: 50,
        is_private: false
    };

    // Categories for selection
    const categories = [
        "Technology",
        "Music",
        "Food & Drink",
        "Sports",
        "Arts",
        "Business",
        "Education",
        "Health"
    ];

    let selectedCategory = categories[0];

    // Form submission handler
    function handleSubmit() {
        // Format the data for submission
        const formattedData = {
            ...eventData,
            start_datetime: `${eventData.start_date}T${eventData.start_time}`,
            end_datetime: `${eventData.end_date}T${eventData.end_time}`,
            keywords: eventData.keywords.split(',').map(k => k.trim()),
            category: selectedCategory
        };

        console.log("Submitting event:", formattedData);
        // Here you would normally submit to your API
        // alert("Event created successfully!");
    }
</script>

<div class="container mx-auto px-4 py-8 max-w-4xl">
    <Heading tag="h1" class="text-3xl font-bold mb-6 text-center">Create New Event</Heading>

    <Card class="p-6">
        <form on:submit|preventDefault={handleSubmit} class="space-y-6">
            <!-- Basic Details Section -->
            <div>
                <Heading tag="h2" class="text-xl font-semibold mb-4">Event Details</Heading>

                <div class="mb-4">
                    <Label for="name" class="mb-2">Event Name*</Label>
                    <Input id="name" required bind:value={eventData.name} placeholder="Enter event name" />
                </div>

                <div class="mb-4">
                    <Label for="description" class="mb-2">Description*</Label>
                    <Textarea id="description" rows={4} required bind:value={eventData.description}
                              placeholder="Describe your event" />
                </div>

                <div class="mb-4">
                    <Label for="category" class="mb-2">Category*</Label>
<!--                    <Select id="category" items={categories} bind:value={selectedCategory} />-->
                </div>
            </div>

            <!-- Date and Time Section -->
            <div>
                <Heading tag="h2" class="text-xl font-semibold mb-4 flex items-center">
                    <CalendarWeekSolid class="mr-2 h-5 w-5 text-blue-600" />
                    Date and Time
                </Heading>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                        <Label for="start_date" class="mb-2">Start Date*</Label>
                        <Input id="start_date" type="date" required bind:value={eventData.start_date} />
                    </div>
                    <div>
                        <Label for="start_time" class="mb-2">Start Time*</Label>
                        <Input id="start_time" type="time" required bind:value={eventData.start_time} />
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <Label for="end_date" class="mb-2">End Date*</Label>
                        <Input id="end_date" type="date" required bind:value={eventData.end_date} />
                    </div>
                    <div>
                        <Label for="end_time" class="mb-2">End Time*</Label>
                        <Input id="end_time" type="time" required bind:value={eventData.end_time} />
                    </div>
                </div>
            </div>

            <!-- Location Section -->
            <div>
                <Heading tag="h2" class="text-xl font-semibold mb-4 flex items-center">
                    <MapPinSolid class="mr-2 h-5 w-5 text-blue-600" />
                    Location
                </Heading>

                <div class="mb-4">
                    <Label for="location" class="mb-2">Location*</Label>
                    <Input id="location" required bind:value={eventData.location}
                           placeholder="Enter venue or address" />
                </div>
            </div>

            <!-- Additional Details Section -->
            <div>
                <Heading tag="h2" class="text-xl font-semibold mb-4">Additional Details</Heading>

                <div class="mb-4">
                    <Label for="keywords" class="mb-2 flex items-center">
                        <TagSolid class="mr-2 h-5 w-5 text-blue-600" />
                        Keywords
                    </Label>
                    <Input id="keywords" bind:value={eventData.keywords}
                           placeholder="Comma-separated keywords (e.g., tech, workshop, networking)" />
                    <p class="text-sm text-gray-500 mt-1">These help people discover your event</p>
                </div>

                <div class="mb-4">
                    <Label for="max_participants" class="mb-2">Maximum Participants</Label>
                    <Input id="max_participants" type="number" min="1" bind:value={eventData.max_participants} />
                </div>

                <div class="mb-4">
                    <Checkbox bind:checked={eventData.is_private} id="is_private">
                        Private Event (invitation only)
                    </Checkbox>
                </div>
            </div>

            <!-- Submit Button -->
            <div class="flex justify-center mt-8">
                <Button type="submit" size="xl" color="blue" class="px-8">Create Event</Button>
            </div>
        </form>
    </Card>
</div>