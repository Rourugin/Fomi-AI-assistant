const { invoke } = window.__TAURI__.core;

export const setIgnoreCursor = (ignore) => invoke('set_ignore_cursor', { ignore });

export const fomiThink = (text) => invoke('fomi_think', { text });
export const fomiReset = (wipe) => invoke('fomi_reset', { wipe });
export const setFomiAvatarState = (state) => invoke('set_fomi_avatar_state', { state });
export const getPersonalities = () => invoke('get_personalities');
export const setPersonality = (name, wipe) => invoke('set_personality', { name, wipe });
export const getActivePersonality = () => invoke('get_active_personality');

export const toggleDashboard = () => invoke('toggle_dashboard');
export const quitApp = () => invoke('quit_app');

export const processVoiceInput = (audio_bytes) => invoke('process_voice_input', { audioBytes: audio_bytes });
export const generateAudio = (text) => invoke('generate_audio', { text });