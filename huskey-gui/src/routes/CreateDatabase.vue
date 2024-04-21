<script setup lang="ts">
import DatabaseSelector from "@/components/DatabaseSelector.vue";
import PasswordInput from "@/components/PasswordInput.vue";
import { databaseState, tryCreateDatabase } from "@/store/useDatabase";
import { message } from "@tauri-apps/api/dialog";
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
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
          <h2>Create a vault</h2>
          <a style="font-size: 2.5rem;">🛠️</a>
        </div>
      </div>
    </div>
    <div class="row" style="gap:2vw">
      <div>
        <DatabaseSelector @selected="$path => selectedPath = $path" :is-selector="false" />
      </div>
      <form class="column" @submit.prevent="createDatabaseCommand">
        <PasswordInput @change="($password) => password = $password"></PasswordInput>
        <button type="submit">Create vault</button>
      </form>
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
