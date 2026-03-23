import * as api from './api.js';


let audioContext = null;
let mediaStream = null;
let processorNode = null;
let inputNode = null
let audioChunks = [];


async function startRecording() {
    audioChunks = [];
    try {
        mediaStream = await navigator.mediaDevices.getUserMedia({ audio: true });
        audioContext = new AudioContext({ sampleRate: 16000 });
        inputNode = audioContext.createMediaStreamSource(mediaStream);
        processorNode = audioContext.createScriptProcessor(4096, 1, 1);
        audioChunks = [];

        inputNode.connect(processorNode);
        processorNode.connect(audioContext.destination);

        processorNode.onaudioprocess = (event) => {
            const pcm = event.inputBuffer.getChannelData(0);
            audioChunks.push(new Float32Array(pcm));
        };

        await audioContext.resume();
    } catch (e) {
        console.error("Microphone access error: ", e);
        audioContext = null;
        mediaStream = null;
        processorNode = null;
        inputNode = null;
        audioChunks = [];
        throw e;
    }
}

async function stopRecording() {
    if (processorNode === null || inputNode === null) {
        return null;
    }

    processorNode.disconnect();
    inputNode.disconnect();
    if (audioContext && audioContext.state !== 'closed') {
        await audioContext.close();
    }

    if (mediaStream) {
        mediaStream.getAudioTracks().forEach(track => {
            track.stop();
        });
    }

    let flatArray = flattenArray(audioChunks);
    let wavBytes = createWavFile(flatArray);

    audioContext = null;
    mediaStream = null;
    processorNode = null;
    inputNode = null;
    audioChunks = [];

    return wavBytes;
}

function flattenArray(chunks) {
    const totalLength = chunks.reduce((sum, chunk) => sum + chunk.length, 0);
    const result = new Float32Array(totalLength);
    let offset = 0;

    for (const chunk of chunks) {
        result.set(chunk, offset);
        offset += chunk.length;
    }

    return result;
}

function createWavFile(samples) {
    const numSamples = samples.length;
    const buffer = new ArrayBuffer(44 + numSamples * 2);
    const view = new DataView(buffer);

    writeString(view, 0, 'RIFF');
    view.setUint32(4, 36 + (numSamples * 2), true);
    writeString(view, 8, 'WAVE');

    writeString(view, 12, 'fmt ');
    view.setUint32(16, 16, true);
    view.setUint16(20, 1, true);
    view.setUint16(22, 1, true);
    view.setUint32(24, 16000, true);
    view.setUint32(28, 16000 * 2, true);
    view.setUint16(32, 2, true);
    view.setUint16(34, 16, true);

    writeString(view, 36, 'data');
    view.setUint32(40, numSamples * 2, true);

    let offset = 44;
    for (let i = 0; i < samples.length; i++, offset += 2) {
        let s = Math.max(-1, Math.min(1, samples[i]));
        view.setInt16(offset, s < 0 ? s * 0x8000 : s * 0x7FFF, true);
    }

    return new Uint8Array(buffer);
}

function writeString(view, offset, str) {
    for (let i = 0; i < str.length; i++) {
        view.setUint8(offset + i, str.charCodeAt(i));
    }
}