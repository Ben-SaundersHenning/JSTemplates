<script setup lang="ts">

    import TopBar from "./components/TopBar.vue"

</script>

<template>
  <div class="container">
    <TopBar/>
    <router-view style="width: 100%"/>
  </div>
</template>
