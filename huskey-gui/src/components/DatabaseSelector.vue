<script setup lang="ts">
import { VueElement, ref, watch } from "vue";
import { open, save } from "@tauri-apps/api/dialog";
import { desktopDir, extname } from "@tauri-apps/api/path";
const path = ref("");

const emit = defineEmits<{
  selected: [path: string]
}>();

const props = defineProps({
  buttonText: {
    type: String,
    default: "Select"
  },
  pathPlaceholder: {
    type: String,
    default: "Enter a path ..."
  },
  isSelector: {
    type: Boolean,
    default: true
  }
});

async function selectFile() {
  const selectedPath = await open({
    directory: false, multiple: false, title: 'Open the database', filters: [
      {
        name: "Huskey Database",
        extensions: [
          'huskey'
        ]
      }
    ]
  });
  if (selectedPath === null || Array.isArray(selectedPath)) return;
  path.value = selectedPath;
  emit("selected", selectedPath);
}

async function saveFile() {
  const desktopPath = await desktopDir();

  const selectedPath = await save({
    title: 'Save the database', defaultPath: desktopPath , filters: [
      {
        name: "Huskey Database",
        extensions: [
          'huskey'
        ]
      }
    ]
  });
  if (selectedPath === null || Array.isArray(selectedPath)) return;
  path.value = selectedPath;
  console.debug("[FileSaver] Selected path ", selectedPath);
  if(!(path.value.lastIndexOf('.') > 1 && path.value.lastIndexOf('.') < path.value.length - 1)){
    path.value += ".huskey";
  }
  else{
    path.value = path.value.slice(0, path.value.lastIndexOf('.')) + ".huskey";
  }
  emit("selected", selectedPath);
}

watch(path, (newPath) => {
  emit("selected", newPath);
});
</script>

<template>
  <form class="row" @submit.prevent="isSelector ? selectFile() : saveFile()">
    <input v-model="path" :placeholder="pathPlaceholder" contenteditable="true">
    </input>
    <button>{{ props.buttonText }}</button>
  </form>
</template>

<style>
</style>
