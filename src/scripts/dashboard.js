/* Fomi AI Assistant - Десктопный ИИ-ассистент
 Copyright (C) 2026 Maksim Fomin (Rourugin/Makss Mef)

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU Affero General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 GNU Affero General Public License for more details.

 You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/


import * as audioHandler from './audio_handler.js';
import * as api from './api.js';
import * as ui from './ui.js';


const dashInput = document.getElementById('chat-input');
const dashSendBtn = document.getElementById('send-btn');
const personalityContainer = document.getElementById('personality-container');
const closeBtn = document.getElementById('btn-close-dash');

const micBtn = document.getElementById('mic-btn');

const emit = window.__TAURI__.event?.emit;

let isRecording = false;
let currentPersonality = await api.getActivePersonality();
loadPersonalities();
InitNavigation();


dashSendBtn.addEventListener('click', async () => {
  const text = dashInput.value;
  if (!text) {
    return;
  }

  dashInput.value = '';
  await emit('fomi-think-request', text);
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

micBtn.addEventListener('click', async () => {
  if (!isRecording) {
    isRecording = true;
    micBtn.textContent = 'Stop';
    await audioHandler.startRecording();
  } else if (isRecording) {
    let result = await audioHandler.stopRecording();
    isRecording = false;
    micBtn.textContent = 'Speak';
    if (result) {
      dashInput.value = await api.processVoiceInput(result);
      dashSendBtn.click();
    }
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
