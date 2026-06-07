import * as api from './api.js';


const { getCurrentWindow } = window.__TAURI__.window;
const listen = window.__TAURI__.event?.listen;


async function checkSystem() {
    try {
        const sysInfo = await api.getSysInfo();
        const deps = await api.getDeps();

        document.getElementById('free-space').innerText = `${sysInfo.free_space_gb.toFixed(2)} GB`;
        document.getElementById('total-ram').innerText = `${sysInfo.total_ram_gb.toFixed(2)} GB`;

        updateStatus('m-llm', deps.has_main_model);
        updateStatus('m-embedder', deps.has_embedder_model);
        updateStatus('m-whisper', deps.has_whisper);
        updateStatus('m-piper', deps.has_piper);
        updateStatus('m-voiceover', deps.has_voiceover && deps.has_voiceover_json);

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

async function initWindow() {
    const appWindow = getCurrentWindow();

    const unlisten = await appWindow.onCloseRequested(async (event) => {
        const confirmed = await confirm('Are you sure you want to close?');

        if (confirmed) {
            event.preventDefault();
            await api.quitApp();
        }
    });
}

async function download(component_type, model_id) {
    try {
        await api.startDownload(component_type, model_id);
    }
    catch (e) {
        console.error(e);
    }
}


window.addEventListener('DOMContentLoaded', async () => {
    await checkSystem();
    await checkDownloadable();
    initWindow();
});

document.getElementById('llm-btn').addEventListener('click', async () => {
    const model_id = document.getElementById('llm-selector').value;

    await download("llm", model_id);
});

document.getElementById('embedder-btn').addEventListener('click', async () => {
    const model_id = document.getElementById('embedder-selector').value;

    await download("embedder", model_id);
});

document.getElementById('whisper-btn').addEventListener('click', async () => {
    const model_id = document.getElementById('whisper-selector').value;

    await download("whisper", model_id);
});

document.getElementById('piper-btn').addEventListener('click', async () => {
    const model_id = document.getElementById('piper-selector').value;

    await download("piper", model_id);
});

document.getElementById('voiceover-btn').addEventListener('click', async () => {
    const model_id = document.getElementById('voiceover-selector').value;

    await download("voiceover", model_id);
});


listen('download_progress', (event) => {
    let component_id = event.payload.id;

    let progress_bar = document.getElementById(`${component_id}-progress`);
    let selector = document.getElementById(`${component_id}-selector`);
    let btn = document.getElementById(`${component_id}-btn`);
    let badge = document.getElementById(`m-${component_id}`);

    let downloaded_procent = Math.floor((event.payload.downloaded / event.payload.total) * 100);

    progress_bar.value = downloaded_procent;
    progress_bar.classList.remove('hidden');
    selector.classList.add('hidden');
    btn.classList.add('hidden');

    badge.innerText = `Downloaded: ${downloaded_procent}`;
});