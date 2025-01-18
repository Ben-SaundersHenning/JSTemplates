<script lang="ts" setup>

    import Generation from "./Generation.vue"
    import { invoke } from "@tauri-apps/api/core";

</script>

<template>
    <div id="parent">
        <Generation/>
    </div>

</template>

<style lang="scss" scoped>

#parent {

    display: flex;
    flex-direction: column;

}

</style>
