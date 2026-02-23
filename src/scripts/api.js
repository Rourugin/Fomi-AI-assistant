const { invoke } = window.__TAURI__.core;
export const setIgnoreCursor = (ignore) => invoke('set_ignore_cursor', { ignore });

export const fomiThink = (text) => invoke('fomi_think', { text });
export const fomiReset = (wipe) => invoke('fomi_reset', { wipe });
export const getPersonalities = () => invoke('get_personalities');
export const setPersonality = (name, wipe) => invoke('set_personality', { name, wipe });
export const getActivePersonality = () => invoke('get_active_personality');

export const quitApp = () => invoke('quit_app');