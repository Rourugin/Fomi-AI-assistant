import * as api from './api.js';
import * as ui from './ui.js';


const characterContainer = document.getElementById('character-container');
const contextMenu = document.getElementById('context-menu');

const listen = window.__TAURI__.event?.listen;

let isDragging = false;
let isInterfaceLocked = false;
let offsetX = 0;
let offsetY = 0;

window.addEventListener('DOMContentLoaded', () => {
  ui.preloadImages();
});

listen('avatar-state-change', (event) => {
  ui.setAvatarState(event.payload);
}).catch(console.error);

listen('show-subtitle', (event) => {
  ui.showSubtitle(event.payload);
}).catch(console.error);

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

characterContainer.addEventListener('mousedown', async (event) => {
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
  isInterfaceLocked = true;

  contextMenu.classList.add('hidden');
  await api.toggleDashboard();
});

document.getElementById('menu-reset').addEventListener('click', async () => {
  contextMenu.classList.add('hidden');
  try {
    const wipe = await askUserToWipeMemory();
    await api.fomiReset(wipe);
    ui.showSubtitle("Memory was resetted");
  } catch (e) {
    console.error(e);
    ui.showSubtitle("Memory reset eror");
  }
});

document.getElementById('menu-close').addEventListener('click', async () => {
  try {
    await api.quitApp();
  } catch (e) {
    console.error(e);
    ui.showSubtitle("Cannot close the program. Check console");
  }
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

characterContainer.addEventListener('mouseleave', async (event) => {
  if (isInterfaceLocked) {
    return;
  }

  if (event.relatedTarget && (event.relatedTarget.closest('#context-menu') || event.relatedTarget.closest('#dashboard-layer'))) {
    return;
  }
  await api.setIgnoreCursor(true)
});

window.addEventListener('focus', async () => {
  await api.setIgnoreCursor(false);
});


async function askUserToWipeMemory() {
  const modal = document.getElementById('confirmation-modal');
  const yesBtn = document.getElementById('btn-confirm-yes');
  const noBtn = document.getElementById('btn-confirm-no');

  modal.classList.remove('hidden');
  await api.setIgnoreCursor(false);

  return new Promise(async (resolve) => {
    const cleanup = () => {
      modal.classList.add('hidden');
      yesBtn.replaceWith(yesBtn.cloneNode(true));
      noBtn.replaceWith(noBtn.cloneNode(true));
    };

    document.getElementById('btn-confirm-yes').onclick = async () => {
      modal.classList.add('hidden');
      await api.setIgnoreCursor(true);
      resolve(true);
    };

    document.getElementById('btn-confirm-no').onclick = async () => {
      modal.classList.add('hidden');
      await api.setIgnoreCursor(true);
      resolve(false);
    };
  });
}