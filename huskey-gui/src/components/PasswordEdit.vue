<script setup lang="ts">
import { addPasswordEntry, editPasswordEntry, removePasswordEntry } from "@/store/useDatabase";
import { PasswordEntry } from "@/types/huskeyTypes";
import { computed, onMounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
import InputIcon from "./InputIcon.vue";

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

const submitEditForm = async () => {
  emitEvent();
  if(isNewEntry.value){
    await addPasswordEntry(entry);
    router.back();
  } else{
    await editPasswordEntry(initialEntry, entry);
    initialEntry = {...entry};
  }
}

const deleteEntry = async () => {
  if(!isNewEntry.value){
    await removePasswordEntry(entry);
    emitEvent();
  }
}

const title = computed(() => { return isNewEntry.value ? "Add entry" : "Edit entry"});
const callToAction = computed(() => { return isNewEntry.value ? "Add" : "Save"});

watch(props, ()=>{
  entry = props.entry;
});

const copyToClipboard = async (toCopy: string) => {
  return navigator.clipboard.writeText(toCopy);
}
</script>

<template>
  <div class="column" style="gap: 1rem; align-items: center;">
    <h1>{{ title }}</h1>
    <form class="column" style="justify-content: center; align-items: center; gap: 0.75rem;" @submit.prevent="submitEditForm">
      <InputIcon v-model="entry.name" icon="fa-copy" placeholder="Enter name" @icon-click="copyToClipboard(entry.name)"></InputIcon>
      <InputIcon v-model="entry.username" icon="fa-copy" placeholder="Enter username" @icon-click="copyToClipboard(entry.username)"></InputIcon>
      <InputIcon v-model="entry.password" icon="fa-copy" placeholder="Enter a password..." @icon-click="copyToClipboard(entry.password)"></InputIcon>
      <InputIcon v-model="entry.url" icon="fa-copy" placeholder="Enter URL" @icon-click="copyToClipboard(entry.url ?? '')"></InputIcon>
      <button type="submit" style="background-color: var(--color-dark-accent);">{{ callToAction }}</button>
    </form>
    <div class="row" v-if="!isNewEntry">
      <button style="justify-self: center" @click="deleteEntry">Delete</button>
    </div>
  </div>


</template>

<style scoped>
</style>
