const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let byeMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function say_bye() {
  byeMsgEl.textContent = await invoke("say_bye", { input: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  byeMsgEl = document.querySelector("#bye-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    say_bye();
    greet();
  });
});
