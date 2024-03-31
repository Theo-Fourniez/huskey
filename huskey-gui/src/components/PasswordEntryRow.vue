<script setup lang="ts">
import { PasswordEntry } from "@/types/huskeyTypes";
import { getFavicon } from "@/utilities";
import { computed, ref } from "vue";
import EntryIcon from "./EntryIcon.vue";

const props = defineProps<{
    entry: PasswordEntry
}>();

</script>

<template>
    <div class="entry-container" @click="$emit('edit', props.entry)">
        <EntryIcon :url="props.entry.url"></EntryIcon>
        <div class="entry-text">
            <p style="font-weight: bold;">{{props.entry.name}}</p>
            <p>{{ props.entry.username }}</p>
        </div>
        <div class="cta-icons-end">
            <i class="fa-solid fa-arrow-right" style="font-size: 20px;"></i>
        </div>
    </div>
</template>

<style scoped>
.entry-container {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    background-color: var(--background-colour-dark-lighter);
}
.entry-text {
    margin-left: 10px;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.entry-text p {
    margin: 0;
}

.cta-icons-end {
    display: flex;
    align-items: center;
    margin-left: auto;
    gap: 10px;
}
</style>
