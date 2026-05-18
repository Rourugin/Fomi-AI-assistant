import * as api from './api.js';


const { getCurrentWindow } = window.__TAURI__.window;


async function checkSystem() {
    try {
        const sysInfo = await api.getSysInfo();
        const deps = await api.getDeps();

        document.getElementById('free-space').innerText = `${sysInfo.free_space_gb.toFixed(2)} GB`;
        document.getElementById('total-ram').innerText = `${sysInfo.total_ram_gb.toFixed(2)} GB`;

        updateStatus('m-main', deps.has_main_model);
        updateStatus('m-embed', deps.has_embedder_model);
        updateStatus('m-whisper', deps.has_whisper);
        updateStatus('m-piper', deps.has_piper);
        updateStatus('m-voice', deps.has_voiceover && deps.has_voiceover_json);

    } catch (err) {
        console.error("Calling Tauri command error:", err);
    }
}

function updateStatus(elementId, isReady) {
    const el = document.getElementById(elementId);
    if (isReady) {
        el.innerText = "Ready";
        el.className = "status-badge status-ready";
    } else {
        el.innerText = "Missing";
        el.className = "status-badge status-missing";
    }
}

async function checkDownloadable() {
    const deps = await api.getDeps();

    if (deps.has_main_model){
        document.getElementById('llm-selector').classList.add('hidden');
        document.getElementById('llm-btn').classList.add('hidden');
    }

    if (deps.has_embedder_model){
        document.getElementById('embedder-selector').classList.add('hidden');
        document.getElementById('embedder-btn').classList.add('hidden');
    }

    if (deps.has_whisper){
        document.getElementById('whisper-selector').classList.add('hidden');
        document.getElementById('whisper-btn').classList.add('hidden');
    }

    if (deps.has_piper){
        document.getElementById('piper-selector').classList.add('hidden');
        document.getElementById('piper-btn').classList.add('hidden');
    }

    if (deps.has_voiceover && deps.has_voiceover_json){
        document.getElementById('voiceover-selector').classList.add('hidden');
        document.getElementById('voiceover-btn').classList.add('hidden');
    }
}

async function InitWindow() {
    const appWindow = getCurrentWindow();

    const unlisten = await appWindow.onCloseRequested(async (event) => {
        const confirmed = await confirm('Are you sure you want to close?');

        if (confirmed) {
            event.preventDefault();
            await api.quitApp();
        }
    });
}


window.addEventListener('DOMContentLoaded', async () => {
    await checkSystem();
    await checkDownloadable();
    InitWindow();
});