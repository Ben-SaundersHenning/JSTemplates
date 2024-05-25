<script lang="ts" setup>

    import { ref, onMounted } from "vue"

    import {invoke} from "@tauri-apps/api/tauri"

    const picked = ref("One")

    const assessors = ref([
        {
            name: "One",
            id: "One"
        },
        {
            name: "Two",
            id: "Two"
        },
        {
            name: "Three",
            id: "Three"
        },
        {
            name: "Four",
            id: "Four"
        },
        {
            name: "Five",
            id: "Five"
        },
    ])

    const types = ref([
        {
            name: "T1",
            id: "T1"
        },
        {
            name: "T2",
            id: "T2"
        },
        {
            name: "T3",
            id: "T3"
        },
    ])

    let settings = ref({
        save_dir: ""
    })

    function updateSettings() {

        const send = JSON.stringify(settings.value);
        console.log(send)
        invoke('update_settings', { newSettings: send });

    }

    // retrieves the selected settings.
    onMounted(() => {
        invoke('get_settings').then((init_settings) => settings.value = init_settings as Object)
        .catch((e) => status = e);
    })

</script>

<template>
    <main class="home-page">
        <h1>Home</h1>

        <!-- temp form for settings, keeps save directory visible -->
        <form>
            <label for="settings">Settings:</label>
            <input type="text" id="settings" name="settings" v-model="settings.save_dir"><br><br>
            <button @click.prevent="updateSettings">Submit settings</button>
        </form>

        <form>
            <fieldset>
                <legend>ASSESSMENT</legend>
                    <div class="assessment-items">

                        <div class="div1">
                            <span v-for="(assessor, index) in assessors">
                                <input type="radio" :id="assessor.id" :value="assessor.id" v-model="picked" />
                                <label :for="assessor.id">{{assessor.name}}</label>
                            </span>
                        </div>

                        <div class="div2">
                            <div class="checkboxes">
                                <span v-for="(type, index) in types">
                                    <input type="checkbox" :id="type.id" :value="type.name" v-model="checkedNames">
                                    <label :for="type.id">{{type.name}}</label>
                                </span>
                            </div>
                        </div>

                        <div class="div3">
                            <select name="test" id="test" size="5">
                              <option>text1</option>
                              <option>text2</option>
                              <option>text3</option>
                              <option>text4</option>
                              <option>text5</option>
                              <option>text6</option>
                              <option>text7</option>
                              <option>text8</option>
                              <option>text9</option>
                              <option>text10</option>
                            </select>
                        </div>

                        <div class="div4">Claim</div>

                        <div class="div5 text-input">
                            <label for="date-input">Date of Assessment</label>
                            <input id="date-input" type="text" name="doa" />
                        </div>

                    </div>
            </fieldset>
            <fieldset>
                <legend>CLIENT</legend>
                    <label for="lname">Last name:</label>
                    <input type="text" id="lname" name="lname"><br><br>
            </fieldset>
            <fieldset>
                <legend>INSURANCE</legend>
                    <label for="rname">rast name:</label>
                    <input type="text" id="rname" name="rname"><br><br>
            </fieldset>
        </form>
    </main>
</template>

<style lang="scss" scoped>

    .assessment-items {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        grid-template-rows: repeat(3, 1fr);
        grid-column-gap: 0px;
        grid-row-gap: 5px;

        div {
            border: 2px solid black;
        }

        .div1 { grid-area: 1 / 1 / 2 / 3; display: flex; flex-direction: row; column-gap: 2rem;}
        .div2 { grid-area: 2 / 1 / 3 / 2; }
        .div3 { grid-area: 2 / 2 / 3 / 3; }
        .div4 { grid-area: 3 / 1 / 4 / 2; }
        .div5 { grid-area: 3 / 2 / 4 / 3; }
    }

    .assessment-items {
        display: grid;
        height: 100%;
        margin: 0;
    }

    .checkboxes {
      //width: 60px;
      display: grid;
      grid-gap: 10px;
      grid-template-columns: 1fr 1fr 1fr;
    }

//    select {
//        text-align-last: center;
//        text-align: center;
//        width: 100%;
//        font-size: 28px;
//    }

    .text-input {

        display: flex;
        flex-direction: column;
        row-gap: 5px;

        input {
            width: 30%;
        }

    }

</style>
