<script lang="ts" setup>

</script>

<template>
    <form @submit="onSubmit">

        <div class="inputs">
            <div class="path-input vertical-input">
                <p class="input-label">Document Save Path</p>
                <input aria-label="Document Save Path" id="savepath-input" class="input-border" type="text" name="savepath"/>
                <!-- <input aria-label="Document Save Path" id="savepath-input" class="input-border" type="text" name="savepath" v-model="" :=""/> -->
                <!-- <span class="error">{{errors['claimant.firstName']}}</span> -->
            </div>
        </div>
        <div class="horizontal-input" style="justify-content: center; margin-top: 30px;">
            <button class="submit" type="submit">Save</button>
        </div>

    </form>
</template>

<style lang="scss" scoped>

    @use '../variables';

    .inputs {
        margin: 30px;
    }

</style>
