class RecorderProcessor extends AudioWorkletProcessor {
    process(inputs, outputs, parameters) {
        const input = inputs[0];

        if (input && input.length > 0) {
            const channelData = input[0];
            if (channelData && channelData.length > 0) {
                const copy = new Float32Array(channelData);
                this.port.postMessage(copy);
            }
        }

        return true;
    }
}


registerProcessor('recorder-worklet', RecorderProcessor);
