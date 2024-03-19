<script>
    import Install from "./Install.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import LoadingBar from "./LoadingBar.svelte";

    let activePage = this;
    let changePage = (page) =>{
        activePage = page;
    }

    let isUpdating = false;
    let isUpToDate = false;

    const updatePackage = async () => {

        isUpToDate = false;
        isUpdating = true;

        await invoke("upgrade_system").then((success) => {
            if (success) {
                isUpToDate = true;
            }
            else
            {
                console.log("update failed");
                //todo
            }
        })

        isUpdating = false;
        console.log("update done");
    };
</script>

{#if activePage === this}
    <main>
        <nav class="fixed top-0 left-0 w-full bg-black p-6">
            <div class="text-center">
                <button class="mr-10 text-3xl font-pixel text-green" on:click={() => changePage(Install)}>INSTALL</button>
                <button class="text-3xl font-pixel text-cyan underline" on:click={() => changePage(this)}>UPDATE</button>
            </div>
        </nav>
        <div class="flex h-screen">
            <div class="m-auto">
                {#if !isUpdating}
                    <h1 class="text-4xl font-pixel">PRESS ICON TO UPDATE PACKAGE(S)</h1>
                {/if}
                <div class="mt-20 flex justify-center">
                {#if isUpToDate}
                    <div class="flex flex-col justify-center items-center">
                        <button id="pacman-uptodate" on:click={updatePackage}></button>
                        <span class="text-4xl font-pixel mt-16 text-green">PACKAGE(S) UP TO DATE</span>
                    </div>
                    <!--need to change css -->
                {:else if isUpdating}
                    <div class="flex flex-col justify-center items-center">
                        <button class="updatePacman" disabled={true} id="pacman-updating" on:click={updatePackage}>
                            <div class="pacman"></div>
                            <div class="dot"></div>
                        </button>
                        <span class="text-4xl font-pixel mt-16 text-black">UPDATING PACKAGE(S)</span>
                        <LoadingBar/>

                    </div>
                {:else}
                <!--  -->
                    <div>
                        <button id="pacman" on:click={updatePackage}></button>
                    </div>
                {/if}
                </div>
            </div>
        </div>
    </main>
{:else if activePage === Install}
    <svelte:component this={Install}/>
{/if}
