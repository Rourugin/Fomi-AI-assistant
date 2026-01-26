const ASSETS = {
  idle: 'assets/fomi/fomi-idle.png',
  think: 'assets/fomi/fomi-think.png',
  talk: 'assets/fomi/fomi-talk.png'
};
const imageCache = {};

export const preloadImages = () => {
  Object.keys(ASSETS).forEach(key => {
    const img = new Image();
    img.src = ASSETS[key];
    imageCache[key] = img;
  });
  console.log('Images preloaded');
};

export const setAvatarState = (state) => {
  const avatar = document.getElementById('fomi-avatar');
  if (!avatar) return;

  if (ASSETS[state]) {
    avatar.src = ASSETS[state];
  }
};

export const showSubtitle = async (text) => {
  const subtitleBox = document.getElementById('subtitle-container');
  const subtitleText = document.getElementById('subtitle-text');

  subtitleBox.classList.remove('hidden');
  subtitleText.textContent = "";
  setAvatarState('talk');

  for (let i = 0; i < text.length; i++) {
    subtitleText.textContent += text[i];
    await new Promise(r => setTimeout(r, 30));
  }

  setAvatarState('idle');
  setTimeout(() => {
    subtitleBox.classList.add('hidden');
  }, 5000);
};

export const showPersonalities = async (names, currentPersonality) => {
  const personalityContainer = document.getElementById('personality-container');
  if (!personalityContainer) {
    return;
  }
  personalityContainer.innerHTML = '';

  for (let i = 0; i < names.length; i++) {
    const newBtn = document.createElement('button');
    newBtn.textContent = names[i];
    newBtn.classList.add('personality-btn');
    if (newBtn.textContent == currentPersonality) {
      newBtn.classList.add('active');
    }
    newBtn.id = 'btn-personality-${i}';
    personalityContainer.appendChild(newBtn);
  }
};