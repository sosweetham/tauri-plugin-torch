<style>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
}
</style>

<script>
import Greet from "./lib/Greet.svelte";
import { ping, toggle, check } from "tauri-plugin-torch-api";

let response = "";

function updateResponse(returnValue) {
    response += `[${new Date().toLocaleTimeString()}]
        ${
            typeof returnValue === "string"
                ? returnValue
                : JSON.stringify(returnValue)
        }
        "<br>"`;
}

async function toggleTorch() {
    const isTorchOn = await check();
    if (isTorchOn) {
        await toggle(false);
        updateResponse("Torch is off");
    } else {
        await toggle(true);
        updateResponse("Torch is on");
    }
}

async function onTorch() {
    toggle(true).then(updateResponse).catch(updateResponse);
}

async function offTorch() {
    toggle(false).then(updateResponse).catch(updateResponse);
}

function _ping() {
    ping("Pong!").then(updateResponse).catch(updateResponse);
}
</script>

<main class="container">
    <button onclick={onTorch}> Turn On Torch </button>
    <button onclick={offTorch}> Turn Off Torch </button>
    <button onclick={toggleTorch}> Toggle Torch </button>
    <h1>Welcome to Tauri!</h1>

    <div class="row">
        <a href="https://vite.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
        </a>
        <a href="https://svelte.dev" target="_blank">
            <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
        </a>
    </div>

    <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

    <div class="row">
        <Greet />
    </div>

    <div>
        <button onclick={_ping}>Ping</button>
        <div>{@html response}</div>
    </div>
</main>
