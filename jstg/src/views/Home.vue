<script lang="ts" setup>

    import { ref, onMounted, watch, computed } from "vue";

    import { invoke } from "@tauri-apps/api/core";

    import { useForm } from "vee-validate";

    import { toTypedSchema } from "@vee-validate/zod";

    import { z } from "zod";

    import dayjs from 'dayjs';

    const { errors, handleSubmit, defineField } = useForm({
        validationSchema: toTypedSchema(
            z.object({
                assessorRegistrationId: z.string().regex(/^G[0-9]{7}$/),
                adjuster: z.string().optional(),
                insuranceCompany: z.string().trim().min(1),
                claimNumber: z.string().trim().min(1),
                referralCompanyId: z.number(),
                dateOfAssessment: z.string().date(),
                claimant: z.object({
                    firstName: z.string().trim().min(1),
                    lastName: z.string().trim().min(1),
                    gender: z.enum(["Male", "Female", "Other"]),
                    dateOfBirth: z.string().date(),
                    dateOfLoss: z.string().date(),
                    address: z.object({
                        streetAddress: z.string().trim().min(1),
                        unit: z.string().trim().min(1).optional(),
                        city: z.string().trim().min(1),
                        province: z.string().trim().min(1),
                        postalCode: z.string().trim().min(1),
                        country: z.string().trim().min(1),
                    }),
                }),
                assessmentTypes: z.string(),
            }),
        ),
    });

    // NAMING SHORTCUTS
    // assessor => asr
    // referralCompany => rc
    // claimant => cl
    // address => add
    // dateOf => do
    const [asrRegistrationId, asrRegistrationIdAtrb] = defineField("assessorRegistrationId");
    const [adjuster, adjusterAtrb] = defineField("adjuster");
    const [insuranceCompany, insuranceCompanyAtrb] = defineField("insuranceCompany");
    const [claimNumber, claimNumberAtrb] = defineField("claimNumber");
    const [rcId, rcIdAtrb] = defineField("referralCompanyId");
    const [doAssessment, doAssessmentAtrb] = defineField("dateOfAssessment");
    const [clFirstName, clFirstNameAtrb] = defineField("claimant.firstName");
    const [clLastName, clLastNameAtrb] = defineField("claimant.lastName");
    const [clGender, clGenderAtrb] = defineField("claimant.gender");
    const [clDoBirth, clDoBirthAtrb] = defineField("claimant.dateOfBirth");
    const [clDoLoss, clDoLossAtrb] = defineField("claimant.dateOfLoss");
    const [clAddStreetAddress, clAddStreetAddressAtrb] = defineField("claimant.address.streetAddress");
    const [clAddUnit, clAddUnitAtrb] = defineField("claimant.address.unit");
    const [clAddCity, clAddCityAtrb] = defineField("claimant.address.city");
    const [clAddProvince, clAddProvinceAtrb] = defineField("claimant.address.province");
    const [clAddPostalCode, clAddPostalCodeAtrb] = defineField("claimant.address.postalCode");
    const [clAddCountry, clAddCountryAtrb] = defineField("claimant.address.country");
    const [assessmentTypes, assessmentTypesAtrb] = defineField("assessmentTypes");

    // OLD OBJECT --
    // const asmtData = reactive({
    //         assessor: {
    //             registrationId: "",
    //         },
    //         adjuster: "",
    //         insuranceCompany: "",
    //         claimNumber: "",
    //         referralCompany: {
    //             id: "",
    //         },
    //         dateOfAssessment: "",
    //         claimant: {
    //             firstName: ref("Ben"),
    //             lastName: "",
    //             gender: "",
    //             dateOfBirth: "",
    //             dateOfLoss: "",
    //             address: {
    //                 address: "",
    //                 city: "",
    //                 province: "",
    //                 postalCode: "",
    //                 country: "",
    //             },
    //         },
    //         // asmtTypes: <{}>[], // types required in document, plus their required info.
    //         // questions: [
    //         // ]
    //     });

    // Example return formats:
    // July 23, 2024
    // December 25, 2019
    function formatDate(date) {
        const parsedDate = z.string().date().safeParse(date);
        if (parsedDate.success === true) {
            return dayjs(date).format('MMMM D, YYYY');
        } else {
            return "";
        }
    }

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

    const onSubmit = handleSubmit(values => {
        // console.log(JSON.stringify(values));
        invoke('request_document', { data: JSON.stringify(values) });
    });

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

        invoke('get_referral_company_options').then((rc_options) => referral_companies.value = rc_options.listing_details as Array)
        .catch((e) => console.log(e));

    })

</script>

<template>
    <form @submit="onSubmit">

        <fieldset>

            <legend>Assessment Information</legend>
            <div class="assessment-inputs">
                <div class="assessor-input vertical-input">
                    <p class="input-label">Assessor</p>
                    <div class="horizontal-input input-border">
                        <span v-for="(assessor, index) in assessors">
                            <input type="radio" name="assessor" :id="'assessor' + assessor.id" :value="assessor.id"
                                   v-model="asrRegistrationId" :="asrRegistrationIdAtrb"/>
                            <label :for="'assessor' + assessor.id">{{assessor.name}}</label>
                        </span>
                    </div>
                    <span class="error">{{errors['assessorRegistrationId']}}</span>
                </div>
                <div class="assessment-type-input vertical-input">
                    <p class="input-label">Type</p>
                    <div class="checkboxes input-border">
                        <span v-for="(type, index) in types">
                            <!-- TODO: ADD SCHEMA FOR THIS INPUT -->
                            <input type="radio" name="assessment-type" :id="'assessment-type' + type.id" :value="type.document" 
                                   v-model="assessmentTypes" :="assessmentTypesAtrb">
                            <label :for="'assessment-type' + type.id">{{type.document}}</label>
                        </span>
                    </div>
                    <span class="error">{{errors['assessmentTypes']}}</span>
                </div>
                <div class="company-input vertical-input">

                    <p class="input-label">Referral Company</p>
                    <div class="checkboxes company input-border">
                        <span v-for="(company, index) in referral_companies">
                            <input type="radio" name="company" :id="'company' + company.id" :value="company.id"
                                   v-model="rcId" :="rcIdAtrb"/>
                            <label :for="'company' + company.id">{{company.name}}</label>
                        </span>
                    </div>
                    <span class="error">{{errors['referralCompanyId']}}</span>
                </div>
                <div class="date-of-assessment-input vertical-input">
                    <p class="input-label">Date of Assessment</p>
                    <div class="date-input">
                        <input aria-label="Date of Assessment" id="doa-input"  type="text" name="doa"
                                       v-model="doAssessment" :="doAssessmentAtrb"/>
                        <span @onclick="document.getElementById('doa-input').focus(); document.getElementById('doa-input').select();">{{formatDate(doAssessment)}}</span>
                    </div>
                    <span class="error">{{errors['dateOfAssessment']}}</span>
                </div>
            </div>

        </fieldset>

        <fieldset>
            <legend>Client Information</legend>
            <div class="client-inputs">
                <div class="firstname-input vertical-input">
                    <p class="input-label">First Name</p>
                    <input aria-label="First Name" id="fname-input" class="input-border" type="text" name="fname" v-model="clFirstName" :="clFirstNameAtrb"/>
                    <span class="error">{{errors['claimant.firstName']}}</span>
                </div>

                <div class="lastname-input vertical-input">
                    <p class="input-label">Last Name</p>
                    <input aria-label="Last Name" id="lname-input" class="input-border" type="text" name="lname"
                                   v-model="clLastName" :="clLastNameAtrb"/>
                    <span class="error">{{errors['claimant.lastName']}}</span>
                </div>

                <div class="gender-input vertical-input">
                    <p class="input-label">Gender</p>
                    <div class="horizontal-input input-border">
                        <input type="radio" id="male" name="gender" value="Male"
                                   v-model="clGender" :="clGenderAtrb"/>
                        <label for="male">Male</label><br>
                        <input type="radio" id="female" name="gender" value="Female"
                                   v-model="clGender" :="clGenderAtrb"/>
                        <label for="female">Female</label><br>
                        <input type="radio" id="other" name="gender" value="Other"
                                   v-model="clGender" :="clGenderAtrb"/>
                        <label for="other">Other</label><br>
                    </div>
                    <span class="error">{{errors['claimant.gender']}}</span>
                </div>

                <div class="dob-input vertical-input">
                    <p class="input-label">Date of Birth</p>
                    <div class="date-input">
                        <input aria-label="Date of Birth" id="dob-input" class="input-border" type="text" name="dob"
                                       v-model="clDoBirth" :="clDoBirthAtrb"/>
                        <span>{{formatDate(clDoBirth)}}</span>
                    </div>
                    <span class="error">{{errors['claimant.dateOfBirth']}}</span>
                </div>

                <div class="dol-input vertical-input">
                    <p class="input-label">Date of Loss</p>
                    <div class="date-input">
                        <input aria-label="Date of Loss" id="dol-input" class="input-border" type="text" name="dol"
                                       v-model="clDoLoss" :="clDoLossAtrb"/>
                        <span>{{formatDate(clDoLoss)}}</span>
                    </div>
                    <span class="error">{{errors['claimant.dateOfLoss']}}</span>
                </div>

                <div class="street-input vertical-input">
                    <p class="input-label">Street Address</p>
                    <input aria-label="Street Address" id="street-address-input" class="input-border" type="text" name="address"
                                   v-model="clAddStreetAddress" :="clAddStreetAddressAtrb"/>
                    <span class="error">{{errors['claimant.address.streetAddress']}}</span>
                </div>

                <div class="apt-input vertical-input">
                    <p class="input-label">Apt, Suite, etc</p>
                    <input aria-label="Apt, Suite, etc" id="unit-input" class="input-border" type="text" name="unit"
                                   v-model="clAddUnit" :="clAddUnitAtrb"/>
                    <span class="error">{{errors['claimant.address.unit']}}</span>
                </div>

                <div class="city-input vertical-input">
                    <p class="input-label">City</p>
                    <input aria-label="City" id="city-input" class="input-border" type="text" name="city"
                                   v-model="clAddCity" :="clAddCityAtrb"/>
                    <span class="error">{{errors['claimant.address.city']}}</span>
                </div>

                <div class="postal-code-input vertical-input">
                    <p class="input-label">Postal Code</p>
                    <input aria-label="Postal Code" id="postal-code-input" class="input-border" type="text" name="postal-code"
                                   v-model="clAddPostalCode" :="clAddPostalCodeAtrb"/>
                    <span class="error">{{errors['claimant.address.postalCode']}}</span>
                </div>

                <div class="province-input vertical-input">
                    <p class="input-label">Province</p>
                    <input aria-label="Province" id="province-input" class="input-border" type="text" name="province"
                                   v-model="clAddProvince" :="clAddProvinceAtrb"/>
                    <span class="error">{{errors['claimant.address.province']}}</span>
                </div>

                <div class="country-input vertical-input">
                    <p class="input-label">Country</p>
                    <input aria-label="Country" id="country-input" class="input-border" type="text" name="country"
                                   v-model="clAddCountry" :="clAddCountryAtrb"/>
                    <span class="error">{{errors['claimant.address.country']}}</span>
                </div>

            </div>
        </fieldset>

        <fieldset>
            <legend>Insurance Information</legend>
            <div class="insurance-inputs">
                <div class="company-input vertical-input">
                    <p class="input-label">Insurance Company</p>
                    <input aria-label="Insurance Company" id="insurance-company-input" class="input-border" type="text" name="insurance-company" 
                           v-model="insuranceCompany" :="insuranceCompanyAtrb"/>
                    <span class="error">{{errors['insuranceCompany']}}</span>
                </div>

                <div class="adjuster-input vertical-input">
                    <p class="input-label">Adjuster</p>
                    <input aria-label="Adjuster" id="adjuster-input" class="input-border" type="text" name="adjuster" v-model="adjuster" :="adjusterAtrb"/>
                    <span class="error">{{errors['adjuster']}}</span>
                </div>

                <div class="claim-number-input vertical-input">
                    <p class="input-label">Claim Number</p>
                    <input aria-label="Claim Number" id="claim-number-input" class="input-border" type="text" name="claim-number"
                                   v-model="claimNumber" :="claimNumberAtrb"/>
                    <span class="error">{{errors['claimNumber']}}</span>
                </div>
            </div>
        </fieldset>

        <div class="horizontal-input" style="justify-content: center; margin-top: 30px;">
            <button class="submit" type="submit">Submit</button>
        </div>

    </form>
</template>

<style lang="scss" scoped>

    @use '../variables';

    ul {

      display: block;
      margin: 0;
      padding: 0;
      background-color: pink;
      position: absolute;
      width: 75px;

        li {
            list-style: none;
            text-align: center;

            &:hover {
                background-color: blue;
                cursor: default;
            }
        }
    }


    .prevent-select {

        -webkit-touch-callout: none; /* iOS Safari */
        -webkit-user-select: none; /* Safari */
        -khtml-user-select: none; /* Konqueror HTML */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* Internet Explorer/Edge */
        user-select: none; /* Non-prefixed version, currently
                                  supported by Chrome and Opera */
    }

    .assessment-inputs {

        display: grid;
        grid-template-columns: repeat(2, auto);
        grid-template-rows: repeat(3, auto);
        grid-column-gap: 50px;
        grid-row-gap: 15px;
        width: fit-content;

        .assessor-input { grid-area: 1 / 1 / 2 / 3; }
        .assessment-type-input { grid-area: 2 / 1 / 3 / 2; }
        .company-input { grid-area: 2 / 2 / 3 / 3; }
        .date-of-assessment-input { grid-area: 3 / 1 / 4 / 2; }

    }

    .client-inputs {

        display: grid;
        grid-template-columns: repeat(3, auto);
        grid-template-rows: repeat(4, auto);
        grid-column-gap: 50px;
        grid-row-gap: 15px;
        width: fit-content;

        .firstname-input { grid-area: 1 / 1 / 2 / 2; }
        .lastname-input { grid-area: 1 / 2 / 2 / 3; }
        .gender-input { grid-area: 1 / 3 / 2 / 4; }

        .dob-input { grid-area: 2 / 1 / 3 / 2; }
        .dol-input { grid-area: 2 / 2 / 3 / 3; }

        .street-input { grid-area: 3 / 1 / 3 / 2; }
        .apt-input { grid-area: 3 / 2 / 3 / 3; }
        .city-input { grid-area: 3 / 3 / 3 / 4; }
        .postal-code-input { grid-area: 4 / 1 / 4 / 2; }
        .province-input { grid-area: 4 / 2 / 4 / 3; }
        .country-input { grid-area: 4 / 3 / 4 / 4; }

    }

    .insurance-inputs {

        display: grid;
        grid-template-columns: repeat(3, auto);
        grid-template-rows: 1fr;
        grid-column-gap: 50px;
        grid-row-gap: 15px;
        width: fit-content;

        .company-input { grid-area: 1 / 1 / 2 / 2; }
        .adjuster-input { grid-area: 1 / 2 / 2 / 3; }
        .claim-number-input { grid-area: 1 / 3 / 2 / 4; }

    }


    .horizontal-input {

        display: flex;
        flex-direction: row;
        column-gap: 1rem;
        align-items: center;
        flex-wrap: wrap;

    }

    .vertical-input {

        display: flex;
        flex-direction: column;
        flex-wrap: scroll;

    }

    .checkboxes {
      display: grid;
      grid-column-gap: 1rem;
      grid-row-gap: 0.5rem;
      grid-template-columns: repeat(4, 1fr);
    }

    .company {
        max-height: 3.5rem;
        overflow: auto;
    }

    .input-label {
        margin: 0px 5px 5px 0px;
    }

    .input-border {

        border: 2px solid variables.$input-border-color;
        border-radius: 4px;
        background-color: variables.$accent-color;
        padding: 0.5rem;
        width: fit-content;

        transition: border 0.2s linear;

        &:focus-within, &:hover {
            border: 2px solid variables.$shadow-color;
        }

    }

    input[type=text] {

        color: variables.$text-color;
        padding: 0.5rem;
        outline: none; 
        border: 2px solid variables.$input-border-color;
        border-radius: 4px;
        width: fit-content;
        transition: border 0.2s linear;

        &:focus {
            border: 2px solid variables.$shadow-color;
        }

    }

    input[type=radio], input[type=checkbox] {

        display: none;
        box-shadow: none;
        /* transition: box-shadow 0.5s linear; */

        &:checked + label {

            box-shadow: 0px 3px 0px 0px variables.$shadow-color;

        }
    }

    .date-input {

        display: flex;
        flex-direction: row;
        column-gap: 5px;
        align-items: center;
        justify-content: start;
        flex-wrap: wrap;

        border: 2px solid variables.$input-border-color;
        border-radius: 4px;
        background-color: variables.$accent-color;
        padding-right: 0.5rem;
        min-width: 25ch;
        width: fit-content;
        height: fit-content;

        transition: border 0.2s linear;

        input[type=text] {

            border: none;
            padding: none;

            color: variables.$text-color;
            outline: none;
            max-width: 10ch;
            height: 1.5rem;
            background-color: variables.$accent-color;

        }

        /* span { */
        /*     width: 40%; */
        /* } */

        &:focus-within, &:hover {
            border: 2px solid variables.$shadow-color;
        }

    }

    .error {
        color: variables.$error-color;
    }

    .submit {


        font-size: 1.15rem;
        color: variables.$text-color;
        border: 2px solid variables.$input-border-color;
        border-radius: 4px;
        background-color: variables.$accent-color;
        padding: 0.5rem;

        transition: border 0.2s linear;

        &:focus-within, &:hover {
            border: 2px solid variables.$shadow-color;
        }

    }

</style>
