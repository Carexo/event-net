<script lang="ts">
    import type { PageProps } from './$types';
    import { Heading, Img } from "flowbite-svelte";

    let { data }: PageProps = $props();
    console.log(data)
</script>

<section class="flex gap-5 justify-between w-full">
    <div>
        <Heading tag="h1" class="py-5 text-center">{data.event.title}</Heading>
    </div>
    <Img src="/event-placeholder.png" alt="sample 1" class="h-96" />
</section>