<script setup lang="ts">
import { computed, ref } from "vue";
const path = ref("");

const emit = defineEmits<{
}>()

const props = defineProps(['entry']);

const iconUrl = computed(() => {
    return props.entry.url ? `https://www.google.com/s2/favicons?domain=${props.entry.url}&sz=32` : '';
});

</script>

<template>
    <tr>
        <td><img :src="iconUrl"></td>
        <td>{{ props.entry.name }}</td>
        <td>{{ props.entry.username }}</td>
        <td>{{ props.entry.password }}</td>
        <td v-if="props.entry.url">{{ props.entry.url }}</td>
    </tr>
</template>

<style scoped></style>
