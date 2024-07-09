<script lang="ts" setup>

    import { ref, onMounted } from "vue"

    import {invoke} from "@tauri-apps/api/tauri"

    const picked = ref("One")

    let assessors = ref([
    ])

    let referral_companies = ref([
    ])

    let types = ref([
    ])

    let settings = ref({
        save_dir: ""
    })

    function updateSettings() {

        const send = JSON.stringify(settings.value);
        console.log(send)
        invoke('update_settings', { newSettings: send });

    }

    onMounted(() => {

        invoke('get_settings').then((init_settings) => settings.value = init_settings as Object)
        .catch((e) => console.log(e));

        invoke('get_assessor_options').then((assessor_options) => {
            console.log(assessor_options.listing_details);
            assessors.value = assessor_options.listing_details as Array;
        })
        .catch((e) => console.log(e));

        invoke('get_document_options').then((document_options) => {
            console.log(document_options.listing_details);
            types.value = document_options.listing_details as Array;
        })
        .catch((e) => console.log(e));

        invoke('get_referral_company_options').then((rc_options) => referral_companies.value = rc_options as Array)
        .catch((e) => console.log(e));

    })

</script>

<template>
    <main class="home-page">
        <form>

            <fieldset>
                <legend>ASSESSMENT</legend>
                    <div class="assessment-items">

                        <div class="div1 horizontal-input">
                            <h3>Assessor:</h3>
                            <span v-for="(assessor, index) in assessors">
                                <input type="radio" :id="assessor.id" :value="assessor.id" v-model="picked" />
                                <label :for="assessor.id">{{assessor.name}}</label>
                            </span>
                        </div>

                        <div class="div2 horizontal-input">
                            <h3>Type:</h3>
                            <div class="checkboxes">
                                <span v-for="(type, index) in types">
                                    <input type="checkbox" :id="type.id" :value="type.document" v-model="checkedNames">
                                    <label :for="type.id">{{type.document}}</label>
                                </span>
                            </div>
                        </div>

                        <div class="div3">
                            <select name="test" id="test" size="5">
                              <option>comp1</option>
                              <option>comp2</option>
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

    .horizontal-input {

        display: flex;
        flex-direction: row;
        column-gap: 1rem;
        align-items: center;
        flex-wrap: wrap;

    }

    .assessment-items {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        grid-template-rows: repeat(3, 1fr);
        grid-column-gap: 0px;
        grid-row-gap: 5px;

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
      grid-column-gap: 1rem;
      grid-row-gap: 0.5rem;
      grid-template-columns: 1fr 1fr 1fr;
    }

//    select {
//        text-align-last: center;
//        text-align: center;
//        width: 100%;
//        font-size: 28px;
//    }

    fieldset {
        border: 5px solid black;
        legend {
            float: left;
            border-bottom: 2px solid blue;
        }
    }

    .text-input {

        display: flex;
        flex-direction: column;
        row-gap: 5px;

        input {
            width: 30%;
        }

    }

</style>
