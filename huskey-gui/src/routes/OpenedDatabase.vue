<script setup lang="ts">
import { databaseState, tryOpenDatabase } from "@/store/useDatabase";
import PasswordEntriesTable from "@/components/PasswordEntriesTable.vue";
import SaveDatabaseButton from "@/components/SaveDatabaseButton.vue";
import CreateEntryButon from "@/components/CreateEntryButon.vue";

</script>

<template>
    <CreateEntryButon></CreateEntryButon>
    <SaveDatabaseButton></SaveDatabaseButton>
    <PasswordEntriesTable :entries="databaseState!.database?.entries"></PasswordEntriesTable>
</template>

<style scoped>
</style>
