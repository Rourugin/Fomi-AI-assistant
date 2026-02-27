import * as api from './api.js';
import * as ui from './ui.js';


const dashInput = document.getElementById('chat-input');
const dashSendBtn = document.getElementById('send-btn');
const personalityContainer = document.getElementById('personality-container');
const closeBtn = document.getElementById('btn-close-dash');

let isThink = false;
let currentPersonality = await api.getActivePersonality();
loadPersonalities();
InitNavigation();


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

closeBtn.addEventListener('click', async () => {
  await api.toggleDashboard();
});

document.addEventListener('keydown', (event) => {
  if (event.altKey) {
    let targetKey = '';

    switch (event.key) {
      case '1':
        targetKey = 'home';
        break;
      case '2':
        targetKey = 'memory';
        break;
      case '3':
        targetKey = 'plugins';
        break;
      case '4':
        targetKey = 'settings';
        break;
    }

    if (targetKey) {
      const btnToClick = document.querySelector(`.nav-btn[data-target="${targetKey}"]`);
      if (btnToClick) {
        btnToClick.click();
      }
    }
  }

  if (event.key == 'Escape') {
    closeBtn.click();
  }
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

function InitNavigation() {
  const navBtns = document.querySelectorAll('.nav-btn[data-target]');
  const viewSections = document.querySelectorAll('.view-section');

  navBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      navBtns.forEach(button => button.classList.remove('active'));
      btn.classList.add('active');
      const targetName = btn.getAttribute('data-target');
      const targetSectionId = `view-${targetName}`;

      viewSections.forEach(section => {
        if (section.id === targetSectionId) {
          section.classList.remove('hidden');
          section.classList.add('active');
        } else {
          section.classList.add('hidden');
          section.classList.remove('active');
        }
      });

      localStorage.setItem('lastView', targetName);
    });
  });
}