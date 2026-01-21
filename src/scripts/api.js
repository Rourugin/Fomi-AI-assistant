const { invoke } = window.__TAURI__.core;
export const fomiThink = (text) => invoke('fomi_think', { text });
export const fomiReset = () => invoke('fomi_reset');