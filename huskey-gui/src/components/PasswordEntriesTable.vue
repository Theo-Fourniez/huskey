<script setup lang="ts">
import PasswordEdit from '@/components/PasswordEdit.vue';
import { PasswordEntry } from '@/types/huskeyTypes';
import { ref } from 'vue';
import PasswordEntryRow from './PasswordEntryRow.vue';


defineProps<{
    entries?: PasswordEntry[]
}>();

const editingEntry = ref<PasswordEntry | null>(null);

const editEntry = (entry: PasswordEntry) => {
    editingEntry.value = editingEntry.value == entry ? null : entry; // Close the edit form if the same entry is clicked twice
}

</script>

<template>
    <div class="row" style="height: 90vh; overflow-y: scroll;">
        <div class="column" style="margin: 45px; gap:20px; flex-grow: 1; flex-shrink: 1; overflow-y: scroll;">
            <PasswordEntryRow v-if="entries" v-for="entry in entries" :entry="entry" @edit="editEntry($event)"/>
            <div v-else>No entries in this database</div>
        </div>
        <div style="position: sticky; flex-grow: 2; top:0px; overflow-y: scroll;" v-if="editingEntry">
            <PasswordEdit :entry="editingEntry" @edited="editingEntry = null"></PasswordEdit>
        </div>
    </div>
</template>

<style scoped>
</style>
