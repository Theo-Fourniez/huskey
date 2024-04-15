<script setup lang="ts">
import { addPasswordEntry, editPasswordEntry } from "@/store/useDatabase";
import { PasswordEntry } from "@/types/huskeyTypes";
import { computed, onMounted, ref, watch } from "vue";
import { useRouter } from "vue-router";

// If this component is used to create a new entry (the entry prop is not passed or is default)
// it will navigate back to the previous page after adding the entry
// If the entry prop is passed, it will edit the entry and assume its not used as a route

// https://vuejs.org/api/sfc-script-setup.html#default-props-values-when-using-type-declaration
export interface Props {
  entry?: PasswordEntry
}

const props = withDefaults(defineProps<Props>(), {
  entry: () => {
    return {
      name: "",
      password: "",
      username: "",
    }
  }
});

const emit = defineEmits<{edited: [edited: PasswordEntry]}>();

const emitEvent = () => {
  emit("edited", entry);
}

const router = useRouter();

const isNewEntry = ref(true);
// The entry that is being edited (reactive)
let entry = props.entry;
// The entry first passed unedited (non reactive)
let initialEntry = {...props.entry};

onMounted(() => {
  isNewEntry.value = entry.name === "" && entry.username === "" && entry.password === "" && (entry.url === "" || entry.url === undefined);
});

const submitForm = async () => {
  emitEvent();
  if(isNewEntry.value){
    await addPasswordEntry(entry);
    router.back();
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
    <form style="display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 10px;" @submit.prevent="submitForm">
      <input type="text" placeholder="Enter name" v-model="entry.name">
      <input type="text" placeholder="Enter username" v-model="entry.username">
      <input type="password" placeholder="Enter a password..." v-model="entry.password">
      <input type="text" placeholder="Enter URL" v-model="entry.url">
      <button type="submit">{{ callToAction }}</button>
    </form>
  </div>


</template>

<style scoped>
</style>
