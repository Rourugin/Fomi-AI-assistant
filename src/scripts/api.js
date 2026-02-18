const { invoke } = window.__TAURI__.core;
export const setIgnoreCursor = (ignore) => invoke('set_ignore_cursor', { ignore });
export const fomiThink = (text) => invoke('fomi_think', { text });
export const fomiReset = (wipe_memory) => invoke('fomi_reset', { wipe_memory });
export const getPersonalities = () => invoke('get_personalities');
export const setPersonality = (name, wipe_memory) => invoke('set_personality', { name, wipe_memory });