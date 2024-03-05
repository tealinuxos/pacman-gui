<script>
    import Install from "./Install.svelte";

    let activePage = this;
    let changePage = (page) =>{
        activePage = page;
    }
</script>

{#if activePage === this}
    <main class="bg-[#D9D9D9]">
        <nav class="fixed top-0 left-0 w-full bg-[#545454] p-6">
            <div class="text-center">
                <button class="mr-10 text-3xl font-['Pixelify_Sans'] text-[#00BF63]" on:click={() => changePage(Install)}>INSTALL</button>
                <button class="text-3xl font-['Pixelify_Sans'] text-[#0CC0DF] underline" on:click={() => changePage(this)}>UPDATE</button>
            </div>
        </nav>
        <div class="flex h-screen">
            <div class="m-auto">
                <h1 class="text-4xl font-['Pixelify_Sans']">PRESS ICON TO UPDATE PACKAGE(S)</h1>
                <div class="mt-20 flex justify-center">
                    <button id="pacman"></button>
                </div>
            </div>
        </div>
    </main>
{:else if activePage === Install}
    <svelte:component this={Install}/>
{/if}