<form on:submit={submitPost}>

    <label for="assessor">Assessor:</label>
    <select id="assessor" bind:value={asmt_data.assessor} required>
    {#each assessors as assessor}
        <option value={assessor}>{assessor.first_name} {assessor.last_name}</option>
    {/each}
    </select>
    <br>

    <label for="claimFirst">Claimaint First Name:</label>
    <input class="textbox" id="claimFirst" type="text" bind:value={asmt_data.claimant.first_name} required/>
    <br>

    <label for="claimLast">Claimaint Last Name:</label>
    <input class="textbox" id="claimLast" type="text" bind:value={asmt_data.claimant.last_name} required/>
    <br>

    <label for="claimGender">Claimaint Gender:</label>
    <select id="claimGender" bind:value={asmt_data.claimant.gender.pronouns.p0_lower} required>
    {#each genders as gender}
        <option value={gender}>{gender}</option>
    {/each}
    </select>
    <br>

    <label for="claimDOB">claimant DOB:</label>
    <input type="text" id="claimDOB" bind:value={asmt_data.claimant.date_of_birth} required/>
    <br>

    <label for="claimDOL">claimant DOL:</label>
    <input type="text" id="claimDOL" bind:value={asmt_data.claimant.date_of_loss} required/>
    <br>

    <label for="claimAddress">Claimaint Address (street):</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmt_data.claimant.address.address} required/>
    <br>

    <label for="claimCity">Claimaint City:</label>
    <input class="textbox" id="claimCity" type="text" bind:value={asmt_data.claimant.address.city} required/>
    <br>

    <label for="claimProvince">Claimaint Province:</label>
    <input class="textbox" id="claimProvince" type="text" bind:value={asmt_data.claimant.address.province_abbreviated} required/>
    <br>

    <label for="claimPostalCode">Claimaint Postal Code:</label>
    <input class="textbox" id="claimPostalCode" type="text" bind:value={asmt_data.claimant.address.postal_code} required/>
    <br>

    <label for="adjuster">Adjuster:</label>
    <input class="textbox" id="adjuster" type="text" bind:value={asmt_data.adjuster}/>
    <br>

    <label for="insuranceComp">Insurance Company:</label>
    <input class="textbox" id="insuranceComp" type="text" bind:value={asmt_data.insurance_company} required/>
    <br>

    <label for="claimNO">Claim Number:</label>
    <input class="textbox" id="claimNO" type="text" bind:value={asmt_data.claim_number} required/>
    <br>

    <label for="doa">Date of assessment:</label>
    <input class="textbox" id="doa" type="text" bind:value={asmt_data.date_of_assessment} required/>
    <br>

    <label for="refComp">Referral Company:</label>
    <select id="refComp" bind:value={asmt_data.referral_company} required>
    {#each referral_companies as company}
        <option value={company}>{company.common_name}</option>
    {/each}
    </select>
    <br>

    <label for="type">Assessment Type:</label>
    <select id="type" bind:value={asmt_data.asmt_type} required>
    {#each asmt_types as type}
        <option value={type.file}>{type.common_name}</option>
    {/each}
    </select>
    <br>

    <button type="submit">Press me to get file</button>

    <p>{status}</p>


    <br>

    <hr/>

    <button on:click={printRequest}>Print request</button>

    {#if asmt_data.asmt_type.includes("AC")}
        <AC bind:acData={asmt_data.asmt_specifics.ac}/>
    {/if}

    {#if asmt_data.asmt_type.includes("CAT")}
        <CAT bind:catData={asmt_data.asmt_specifics.cat}/>
    {/if}

    {#if asmt_data.asmt_type.includes("MRB")}
        <MRB bind:mrbData={asmt_data.asmt_specifics.mrb}/>
    {/if}

    {#if asmt_data.asmt_type.includes("NEB")}
        <NEB bind:nebData={asmt_data.asmt_specifics.neb}/>
    {/if}

</form>

<script lang="ts">

    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'
    import AC from './components/ac.svelte'
    import CAT from './components/cat.svelte'
    import MRB from './components/mrb.svelte'
    import NEB from './components/neb.svelte'

    onMount(() => {
        invoke('get_assessors').then((assessor_opts) => assessors = assessor_opts as any[]);
        assessors = assessors;
        invoke('get_referral_company_options').then((comps) => referral_companies = comps as any[]);
        referral_companies = referral_companies;
        invoke('get_document_options').then((types) => asmt_types = types as any[])
        .catch((e) => status = e);
        asmt_types = asmt_types;
    });

    async function printRequest() {
        try {

            const send = JSON.stringify(asmt_data);

            invoke('print_request', {data: send});
            status="Saved."; //only works for 1 document

        } catch (exceptionVar){
            status="Error!";
        }
    }

    async function submitPost() {
        try {

            const send = JSON.stringify(asmt_data);

            invoke('request_document', {data: send});
            status="Saved."; //only works for 1 document

        } catch (exceptionVar){
            status="Error!";
        }
    }

    let status = "Not sent"

    let genders = [
        "male",
        "female",
        "other"
    ]

    let assessors: any[] = new Array();

    let referral_companies: any[] = new Array();

    let asmt_types: any[] = new Array();

    let asmt_data = {
        asmt_type: "",
        types: [],
        assessor: {
            registration_id: "",
            title: "",
            first_name: "",
            last_name: "",
            email: "",
            qualifications_paragraph: ""
        },
        adjuster: "",
        insurance_company: "",
        claim_number: "",
        referral_company: {
            id: "",
            common_name: "",
            name: "",
            address: {
                address: "",
                city: "",
                province: "",
                province_abbreviated: "",
                postal_code: "",
                country: "",
                address_long: "",
            },
            phone: "",
            fax: "",
            email: ""
        },
        date_of_assessment: "",
        seiden_file_number: "",
        claimant: {
            first_name: "",
            last_name: "",
            gender: {
                title: "",
                pronouns: {
                    p0_lower: "",
                    p1_lower: "",
                    p2_lower: "",
                    p3_lower: "",
                    p0_upper: "",
                    p1_upper: "",
                    p2_upper: "",
                    p3_upper: "",
                }
            },
            date_of_birth: "",
            age: "",
            youth: "false",
            date_of_loss: "",
            address: {
                address: "",
                city: "",
                province: "",
                province_abbreviated: "ON",
                postal_code: "",
                country: "Canada",
                address_long: "",
            },
        },
        asmt_specifics: <any>{
        },
        questions: [
        ]
    }

</script>

