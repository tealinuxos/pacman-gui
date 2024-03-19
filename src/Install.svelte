<script>

    import Update from "./Update.svelte";
    import InstallButton from "./InstallButton.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import SelectedPackagesStore from "./lib/stores/SelectedPackagesStore.js";

    let activePage = this;
    let changePage = (page) =>{
        activePage = page;
    }

    let packageName;
    let showResult = false;
    let searchResult;
    let installedPackages;
    let selectedPackages = [];
    let isInstalled = false;
    let showInstallModal = false;
    let installMessage = "SELECT PACKAGE(S)";
    let isInstalling = false;
    let isDone = false;
    let isDisabled = false;

    SelectedPackagesStore.subscribe((data) => {
        selectedPackages = data;
    });

    const handleChange = (e) => {

        isDone = false;
        
        SelectedPackagesStore.update((current) => {
            if (e.target.checked) {
                return [e.target.value, ...current];
            }
            else {
                return current.filter((pkg) => pkg != e.target.value);
            }
        });
    };

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
        isDisabled = true;

        for (const [id, name] of Object.entries(selectedPackages))
        {
            installMessage = "Installing " + name + " ...";
            await invoke("install_package", {packageName: name})
                .then((response) => {
                    isInstalled = response;
                    isDisabled = !response;
                });

            await invoke("installed", {packagesName: searchResult}).then((response) => installedPackages = response);
        }

        isInstalling = false;
        isDone = true;
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
                        <input bind:this={e} bind:value={packageName} type="text" placeholder="Type Something" name="package" class="border-4 border-black w-full p-2 placeholder:font-pixel text-xl font-pixel" disabled={isDisabled}>
                        <button class="ml-4" disabled={isDisabled}>
                            <div class="bg-gray rounded-full p-6 outline">
                                <div id="search"></div>
                            </div>
                        </button>
                    </form>

                    <hr class="absolute  left-0 w-full bg-black p-[2px] z-50 mt-6">

                <div class="my-10 pb-[60px] font-pixel mt-[3rem]">
                    {#if showResult}
                        <span class="flex justify-center text-2xl"></span>
                        {#each Object.entries(searchResult) as [key, value]}
                        <div class="relative bg-white shadow-lg my-5 p-5 rounded-3xl">
                            {#if Object.values(installedPackages).includes(value)}
                                <div class="flex justify-between items-center text-3xl rounded">
                                    <span>{value}</span>
                                    <button>unin<button/> <!-- todo: bikin button uninstal -->
                                </div>
                                <span class="text-green">Installed</span>
                            {:else}
                                <div class="flex justify-between items-center text-3xl rounded">
                                    <span>{value}</span>
                                    <div class="checkbox-wrapper-24">
                                        <input type="checkbox" id={"checkbox" + key} on:change={handleChange} bind:group={selectedPackages} value={value} disabled={isDisabled}/>
                                        <label for={"checkbox" + key}>
                                          <span></span>
                                        </label>
                                      </div>
                                </div>
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
