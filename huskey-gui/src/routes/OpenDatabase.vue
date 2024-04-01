<script setup lang="ts">
import DatabaseSelector from "@/components/DatabaseSelector.vue";
import PasswordInput from "@/components/PasswordInput.vue";
import { databaseState, tryOpenDatabase } from "@/store/useDatabase";
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

// write the openDatabaseCommand as a function
async function openDatabaseCommand() {
  try{
    if(selectedPath.value === ""){
      await message("Please select a file to open");
      return;
    }
    await tryOpenDatabase(selectedPath.value, password.value);
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
          <h2>Open a database</h2>
          <a style="font-size: 2.5rem; color: #f7f7f7;" @click="openDatabaseCommand">🔑</a>
        </div>
      </div>
    </div>
    <div class="row">
      <div>
        <DatabaseSelector @selected="$path => selectedPath = $path" :is-selector="true"/>
      </div>
      <form>
        <PasswordInput @change="$password => password = $password "></PasswordInput>
        <button type="submit" @click="openDatabaseCommand">Open vault</button>
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
