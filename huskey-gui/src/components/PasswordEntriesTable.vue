<script setup lang="ts">
import { ref } from "vue";


import PasswordEntry from "./PasswordEntry.vue";

const props = defineProps(['entries']);
</script>

<template>
    <table>
        <thead>
            <th>Name</th>
            <th>Username</th>
            <th>Password</th>
            <th>URL?</th>
        </thead>
        <tbody>
            <PasswordEntry v-if="props.entries" v-for="entry in props.entries" :entry="entry" />
            <div v-else>No entries in this database</div>
        </tbody>
    </table>
</template>

<style scoped></style>
