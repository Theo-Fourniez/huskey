<script setup lang="ts">


withDefaults(defineProps<{
    icon: string,
    placeholder?: string
}>(), {
    icon: 'fa-question',
    placeholder: undefined
});

const model = defineModel<any>(); // Proxy the 2 way binding of the input

const emit = defineEmits<{
  iconClick: [payload: MouseEvent]
}>();

</script>

<template>
    <div>
        <input type="text" :placeholder v-model="model" style="width: 100%;">
        <i :class="'fa ' + icon" @click="emit('iconClick', $event)"></i>
    </div>
</template>

<style scoped>
div{
    position: relative;
}

i{
    font-size: 1.3rem; 
    position: absolute; 
    top: 25%; 
    left: 105%;
    cursor: pointer;
    z-index: 10;
}

@media (prefers-color-scheme: dark) {
    i{
        background-color: var(--background-colour-dark-lighter);
    }
}
</style>
