<script>
    import Update from "./Update.svelte";

    let activePage = this;
    let changePage = (page) =>{
        activePage = page;
    }
</script>

{#if activePage === this}
    <main class="bg-[#D9D9D9] h-screen">
        <nav class="fixed top-0 left-0 w-full bg-[#545454] p-6">
            <div class="text-center">
                <button class="mr-10 text-3xl font-['Pixelify_Sans'] text-[#00BF63] underline" on:click={() => changePage(this)}>INSTALL</button>
                <button class="text-3xl font-['Pixelify_Sans'] text-[#0CC0DF]" on:click={() => changePage(Update)}>UPDATE</button>
            </div>
        </nav>
        <div class="flex justify-center items-start h-screen">
            <div class="mt-24">
                <h1 class="text-7xl font-['Pixelify_Sans']">TYPE PACKAGE</h1>
                <div class="flex items-center mt-10">
                    <input type="text" name="package" class="border-4 border-black w-full p-2">
                    <button class="ml-4">
                        <div id="search"></div>
                    </button>
                </div>
            </div>
            
        </div>
    </main>
{:else if activePage === Update}
    <svelte:component this={Update}/>
{/if}