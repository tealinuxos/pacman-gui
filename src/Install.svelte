<script>
    import Update from "./Update.svelte";
    import InstallButton from "./InstallButton.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let activePage = this;
    let changePage = (page) =>{
        activePage = page;
    }

    let packageName;
    let showResult = false;
    let searchResult;
    let installedPackages;
    let selectedPackages;
    let isInstalled = false;
    let showInstallModal = false;
    let installMessage = "SELECT PACKAGE(S)";
    let isInstalling = false;
    let isDone = false;


    let e;

    onMount(function() {
        e.focus();
    });

    const searchPackage = async () => {
        await invoke("search_package", {packageName: packageName}).then((response) => searchResult = response);
        await invoke("installed", {packagesName: searchResult}).then((response) => installedPackages = response);
        showResult = true;
        showInstallModal = true;
    };

    const installPackage = async () => {

        isInstalling = true;

        for (const [id, name] of Object.entries(selectedPackages))
        {
            installMessage = "Installing " + name + " ...";
            await invoke("install_package", {packageName: name})
                .then((response) => isInstalled = response);
        }

        isInstalling = false;
        isDone = true;

        await invoke("installed", {packagesName: searchResult}).then((response) => installedPackages = response);
    }
</script>

{#if activePage === this}
    <main>
        <nav class="fixed top-0 left-0 w-full bg-black p-6 z-50">
            <div class="text-center">
                <button class="mr-10 text-3xl font-pixel text-green underline" on:click={() => changePage(this)}>INSTALL</button>
                <button class="text-3xl font-pixel text-cyan" on:click={() => changePage(Update)}>UPDATE</button>
            </div>
        </nav>
        <div class="flex justify-center items-start h-screen">
            <div class="mt-24">
                <h1 class="text-7xl font-pixel">TYPE PACKAGE</h1>
                    <form class="flex items-center mt-10 w-full h-full" on:submit|preventDefault={searchPackage}>
                        <input bind:this={e} bind:value={packageName} type="text" name="package" class="border-4 border-black w-full p-2">
                        <button class="ml-4">
                            <div id="search"></div>
                        </button>
                    </form>
                <div class="my-10 font-pixel">
                    {#if showResult}
                        <span class="flex justify-center text-2xl"></span>
                        {#each Object.entries(searchResult) as [key, value]}
                            <div class="relative bg-white shadow-lg my-5 p-5 rounded-lg">
                                <div class="flex justify-between items-center text-3xl rounded">
                                    <span>{value}</span>
                                    <input type="checkbox" on:change={() => isDone = false} bind:group={selectedPackages} value={value} class="w-6 h-6 checked:w-8 checked:h-8" />
                                </div>
                                {#if Object.values(installedPackages).includes(value)}
                                    <span class="text-green">Installed</span>
                                {/if}
                            </div>
                        {/each}
                    {/if}
                </div>
            </div>
            <div class="fixed flex justify-center items-center bottom-0 font-pixel bg-black w-full h-20">
            {#if showInstallModal}
                {#if selectedPackages != undefined && Object.keys(selectedPackages).length != 0}
                    <InstallButton {isInstalling} {installMessage} {installPackage} {isDone} isTicked={true}>
                    {#if isDone}
                        {Object.keys(selectedPackages).length} Package(s) Installed
                    {:else if Object.keys(selectedPackages).length == 1}
                        Install Package
                    {:else}
                        Install {Object.keys(selectedPackages).length} package(s)
                    {/if}
                    </InstallButton>
                {:else}
                    <InstallButton {isInstalling} {installPackage} >
                        <span>select package(s)</span>
                    </InstallButton>
                {/if}
            {/if}
            </div>
        </div>
    </main>
{:else if activePage === Update}
    <svelte:component this={Update}/>
{/if}
