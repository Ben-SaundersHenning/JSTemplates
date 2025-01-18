<script lang="ts" setup>

    import { onMounted } from "vue";
    import { useForm } from "vee-validate";
    import { toTypedSchema } from "@vee-validate/zod";
    import { z } from "zod";

    const { errors, handleSubmit, defineField } = useForm({
        validationSchema: toTypedSchema(z.object({
            savePath: z.string().trim().min(1), //TODO validate directory
        }))
    });

    const [savePath, savePathAtrb] = defineField("savePath");

    const onSubmit = handleSubmit(onSuccess, onInvalidSubmit);

    function onSuccess(values) {

    }

    function onInvalidSubmit({ values, errors, results }) {
        console.log(errors);
    }

    onMounted(() => {

        //get config
        //apply config to fields

    })

</script>

<template>
    <form @submit="onSubmit">

        <div class="inputs">
            <div class="path-input vertical-input">
                <p class="input-label">Document Save Path</p>
                <input aria-label="Document Save Path" id="savepath-input" class="input-border" type="text" name="savepath" v-model="savePath" :="savePathAtrb"/>
                <span class="error">{{errors['savePath']}}</span>
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
