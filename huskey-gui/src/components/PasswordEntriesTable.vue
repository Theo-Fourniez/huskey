<script setup lang="ts">
import { PasswordEntry } from '@/types/huskeyTypes';
import PasswordEntryRow from './PasswordEntryRow.vue';
import { useRouter } from 'vue-router';
import { ref } from 'vue';
import PasswordEdit from '@/components/PasswordEdit.vue';


defineProps<{
    entries?: PasswordEntry[]
}>();

const editingEntry = ref<PasswordEntry | null>(null);

const editEntry = (entry: PasswordEntry) => {
    console.log("clicked ", entry);
    editingEntry.value = editingEntry.value == entry ? null : entry;
}

</script>

<template>
    <div class="row" style="height: 100vh; overflow-y: scroll;">
        <div class="column" style="margin: 45px; gap:20px; flex-grow: 1; flex-shrink: 1; overflow-y: scroll; min-height: fit-content;">
            <PasswordEntryRow v-if="entries" v-for="entry in entries" :entry="entry" @edit="editEntry($event)"/>
            <div v-else>No entries in this database</div>
        </div>
        <div style="position: sticky; flex-grow: 2; top:0px;" v-if="editingEntry">
            <PasswordEdit :entry="editingEntry"></PasswordEdit>
        </div>
    </div>
</template>

<style scoped>
</style>
