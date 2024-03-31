<script setup lang="ts">
import { databaseState, tryCloseDatabase, isDatabaseOpen } from "@/store/useDatabase";
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const goToNewEntry = async () => {
  router.push("/database/create");
}

const route = useRoute();

const isOnLandingScreen = computed(() => {
  return route.path === "/";
});

const closeDatabase = async () => {
  router.push("/"); // Go back to landing screen
  await tryCloseDatabase();
}

const isOnDatabaseScreen = computed(() => {
  return route.path === "/database";
});


const isAfterDatabaseScreen = computed(() => {
  return route.path.includes("/database") && route.path !== "/database";
});
</script>

<template>
    <div class="header">
    <div v-if="!isOnLandingScreen && (!isDatabaseOpen && !isOnDatabaseScreen) || (isDatabaseOpen && isAfterDatabaseScreen)" @click="router.back()" style="cursor: pointer;" >
      <i class="fas fa-arrow-left" style="font-size: large;"></i>
    </div>
    <div v-if="isDatabaseOpen" style="cursor: pointer;" @click="closeDatabase();" >
      <a style="font-size: x-large;">🔒</a>
    </div>
  </div>
</template>

<style scoped>
.header{
  display: flex;
  justify-content: flex-start;
  align-items: center;
  padding: 1vw;
  gap: 1vw;
  color: #ffffffd8;
  font-size: 0.75rem;
  background-color: #343434;
}
</style>

