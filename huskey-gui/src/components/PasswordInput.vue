<script setup lang="ts">
import { ref} from "vue";

const password = ref("");

const emit = defineEmits<{
  change: [password: string]
}>();

const emitPasswordChange = () => {
  emit("change", password.value);
}

</script>

<template>
  <input type="password" placeholder="Enter a password..." @change="emitPasswordChange" v-model="password"/>
</template>

<style scoped>

</style>
