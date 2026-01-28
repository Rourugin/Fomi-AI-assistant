import * as api from './api.js';
import * as ui from './ui.js';


const { getCurrentWindow } = window.__TAURI__.window;

const characterContainer = document.getElementById('character-container');
const contextMenu = document.getElementById('context-menu');
const dashboard = document.getElementById('dashboard-layer');

const dashInput = document.getElementById('dashboard-input');
const personalityContainer = document.getElementById('personality-container');
const dashSendBtn = document.getElementById('dashboard-send');
const dashCloseBtn = document.getElementById('btn-close-dash');

let isThink = false;
let currentPersonality = 'standard';
ui.preloadImages();

characterContainer.addEventListener('contextmenu', (e) => {
  e.preventDefault();

  if (e.target.id === 'fomi-avatar') {
      contextMenu.style.left = `${e.clientX}px`;
      contextMenu.style.top = `${e.clientY}px`;
      contextMenu.classList.remove('hidden');
  }
});

document.addEventListener('mousedown', (e) => {
  if (!contextMenu.classList.contains('hidden') && !contextMenu.contains(e.target)) {
    contextMenu.classList.add('hidden');
  }
});

document.getElementById('menu-open-dashboard').addEventListener('click', () => {
  loadPersonalities();
  dashboard.classList.remove('hidden');
  contextMenu.classList.add('hidden');
});

document.getElementById('menu-reset').addEventListener('click', async () => {
  contextMenu.classList.add('hidden');
  try {
    await api.fomiReset();
    ui.showSubtitle("Memory was resetted");
  } catch (e) {
    console.error(e);
    ui.showSubtitle("Memory reset eror");
  }
});

document.getElementById('menu-close').addEventListener('click', async () => {
  try {
    const appWindow = getCurrentWindow();
    await appWindow.close();
  } catch (e) {
    console.error(e);
    ui.showSubtitle("Cannot close the program. Check console");
  }
});

dashCloseBtn.addEventListener('click', () => {
  dashboard.classList.add('hidden');
});

dashSendBtn.addEventListener('click', async () => {
  const text = dashInput.value;
  if (!text) return;

  dashInput.value = '';
  dashboard.classList.add('hidden');
  await thinkInput(text);
});

personalityContainer.addEventListener('click', async (event) => {
  const clickedBtn = event.target.closest('.personality-btn');
  if (!clickedBtn) {
    return;
  }
  currentPersonality = clickedBtn.title;
  await api.setPersonality(currentPersonality);
  loadPersonalities();
});


async function thinkInput(text) {
  if (isThink) return;
  isThink = true;
  ui.setAvatarState('think');

  try {
    const response = await api.fomiThink(text);
    await ui.showSubtitle(response);
  } catch (e) {
    console.error(e);
    await ui.showSubtitle("AI Error: " + e);
  } finally {
    isThink = false;
    ui.setAvatarState('idle');
  }
}

async function loadPersonalities() {
  const names = await api.getPersonalities();
  dashboard.style.cursor = "wait";
  for (let i = 0; i < names.length; i++) {
    if (names[i] == 'standard') {
      names.splice(i, 1);
      break;
    }
  }
  names.sort();
  names.unshift('standard');
  await ui.showPersonalities(names, currentPersonality);
  dashboard.style.cursor = "default";
}