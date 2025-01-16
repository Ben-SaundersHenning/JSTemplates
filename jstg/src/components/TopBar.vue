<script lang="ts" setup>

</script>

<template>

    <nav class="navbar">
        <div class="doc-nav center">
            <p>docs</p>
        </div>
        <div class="config-nav center">
            <p>settings</p>
        </div>
    </nav>

</template>

<style lang="scss" scoped>

    @use '../variables';

    .navbar {

        padding: 5px;
        display: grid;
        grid-template-columns: 1fr 1fr 7fr 1fr;
        grid-template-rows: 1fr;
        grid-column-gap: 50px;
        grid-row-gap: 15px;
        width: stretch;

        border: 5px solid variables.$secondary-color;

        // row col row col
        .doc-nav { grid-area: 1 / 1 / 1 / 1; }
        .temp-nav { grid-area: 1 / 2 / 1 / 2; }
        .config-nav { grid-area: 1 / 4 / 1 / 4; }

    }

    .center {
        display: flex;
        justify-content: center;
        align-items: center;
        border: 2px solid yellow;
    }

</style>
