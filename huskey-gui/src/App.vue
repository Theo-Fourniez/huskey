<script setup lang="ts">
import { register } from '@tauri-apps/api/globalShortcut';
import AppNavbar from './components/AppNavbar.vue';
register("CommandOrControl+S", () => {
    console.log("Save database shortcut pressed");
    // saveDatabase(databaseState.cachedPassword);
});

</script>

<template>
    <AppNavbar></AppNavbar>
    <router-view></router-view>
</template>

<style>
</style>
