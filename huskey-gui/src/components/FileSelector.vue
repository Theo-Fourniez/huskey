<script setup lang="ts">
import { VueElement, ref, watch } from "vue";
import { open } from "@tauri-apps/api/dialog";

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

async function browseFile() {

  open({
    directory: false, multiple: false, title: 'Open the database', filters: [
      {
        name: "json",
        extensions: [
          'json'
        ]
      }
    ]
  })
    .then((selectedPath) => {
      if (selectedPath === null || Array.isArray(selectedPath)) return;
      path.value = selectedPath;
      console.debug("[FileSelector] Selected path ", selectedPath);
      emit("selected", selectedPath);
    })
    .catch((err) => {
      console.error(err);
    });
}

watch(path, (newPath) => {
  console.debug("[FileSelector] Path changed to ", newPath);
  emit("selected", newPath);
});
</script>

<template>
  <form class="row" @submit.prevent="browseFile">
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
}

@media (prefers-color-scheme: dark) {
  textarea {
    background-color: #0f0f0f98;
  }
}
</style>
