<script lang="ts">
    import {Card} from "flowbite-svelte";
    import { ArrowRightOutline } from "flowbite-svelte-icons";

    export let id: string;
    export let img: string = "/event-placeholder.png";
    export let title: string;
    export let description: string;
</script>

<Card {img} class="hover:shadow-xl transition-shadow">
    <div class="m-6 flex flex-col items-center gap-1 text-center">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{title}</h5>
        <p class="mb-3 leading-tight font-normal text-gray-700 dark:text-gray-400">{description}</p>
        <a href={`event/${id}`} class="cursor-pointer flex">
                Read more <ArrowRightOutline class="ms-2 h-6 w-6" />
        </a>
    </div>
</Card>