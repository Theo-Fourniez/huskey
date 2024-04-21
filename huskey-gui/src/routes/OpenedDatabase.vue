<script setup lang="ts">
import CreateEntryButon from "@/components/CreateEntryButon.vue";
import PasswordEntriesTable from "@/components/PasswordEntriesTable.vue";
import SaveDatabaseButton from "@/components/SaveDatabaseButton.vue";
import { databaseState } from "@/store/useDatabase";

</script>

<template>
    <CreateEntryButon></CreateEntryButon>
    <SaveDatabaseButton></SaveDatabaseButton>
    <PasswordEntriesTable v-if="databaseState.database?.entries && databaseState.database.entries.length > 0" :entries="databaseState.database?.entries"></PasswordEntriesTable>
    <h1 id="empty-text">The database is empty, click on the button at the top to start</h1>
</template>

<style scoped>
#empty-text{
    color: var(--color-dark-text);
    font-size: 1.5rem;
    text-align: center;
    font-weight: normal;
}
</style>
