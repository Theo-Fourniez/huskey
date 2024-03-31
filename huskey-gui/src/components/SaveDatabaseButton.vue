<script setup lang="ts">
import { VueElement, ref, watch } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { databaseState, trySaveDatabase } from "@/store/useDatabase";
import { invoke } from "@tauri-apps/api/tauri";

const saveDatabase = async () => {
  if(databaseState.cachedPassword){
    await trySaveDatabase(databaseState.cachedPassword);
  }else{
    const password = prompt("Please enter a password to save the database");
    if(password){
      await trySaveDatabase(password);
    }
    else{
      alert("No password entered, database not saved");
    }
  }
}
</script>

<template>
  <button @click="saveDatabase()" v-if="databaseState && databaseState.hasUnsavedChanges">💾</button>
</template>

<style scoped>

</style>
