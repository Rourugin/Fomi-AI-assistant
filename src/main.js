const { invoke } = window.__TAURI__.core;

let greetInputEl;
let installPluginInputEl;
let greetMsgEl;
let byeMsgEl;
let pluginListMsgEl;
let installPluginMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function say_bye() {
  byeMsgEl.textContent = await invoke("say_bye", { input: greetInputEl.value });
}

async function get_active_plugins() {
  let active_plugins = await invoke("get_active_plugins", {  });
  pluginListMsgEl.textContent = active_plugins.join("\n");
}

async function install_plugin() {
  installPluginMsgEl.textContent = "New plugin installed successfully!";
  await invoke("install_plugin", { name: installPluginInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  installPluginInputEl = document.querySelector("#install-plugin-input");
  greetMsgEl = document.querySelector("#greet-msg");
  byeMsgEl = document.querySelector("#bye-msg");
  pluginListMsgEl = document.querySelector("#get-active-plugins-msg");
  installPluginMsgEl = document.querySelector("#install-plugin-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    say_bye();
    greet();
  });
  document.querySelector("#get-active-plugins-form").addEventListener("submit", (e) => {
    e.preventDefault();
    get_active_plugins();
  });
  document.querySelector("#install-plugin-form").addEventListener("submit", (e) => {
    e.preventDefault();
    install_plugin();
  });
});
