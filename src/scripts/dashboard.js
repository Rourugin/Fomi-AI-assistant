import * as api from './api.js';
import * as ui from './ui.js';


document.getElementById('btn-close-dash').addEventListener('click', async () => {
    await api.toggleDashboard();
})