import * as api from './api.js';

const invoke = window.__TAURI__.core.invoke;


async function checkSystem() {
    try {
        document.getElementById('free-space').innerText = `${api.sysInfo.free_space_gb.toFixed(2)} GB`;
        document.getElementById('total-ram').innerText = `${api.sysInfo.total_ram_gb.toFixed(2)} GB`;

        updateStatus('m-main', api.deps.has_main_model);
        updateStatus('m-embed', api.deps.has_embedder_model);
        updateStatus('m-whisper', api.deps.has_whisper);
        updateStatus('m-piper', api.deps.has_piper);
        updateStatus('m-voice', api.deps.has_voiceover && api.deps.has_voiceover_json);

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


window.addEventListener('DOMContentLoaded', checkSystem);