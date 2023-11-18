<script setup lang="ts">
import { ref } from "vue";
const path = ref("");

const emit = defineEmits<{
}>()

const props = defineProps(['entry']);
</script>

<template>
    <tr>
        <td>{{ props.entry.name }}</td>
        <td>{{ props.entry.username }}</td>
        <td>{{ props.entry.password }}</td>
        <td v-if="props.entry.url">{{ props.entry.url }}</td>
    </tr>
</template>

<style scoped></style>
