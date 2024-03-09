<script>
  import Install from "./Install.svelte";
  import Update from "./Update.svelte";

  let activePage = this;
  let changePage = (page) => {
      activePage = page;
  }
</script>

{#if activePage === this}
  <main>
      <div class="flex h-screen">
        <div class="m-auto">
            <h1 class="font-pixel mt-24 text-9xl mb-11">PA<span class="ml-3 -mt-7 inline-block align-middle" id="pacman"></span>MAN</h1>
            <div class="mt-32 flex justify-center">
                <button class="font-pixel mr-10 text-4xl text-white rounded-full bg-green pl-3 pr-3" on:click={() => changePage(Install)}>INSTALL</button>
                <button class="font-pixel text-4xl text-white rounded-full bg-sky-400 pl-3 pr-3" on:click={() => changePage(Update)}>UPDATE</button>
            </div>
        </div>
      </div>
  </main>
{:else if activePage === Install}
  <svelte:component this={Install}/>
{:else if activePage === Update}
  <svelte:component this={Update}/>
{/if}
