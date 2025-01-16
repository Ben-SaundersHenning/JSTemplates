<script lang="ts" setup>

    import Generation from "./Generation.vue"
    import TopBar from "../components/TopBar.vue"
    import { invoke } from "@tauri-apps/api/core";

    function getSettings() {

        // invoke('get_settings').then((cur_settings) => console.log(cur_settings));

    }

</script>

<template>
    <!-- <button @click="getSettings">Get Settings</button> -->
    <div id="parent">
        <TopBar/>
        <Generation/>
    </div>

</template>

<style lang="scss" scoped>

#parent {

    display: flex;
    flex-direction: column;

}

</style>
