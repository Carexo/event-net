<script lang="ts">
    import type {PageProps} from './$types';
    import {Card, Heading} from "flowbite-svelte";
    import { PaginationNav } from "flowbite-svelte";
    import {ArrowLeftOutline, ArrowRightOutline} from "flowbite-svelte-icons";
    import {selectedUser, selectUser} from '$lib/stores/userStore';
    import {goto} from '$app/navigation';

    let {data}: PageProps = $props();

    if (data.users && data.users.length > 0 && !$selectedUser) {
        selectUser(data.users[0].name);
    }

    function handleSelectUser(userName: string) {
        selectUser(userName);
    }

    function handlePageChange(page: number) {
        goto(`/users?page=${page}`);
    }
</script>

<div class="text-center">
    <Heading tag="h1" class="py-5">Users List</Heading>

    {#if data.error}
        <div class="text-red-500 p-8">
            <p>Error: {data.error}</p>
        </div>
    {:else if data.users.length === 0}
        <div class="p-8">
            <p>No users found.</p>
        </div>
    {:else}
        <section class="p-5 flex flex-col items-center">
            {#if $selectedUser}
                <div class="mb-5 bg-green-100 p-3 rounded-md">
                    <p>Selected user: <strong>{$selectedUser}</strong></p>
                </div>
            {/if}

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-full max-w-4xl">
                {#each data.users as user}
                    <Card
                            class={$selectedUser === user.name ? 'outline-2 border-blue-500' : ''}
                            onclick={() => selectUser(user.name)}>
                        <div class="p-3 cursor-pointer">
                            <p class="font-semibold">{user.name}</p>
                        </div>
                    </Card>
                {/each}
            </div>

        </section>

        <div class="mt-6 ">
            <PaginationNav currentPage={data.pagination.page} totalPages={data.pagination.pages} onPageChange={handlePageChange}>
                {#snippet prevContent()}
                    <span class="sr-only">Previous</span>
                    <ArrowLeftOutline class="h-5 w-5"/>
                {/snippet}
                {#snippet nextContent()}
                    <span class="sr-only">Next</span>
                    <ArrowRightOutline class="h-5 w-5"/>
                {/snippet}
            </PaginationNav>
            <p class="mt-2.5 text-sm text-gray-600">
                Showing page {data.pagination.page} of {data.pagination.pages}
                ({data.pagination.total} total users)
            </p>
        </div>
    {/if}
</div>