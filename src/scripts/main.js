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
let isDragging = false;
let offsetX = 0;
let offsetY = 0;
let currentPersonality = 'standard';
ui.preloadImages();


characterContainer.addEventListener('mouseenter', async () => {
    await api.setIgnoreCursor(false);
});

characterContainer.addEventListener('contextmenu', (event) => {
    event.preventDefault();

  if (event.target.id === 'fomi-avatar') {
      contextMenu.style.left = `${event.clientX}px`;
      contextMenu.style.top = `${event.clientY}px`;
      contextMenu.classList.remove('hidden');
  }
});

characterContainer.addEventListener('mousedown', (event) => {
  isDragging = true;
  characterContainer.style.cursor = "grabbing";

  const rect = characterContainer.getBoundingClientRect();
  offsetX = event.clientX - rect.left;
  offsetY = event.clientY - rect.top;
});

document.addEventListener('mousedown', (event) => {
  if (!contextMenu.classList.contains('hidden') && !contextMenu.contains(event.target)) {
    contextMenu.classList.add('hidden');
  }
});

document.getElementById('menu-open-dashboard').addEventListener('click', async () => {
  const avatar = document.getElementById('fomi-avatar');
  const avatarRect = avatar.getBoundingClientRect();

  dashboard.classList.remove('hidden');
  await loadPersonalities();

  positionDashboardNearAvatar(avatarRect);
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

dashCloseBtn.addEventListener('click', async () => {
  dashboard.classList.add('hidden');
});

dashSendBtn.addEventListener('click', async () => {
  const text = dashInput.value;
  if (!text) return;

  dashInput.value = '';
  dashboard.classList.add('hidden');
  await thinkInput(text);
});

dashInput.addEventListener('keydown', (event) => {
  if (event.key === 'Enter') {
    dashSendBtn.click();
  }
})

personalityContainer.addEventListener('click', async (event) => {
  const clickedBtn = event.target.closest('.personality-btn');
  if (!clickedBtn) {
    return;
  }
  currentPersonality = clickedBtn.title;
  await api.setPersonality(currentPersonality);
  loadPersonalities();
});

document.addEventListener('mousemove', (event) => {
  if (!isDragging) {
    return;
  }

  const newX = event.clientX - offsetX;
  const newY = event.clientY - offsetY;

  characterContainer.style.position = "absolute";
  characterContainer.style.left = newX + 'px';
  characterContainer.style.top = newY + 'px';
});

document.addEventListener('mouseup', async (event) => {
    isDragging = false;
    characterContainer.style.cursor = "grab";
});

characterContainer.addEventListener('mouseleave', async () => {
  await api.setIgnoreCursor(true);
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

function positionDashboardNearAvatar(avatarRect) {
  const dashRect = dashboard.getBoundingClientRect();
  const margin = 20;

  let left = avatarRect.right + margin;
  let top = avatarRect.top;

  if (left + dashRect.width > window.innerWidth) {
    left = avatarRect.left - dashRect.width - margin;
  }
  if (top + dashRect.height > window.innerHeight) {
    top = window.innerHeight - dashRect.height - margin;
  }
  if (top < margin) {
    top = margin;
  }

  dashboard.style.left = `${left}px`;
  dashboard.style.top = `${top}px`;
}