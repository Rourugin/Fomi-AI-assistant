import * as api from './api.js';
import * as ui from './ui.js';


const dashInput = document.getElementById('chat-input');
const dashSendBtn = document.getElementById('send-btn');
const personalityContainer = document.getElementById('personality-container');

let isThink = false;
let currentPersonality = await api.getActivePersonality();
loadPersonalities();


dashSendBtn.addEventListener('click', async () => {
  const text = dashInput.value;
  if (!text) {
    return;
  }

  dashInput.value = '';
  await thinkInput(text);
});

dashInput.addEventListener('keydown', (event) => {
  if (event.key == 'Enter') {
    dashSendBtn.click();
  }
});

personalityContainer.addEventListener('click', async (event) => {
  const clickedBtn = event.target.closest('.personality-btn');

  if (!clickedBtn) {
    return;
  }
  currentPersonality = clickedBtn.title;
  await api.setPersonality(currentPersonality, false);
  loadPersonalities();
});

document.getElementById('btn-close-dash').addEventListener('click', async () => {
  await api.toggleDashboard();
});


async function loadPersonalities() {
  const names = await api.getPersonalities();
  for (let i = 0; i < names.length; i++) {
    if (names[i] == 'standard') {
      names.splice(i, 1);
      break;
    }
  }
  names.sort();
  names.unshift('standard');
  await ui.showPersonalities(names, currentPersonality);
}

async function thinkInput(text) {
  if (isThink) {
    return
  };
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