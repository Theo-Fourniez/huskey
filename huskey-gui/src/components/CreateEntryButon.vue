<script setup lang="ts">
import { databaseState } from "@/store/useDatabase";
import { useRouter } from "vue-router";

const router = useRouter();
const goToNewEntry = async () => {
  router.push("/database/create");
}
</script>

<template>
  <button @click="goToNewEntry()" v-if="databaseState">Create</button>
</template>

<style scoped>

</style>
