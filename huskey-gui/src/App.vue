<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FileSelector from "./components/FileSelector.vue";
import PasswordEntry from "./components/PasswordEntry.vue";
import { invoke } from '@tauri-apps/api/tauri'

import { HuskeyDatabase } from '@/types/huskeyTypes';
import { handleError, ref } from "vue";
import PasswordEntriesTable from "./components/PasswordEntriesTable.vue";
import { message } from "@tauri-apps/api/dialog";

const db = ref<HuskeyDatabase | undefined>(undefined);
const selectedPath = ref<string>("")
// write the openDatabaseCommand as a function
async function openDatabaseCommand() {
  console.log("Open database")
  try {
    db.value = await invoke<HuskeyDatabase>('open_database', { path: selectedPath.value, password: 'password' });
    console.log("Set the database value");
  } catch (e) {
    console.error(e);
    if (e instanceof Object) {
      await message(Object.values(e)[0]
        , { title: "Could not open database", type: "error", "okLabel": "Ok" });
    }
  }

}

</script>

<template>
  <!-- A little header in the top left corner with the title of the app-->
  <div class="header">
    <h1>Huskey</h1>
  </div>
  <div>
    <div class="container">
      <div class="row">
        <div class="card">
          <h2>Create a database</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;">🛠️</a>
        </div>
        <div class="card">
          <h2>Open a database</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;" @click="openDatabaseCommand">🔑</a>
        </div>
      </div>
    </div>
    <PasswordEntriesTable v-if="db && db.entries" :entries="db?.entries" />
    <h1 v-else>No entries</h1>
    <FileSelector @selected="$path => selectedPath = $path" />
    <button @click="openDatabaseCommand">Open database</button>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.card {
  display: flex;
  padding: 1.25vw;
  flex-direction: column;
  flex-wrap: wrap;
  transition: ease-in-out 500ms;

}

.card:hover {
  filter: drop-shadow(0 0 2em #747bff);
  color: #747bff;
}

.header{
  display: flex;
  justify-content: flex-start;
  align-items: center;
  padding: 1vw;
  color: #ffffffd8;
  font-size: 0.75rem;
  background-color: #343434;
}
</style>
