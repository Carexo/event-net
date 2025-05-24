<script lang="ts">
    import type {PageProps} from './$types';
    import {Heading, Card, Button} from "flowbite-svelte";

    let {data}: PageProps = $props();

    let selectedUser: string = $state(data.users[0].name);

    function selectUser(userName: string) {
        console.log(userName)
        selectedUser = userName;
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
            {#if selectedUser}
                <div class="mb-5 bg-green-100 p-3 rounded-md">
                    <p>Selected user: <strong>{selectedUser}</strong></p>
                </div>
            {/if}

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-full max-w-4xl">
                {#each data.users as user}
                    <Card
                          class={selectedUser === user.name ? 'outline-2 border-blue-500' : ''}
                          onclick={() => selectUser(user.name)}>
                        <div class="p-3 cursor-pointer">
                            <p class="font-semibold">{user.name}</p>
                        </div>
                    </Card>
                {/each}
            </div>
        </section>
    {/if}
</div>