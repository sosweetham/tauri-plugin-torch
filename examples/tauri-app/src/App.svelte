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
import {
    disable,
    enable,
    isAvailable,
    isEnabled,
    toggle,
} from "@sosweetham/tauri-plugin-torch-api";

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
    toggle()
        .then((on) => updateResponse(on ? "Torch is on" : "Torch is off"))
        .catch(updateResponse);
}

async function onTorch() {
    enable()
        .then(() => updateResponse("Torch is on"))
        .catch(updateResponse);
}

async function offTorch() {
    disable()
        .then(() => updateResponse("Torch is off"))
        .catch(updateResponse);
}

async function checkTorch() {
    Promise.all([isAvailable(), isEnabled()])
        .then(([available, enabled]) => updateResponse({ available, enabled }))
        .catch(updateResponse);
}
</script>

<main class="container">
    <button onclick={onTorch}> Turn On Torch </button>
    <button onclick={offTorch}> Turn Off Torch </button>
    <button onclick={toggleTorch}> Toggle Torch </button>
    <button onclick={checkTorch}> Check Torch </button>
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
</main>
