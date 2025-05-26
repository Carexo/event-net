<script lang="ts">
    import { page } from '$app/stores';
    import { Heading, Button } from 'flowbite-svelte';
    import { ArrowLeftOutline } from 'flowbite-svelte-icons';
</script>

<div class="flex flex-col items-center justify-center px-6 py-16 min-h-[70vh] m-auto text-center">
    <Heading tag="h1" class="text-6xl font-bold mb-4 text-blue-600">404</Heading>
    <Heading tag="h2" class="text-3xl font-semibold mb-4">Page not found</Heading>

    <p class="text-lg text-gray-600 dark:text-gray-400 mb-8 max-w-lg">
        Sorry, we couldn't find the page you're looking for. It might have been moved or doesn't exist.
    </p>

    <Button href="/" color="blue" size="xl" class="mt-4">
        <ArrowLeftOutline class="mr-2 h-5 w-5" />
        Back to Home
    </Button>
</div>