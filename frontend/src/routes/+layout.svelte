<script lang="ts">
	import '../styles/app.css';
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from "flowbite-svelte";
    import { Footer, FooterCopyright, FooterLinkGroup, FooterLink, DarkMode} from "flowbite-svelte";

	let { children } = $props();
</script>

<Navbar>
    <NavBrand href="/">
        <img src="/logo.png" class="me-3 h-9 sm:h-12" alt="Flowbite Logo" />
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">event net</span>
    </NavBrand>
    <NavHamburger />
    <NavUl>
        <NavLi href="/">Home</NavLi>
        <NavLi href="/recommended-events">Recommended</NavLi>
        <NavLi href="/my-events">My events</NavLi>
        <NavLi href="/users">Users</NavLi>
        <NavLi href="/events">All events</NavLi>
        <NavLi href="/event/add">Create event</NavLi>
    </NavUl>
</Navbar>
<main class="flex-grow">
    {@render children()}
</main>
<Footer>
    <FooterCopyright href="/" by="Marceli Mietła & Bartosz Czyż™" year={2025} />
    <FooterLinkGroup class="mt-3 flex flex-wrap items-center text-sm text-gray-500 sm:mt-0 dark:text-gray-400">
        <FooterLink href="/recommended-events">Recommended</FooterLink>
        <FooterLink href="/my-events">My events</FooterLink>
        <FooterLink href="/users">Users</FooterLink>
        <FooterLink href="/events">All events</FooterLink>
        <FooterLink href="/event/add">Create event</FooterLink>
    </FooterLinkGroup>
</Footer>