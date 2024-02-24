<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from '@tauri-apps/api/tauri'

import { HuskeyDatabase } from '@/types/huskeyTypes';
import { ref } from "vue";
import { message } from "@tauri-apps/api/dialog";
import { database } from "@/store/useDatabase";

</script>

<template>
    <router-view></router-view>
</template>

<style>
</style>
