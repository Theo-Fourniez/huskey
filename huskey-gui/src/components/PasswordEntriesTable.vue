<script setup lang="ts">
import { PasswordEntry } from '@/types/huskeyTypes';
import PasswordEntryRow from './PasswordEntryRow.vue';


defineProps<{
    entries?: PasswordEntry[]
}>();

</script>

<template>
    <table>
        <thead>
            <th>Icon</th>
            <th>Name</th>
            <th>Username</th>
            <th>Password</th>
            <th>URL?</th>
        </thead>
        <tbody>
            <PasswordEntryRow v-if="entries" v-for="entry in entries" :entry="entry" />
            <div v-else>No entries in this database</div>
        </tbody>
    </table>
</template>

<style scoped>
table {
    border-collapse: collapse;
    margin: 25px 0;
    font-size: 0.9em;
    min-width: 400px;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
}
</style>
