<script lang="ts">
    import type {PageProps} from './$types';
    import {Heading, MultiSelect, Button} from "flowbite-svelte";
    import EventCard from "$lib/components/EventCard.svelte";
    import {goto} from "$app/navigation";
    let {data}: PageProps = $props();

    const keywords = data.keywords?.map((item) => ({ value: item, name: item }));
    const multiSelectPlaceholder = "keywords";
    let selected: string[] = $state(data.keywordsParams || []);

    const getSearchParams = () => {
        console.log(selected);
        const searchParams = new URLSearchParams();
        selected.forEach(kw => searchParams.append("keyword", kw));
        goto(new URL(window.location.href).pathname + "?" + searchParams.toString());
    }
</script>

<div class="text-center">
    <Heading tag="h1" class="py-5">All events</Heading>
    <div class="flex gap-8 place-self-center" >
        <MultiSelect class="w-lg" items={keywords} bind:value={selected} size="lg" placeholder={multiSelectPlaceholder} />
        <Button class="cursor-pointer" onclick={() => getSearchParams()} outline color="light" size="xl">Search</Button>
    </div>

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