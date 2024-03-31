<script setup lang="ts">
import FileSelector from "@/components/FileSelector.vue"
import { computed, ref, watch } from "vue";
import { message } from "@tauri-apps/api/dialog";
import { databaseState, tryCreateDatabase } from "@/store/useDatabase";
import PasswordInput from "@/components/PasswordInput.vue";
import { useRouter } from "vue-router";
import FileSaver from "@/components/FileSaver.vue";
const selectedPath = ref<string>("");
const password = ref<string>("");

const router = useRouter();

watch(databaseState, () => {
  if(databaseState.database){
    router.push("/database");
  }
});

// write the createDatabaseCommand as a function
async function createDatabaseCommand() {
  try{
    if(selectedPath.value === ""){
      await message("Please select a path to create a database");
      return;
    }
    console.log(selectedPath.value + " T " + password.value);
    await tryCreateDatabase(selectedPath.value, password.value);
  }catch(e){
    if(e instanceof Object){
      await message(Object.values(e).join("\n"), {title: "Error", type: "error"});
    }
    console.error(e);
  }
}

</script>

<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="card">
          <h2>Create a database</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;" @click="createDatabaseCommand">🛠️</a>
        </div>
      </div>
    </div>
    <div class="row">
      <div>
        <FileSaver @selected="$path => selectedPath = $path" />
      </div>
      <div>
        <PasswordInput @change="($password) => password = $password"></PasswordInput>
        <button @click="createDatabaseCommand">Open database</button>
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
