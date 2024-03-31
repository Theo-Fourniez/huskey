<script setup lang="ts">
import { PropType, VueElement, computed, onMounted, reactive, ref, watch } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { PasswordEntry } from "@/types/huskeyTypes";
import { addPasswordEntry } from "@/store/useDatabase";
import { useRoute, useRouter } from "vue-router";
import { getFavicon } from "@/utilities";

const route = useRoute();

const router = useRouter();

const entry = reactive<PasswordEntry>(route.params.entry as any || {
  name: "",
  password: "",
  username: "",
  url: undefined
});

const emit = defineEmits<{edited: [edited: PasswordEntry]}>();

const emitEdit = () => {
  emit("edited", entry);
}

const isNewEntry = ref(true);

onMounted(() => {
  isNewEntry.value = entry.name === "" && entry.username === "" && entry.password === "" && (entry.url === "" || entry.url === undefined);
});


const submitForm = async () => {
  emitEdit();
  if(isNewEntry){
    await addPasswordEntry(entry);
    router.back();
  }
}

</script>

<template>
  <div>
    <h1>Add a new password</h1>
    <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 10px;">
      <input type="text" placeholder="Enter name" v-model="entry.name">
      <input type="text" placeholder="Enter username" v-model="entry.username">
      <input type="password" placeholder="Enter a password..." v-model="entry.password">
      <input type="text" placeholder="Enter URL" v-model="entry.url">
      <button @click="submitForm">Add</button>
    </div>
  </div>


</template>

<style scoped>

</style>
