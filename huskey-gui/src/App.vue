<script setup lang="ts">
import AppNavbar from './components/AppNavbar.vue';

// For debugging purposes
window.addEventListener('unhandledrejection', function (event) {
    alert('An error occurred: ' + event.reason);
    console.log('An error occurred: ', event.reason);
});

</script>

<template>
    <AppNavbar></AppNavbar>
    <router-view></router-view>
</template>

<style>
</style>
