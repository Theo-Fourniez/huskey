<script setup lang="ts">
import { databaseState } from "@/store/useDatabase";
import { useRouter } from "vue-router";

const router = useRouter();
const goToNewEntry = async () => {
  router.push("/database/create");
}
</script>

<template>
  <button @click="goToNewEntry()" v-if="databaseState" style="background-color: #2d5bd3; font-size: 25px; padding: 5px;">Add</button>
</template>

<style scoped>

</style>
