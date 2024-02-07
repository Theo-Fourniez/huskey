<script setup lang="ts">
import { PasswordEntry } from '@/types/huskeyTypes';
import { PropType, Ref, computed, ref } from "vue";


const thisEntry: Ref<PasswordEntry> = ref({
    name: "",
    username: "",
    password: "",
    url: "",
});


const props = defineProps({
    entry: {
        type: Object as PropType<PasswordEntry>,
        default: undefined
    }
});

const hasPassedEntry = computed(() => {
    return props.entry !== undefined;
});

const emit = defineEmits<{
  change: [entry: PasswordEntry]
}>();

const iconUrl = computed(() => {
    return hasPassedEntry.value ? props.entry!.url ? `https://www.google.com/s2/favicons?domain=${props.entry!.url}&sz=32` : undefined : undefined;
});

if (hasPassedEntry.value) {
    thisEntry.value = props.entry!;
}

const buttonClicked = (payload: MouseEvent) => {
    emit("change", thisEntry.value);
};
</script>

<template>
    <h2>Create an entry</h2>
    <img :src="iconUrl" class="icon">
    <input type="text" v-model="thisEntry.name" placeholder="Entry name, website name...">
    <input type="text" v-model="thisEntry.username" placeholder="Username...">
    <input type="text" v-model="thisEntry.password" placeholder="Password...">
    <input type="text" v-model="thisEntry.url" placeholder="Website URL...">
    <button @click="buttonClicked">Create</button>
</template>

<style scoped>
.icon {
    width: 32px;
    height: 32px;
}
</style>
