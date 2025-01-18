<script lang="ts" setup>

    import { computed } from 'vue'
    import { useRouter, useRoute } from 'vue-router'

    const route = useRoute()

    const inSettings = computed(() => {
        return route.path === "/settings";
    })

</script>

<template>

    <nav class="navbar">
        <div class="doc-nav center">
            <RouterLink to="/" :class="[!inSettings ? 'selected' : '']">Document Generation</RouterLink>
        </div>
        <div class="temp-nav center">
            <h2 id="title">JSTG</h2>
        </div>
        <div class="config-nav center">
            <RouterLink to="/settings" :class="[inSettings ? 'selected' : '']">Settings</RouterLink>
        </div>
    </nav>

</template>

<style lang="scss" scoped>

    @use '../variables';

    .navbar {

        padding: 5px;
        display: grid;
        grid-template-columns: 1fr 3fr 1fr;
        grid-template-rows: 1fr;
        grid-column-gap: 50px;
        grid-row-gap: 15px;
        width: stretch;

        border: 5px solid variables.$secondary-color;

        // row col row col
        .doc-nav { grid-area: 1 / 1 / 1 / 1; }
        .temp-nav { grid-area: 1 / 2 / 1 / 2; }
        .config-nav { grid-area: 1 / 3 / 1 / 3; }


    }

    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .selected {
        border-bottom: 2px solid variables.$shadow-color;
    }

    #title {
        text-decoration: solid;
    }

</style>
