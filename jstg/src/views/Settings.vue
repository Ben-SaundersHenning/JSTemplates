<script lang="ts" setup>

    import { ref, onMounted } from "vue";
    import { useForm } from "vee-validate";
    import { toTypedSchema } from "@vee-validate/zod";
    import { z } from "zod";
    import { invoke } from "@tauri-apps/api/core";

    const { errors, handleSubmit, setFieldError, defineField } = useForm({
        validationSchema: toTypedSchema(z.object({
            savePath: z.string().trim().min(1),
        }))
    });

    const [savePath, savePathAtrb] = defineField("savePath");

    const onSubmit = handleSubmit(onSuccess, onInvalidSubmit);

    let config = ref({
        save_dir: ""
    })

    function onSuccess(values) {

        // verify path
        // if there is an error, set:
        setFieldError('savePath', 'Not a valid path');

    }

    function onInvalidSubmit({ values, errors, results }) {
        console.log(errors);
    }

    onMounted(() => {

        invoke('get_config').then((init_config) => config.value = init_config as Object)
        .catch((e) => console.log(e));

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
