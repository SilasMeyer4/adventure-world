<script setup lang="ts">
import { onBeforeMount, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Editor from "./components/Editor.vue";
import ConfigEditor from "./components/ConfigEditor.vue";
import { errorMessages } from "vue/compiler-sfc";
import { app } from "@tauri-apps/api";
import { emit, listen} from "@tauri-apps/api/event";
import '@fortawesome/fontawesome-free/css/all.min.css';
import Sidebar from "./components/Sidebar.vue";
import { sidebarWidth } from "./components/SidebarState";
import { components } from "vuetify/dist/vuetify.js";
import "vue3-select/dist/vue3-select.css";
import LootMain from "./components/Loot/LootMain.vue"
import { currentMenu } from "./components/SidebarState";
import { NavbarMenus } from "./components/SidebarState";
import "./style.css"
import HomeMain from "./components/Home/HomeMain.vue";
import SettingsMain from "./components/SettingsMenu/SettingsMain.vue";
import DatabaseMain from "./components/Database/DatabaseMain.vue";
import CharactersMain from "./components/Characters/CharactersMain.vue";


const databaseState = ref("");


onMounted(() => {

listen("database_load", (event) => {
  console.log("Database load event:", event.payload);
});

}) 


</script>

<template>
  <Sidebar/>
  <main class="container">
    
    <HomeMain v-if="currentMenu === NavbarMenus.HOME"/>
    <CharactersMain v-if="currentMenu === NavbarMenus.CHARACTERS"/>
    <LootMain v-if="currentMenu === NavbarMenus.LOOT"/>
    <DatabaseMain v-if="currentMenu === NavbarMenus.DATABASE"/>
    <SettingsMain v-if="currentMenu === NavbarMenus.SETTINGS"/>
   
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  width: 100%; /* Ensure container takes full width */
   margin-left: 70px; /* Adjust width to collapsed siebar width */
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>