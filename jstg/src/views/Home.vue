<script lang="ts" setup>

    import Generation from "./Generation.vue"
    import { invoke } from "@tauri-apps/api/core";

    function getSettings() {

        // invoke('get_settings').then((cur_settings) => console.log(cur_settings));

    }

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
