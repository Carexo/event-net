<script lang="ts">
	import '../app.css';
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from "flowbite-svelte";
    import { Footer, FooterCopyright, FooterLinkGroup, FooterLink } from "flowbite-svelte";

	let { children } = $props();
</script>

<Navbar>
    <NavBrand href="/">
<!--        <img src="/images/flowbite-svelte-icon-logo.svg" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />-->
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">event net</span>
    </NavBrand>
    <NavHamburger />
    <NavUl>
        <NavLi href="/">Home</NavLi>
        <NavLi href="/events">All events</NavLi>
    </NavUl>
</Navbar>
{@render children()}
<Footer>
    <FooterCopyright href="/" by="Marceli Mietła & Bartosz Czyż™" year={2025} />
    <FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
        <FooterLink href="/events">All events</FooterLink>
    </FooterLinkGroup>
</Footer>
