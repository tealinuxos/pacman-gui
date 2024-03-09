<script>
    import Install from "./Install.svelte";

    export let installPackage;
    export let isInstalling = false;
    export let isTicked = false;
    export let installMessage;
    export let isDone = false;
</script>

{#if isDone}
    <button enabled={false} class="bg-blue px-16 py-3 rounded-full text-white text-3xl uppercase">
        <slot />
    </button>
{:else if isInstalling}
    <button enabled={false} class="bg-gray px-16 py-3 rounded-full text-black text-3xl uppercase">
        {installMessage}
    </button>
{:else if isTicked}
    <button enabled={true} on:click={installPackage} class="bg-green px-16 py-3 rounded-full text-white text-3xl uppercase">
        <slot />
    </button>
{:else}
    <button enabled={false} class="bg-gray px-16 py-3 rounded-full text-black text-3xl uppercase">
        <slot />
    </button>
{/if}
