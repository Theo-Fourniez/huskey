<script setup lang="ts">
import { onMounted, reactive, ref, watch } from "vue";
import { PasswordEntry } from "@/types/huskeyTypes";
import { addPasswordEntry, editPasswordEntry } from "@/store/useDatabase";
import { useRouter } from "vue-router";

const router = useRouter();

const props = defineProps<{
  entry: PasswordEntry
}>();

const isNewEntry = ref(true);

let entry = props.entry;
let initialEntry = {...props.entry};

const emit = defineEmits<{edited: [edited: PasswordEntry]}>();

const emitEdit = () => {
  emit("edited", entry);
}

onMounted(() => {
  console.log("onMounted")
  isNewEntry.value = entry.name === "" && entry.username === "" && entry.password === "" && (entry.url === "" || entry.url === undefined);
});

const submitForm = async () => {
  console.log("submitForm")
  emitEdit();
  console.log("isNewEntry", isNewEntry.value)
  if(isNewEntry.value){
    await addPasswordEntry(entry);
  } else{
    await editPasswordEntry(initialEntry, entry);  
    initialEntry = {...entry};
  }
}

watch(props, ()=>{
  entry = props.entry;
  console.log(props);
});
</script>

<template>
  <div>
    <h1>Add a new password</h1>
    <form style="display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 10px;">
      <input type="text" placeholder="Enter name" v-model="entry.name">
      <input type="text" placeholder="Enter username" v-model="entry.username">
      <input type="password" placeholder="Enter a password..." v-model="entry.password">
      <input type="text" placeholder="Enter URL" v-model="entry.url">
      <button type="submit" @click="submitForm()">Add</button>
    </form>
  </div>


</template>

<style>

</style>
