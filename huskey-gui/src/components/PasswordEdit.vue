<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from "vue";
import { PasswordEntry } from "@/types/huskeyTypes";
import { addPasswordEntry, editPasswordEntry } from "@/store/useDatabase";
import { useRouter } from "vue-router";

const router = useRouter();

const props = defineProps<{
  entry: PasswordEntry
}>();

const isNewEntry = ref(true);

let entry = {...props.entry};
let initialEntry = {...props.entry};

const emit = defineEmits<{edited: [edited: PasswordEntry]}>();

const emitEdit = () => {
  emit("edited", entry);
}

onMounted(() => {
  isNewEntry.value = entry.name === "" && entry.username === "" && entry.password === "" && (entry.url === "" || entry.url === undefined);
});

const submitForm = async () => {
  emitEdit();
  if(isNewEntry.value){
    await addPasswordEntry(entry);
  } else{
    await editPasswordEntry(initialEntry, entry);  
    initialEntry = {...entry};
  }
}

const title = computed(() => { return isNewEntry.value ? "Add entry" : "Edit entry"});
const callToAction = computed(() => { return isNewEntry.value ? "Add" : "Save"});

watch(props, ()=>{
  entry = props.entry;
});
</script>

<template>
  <div>
    <h1>{{ title }}</h1>
    <form @submit.prevent="submitForm" style="display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 10px;">
      <input type="text" placeholder="Enter name" v-model="entry.name">
      <input type="text" placeholder="Enter username" v-model="entry.username">
      <input type="password" placeholder="Enter a password..." v-model="entry.password">
      <input type="text" placeholder="Enter URL" v-model="entry.url">
      <button>{{ callToAction }}</button>
    </form>
  </div>


</template>

<style scoped>
</style>
