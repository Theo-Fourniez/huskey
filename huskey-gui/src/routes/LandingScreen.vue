<script setup lang="ts">
import { useRouter } from "vue-router";
const router = useRouter();
</script>

<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="card" @click="router.push('/create')">
          <h2>Create a vault</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;">🛠️</a>
        </div>
        <div class="card" @click="  router.push('/open')">
          <h2>Open a vault</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;">🔑</a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.card {
  display: flex;
  padding: 1.25vw;
  flex-direction: column;
  flex-wrap: wrap;
  transition: ease-in-out 500ms;

}

.card:hover {
  filter: drop-shadow(0 0 2em #747bff);
  color: #747bff;
}
</style>
