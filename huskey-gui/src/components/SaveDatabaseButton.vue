<script setup lang="ts">
import { databaseState, trySaveDatabase } from "@/store/useDatabase";

const saveDatabase = async () => {
  if(databaseState.cachedPassword !== null){
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
