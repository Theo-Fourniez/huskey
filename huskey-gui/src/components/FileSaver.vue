<script setup lang="ts">
import { ref, watch } from "vue";
import {  save } from "@tauri-apps/api/dialog";
import { desktopDir, extname } from '@tauri-apps/api/path';

const path = ref("");

const emit = defineEmits<{
  selected: [path: string]
}>();

const props = defineProps({
  browseBtnText: {
    type: String,
    default: "Browse"
  },
  pathPlaceholder: {
    type: String,
    default: "Enter a path ..."
  }
});


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
  console.debug("[FileSaver] Path changed to ", newPath);
  emit("selected", newPath);
});
</script>

<template>
  <form class="row" @submit.prevent="saveFile">
    <textarea id="file-path-input" v-model="path" :placeholder="pathPlaceholder">
    </textarea>
    <button type="submit">{{ props.browseBtnText }}</button>  
  </form>
  
</template>

<style scoped>
textarea {
  resize: none;
  border: none;
  outline: none;
  font-size: 1.2em;
  white-space: pre;
  vertical-align: middle;
  padding:5px;
  line-height: 43px;
}

@media (prefers-color-scheme: dark) {
  textarea {
    background-color: #0f0f0f98;
  }
}
</style>
