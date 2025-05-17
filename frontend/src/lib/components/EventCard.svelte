<script lang="ts">
    import {Card} from "flowbite-svelte";
    import { ArrowRightOutline } from "flowbite-svelte-icons";
</script>

<Card img="/event-placeholder.png">
    <div class="m-6 flex flex-col items-center gap-1">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Noteworthy technology acquisitions 2021</h5>
        <p class="mb-3 leading-tight font-normal text-gray-700 dark:text-gray-400">Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.</p>
        <a href="event/123" class="cursor-pointer flex">
                Read more <ArrowRightOutline class="ms-2 h-6 w-6" />
        </a>
    </div>
</Card>