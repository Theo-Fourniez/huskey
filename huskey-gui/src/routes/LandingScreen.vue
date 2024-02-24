<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FileSelector from "@/components/FileSelector.vue"
import { invoke } from '@tauri-apps/api/tauri'
import { ref, watch } from "vue";
import { message } from "@tauri-apps/api/dialog";
import { HuskeyDatabase } from "../types/huskeyTypes"
import { database, tryOpenDatabase } from "@/store/useDatabase";
import PasswordEntriesTable from "@/components/PasswordEntriesTable.vue";

let selectedPath = ref<string>("");

watch(database, () => {
  console.log("Database changed", database.value);
  if(database.value){
    console.log("Database has entries", database.value.entries);
  }
});

// write the openDatabaseCommand as a function
async function openDatabaseCommand() {
  console.log("Open database")
  try{
    if(selectedPath.value === ""){
      await message("Please select a file to open");
      return;
    }
    await tryOpenDatabase(selectedPath.value, "password");
  }catch(e){
    if(e instanceof Object){
      await message(Object.values(e).join("\n"), {title: "Error", type: "error"});
    }
    console.error(e);
  }
}

</script>

<template>
  <!-- A little header in the top left corner with the title of the app-->
  <div class="header">
    <h1>Huskey router</h1>
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
    <PasswordEntriesTable v-if="database && database.entries" :entries="database?.entries" />
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
