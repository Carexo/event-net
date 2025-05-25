<script lang="ts">
    import {Card} from "flowbite-svelte";
    import { ArrowRightOutline } from "flowbite-svelte-icons";

    export let id: number;
    export let title: string;

    const img = `/events/${id % 10}.png`;
</script>

<Card {img} class="hover:shadow-xl transition-shadow">
    <div class="m-6 flex flex-col items-center justify-between gap-1 text-center h-full">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{title}</h5>
        <a href={`event/${id}`} class="cursor-pointer flex">
                Read more <ArrowRightOutline class="ms-2 h-6 w-6" />
        </a>
    </div>
</Card>