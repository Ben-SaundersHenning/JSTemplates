<form on:submit={submitPost}>

    <label for="assessor">Assessor:</label>
    <select id="assessor" bind:value={asmtData.assessor} required>
    {#each assessors as assessor}
        <option value={assessor}>{assessor.firstName} {assessor.lastName}</option>
    {/each}
    </select>
    <br>

    <label for="claimFirst">Claimaint First Name:</label>
    <input class="textbox" id="claimFirst" type="text" bind:value={asmtData.claimant.firstName} required/>
    <br>

    <label for="claimLast">Claimaint Last Name:</label>
    <input class="textbox" id="claimLast" type="text" bind:value={asmtData.claimant.lastName} required/>
    <br>

    <label for="claimGender">Claimaint Gender:</label>
    <select id="claimGender" bind:value={asmtData.claimant.gender.pronouns.p0Lower} required>
    {#each genders as gender}
        <option value={gender}>{gender}</option>
    {/each}
    </select>
    <br>

    <label for="claimDOB">claimant DOB:</label>
    <input type="text" id="claimDOB" bind:value={asmtData.claimant.dateOfBirth} required/>
    <br>

    <label for="claimDOL">claimant DOL:</label>
    <input type="text" id="claimDOL" bind:value={asmtData.claimant.dateOfLoss} required/>
    <br>

    <label for="claimAddress">Claimaint Address (street):</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmtData.claimant.address.address} required/>
    <br>

    <label for="claimCity">Claimaint City:</label>
    <input class="textbox" id="claimCity" type="text" bind:value={asmtData.claimant.address.city} required/>
    <br>

    <label for="claimProvince">Claimaint Province:</label>
    <input class="textbox" id="claimProvince" type="text" bind:value={asmtData.claimant.address.provinceAb} required/>
    <br>

    <label for="claimPostalCode">Claimaint Postal Code:</label>
    <input class="textbox" id="claimPostalCode" type="text" bind:value={asmtData.claimant.address.postalCode} required/>
    <br>

    <label for="adjuster">Adjuster:</label>
    <input class="textbox" id="adjuster" type="text" bind:value={asmtData.adjuster}/>
    <br>

    <label for="insuranceComp">Insurance Company:</label>
    <input class="textbox" id="insuranceComp" type="text" bind:value={asmtData.insuranceCompany} required/>
    <br>

    <label for="claimNO">Claim Number:</label>
    <input class="textbox" id="claimNO" type="text" bind:value={asmtData.claimNumber} required/>
    <br>

    <label for="doa">Date of assessment:</label>
    <input class="textbox" id="doa" type="text" bind:value={asmtData.dateOfAssessment} required/>
    <br>

    <label for="refComp">Referral Company:</label>
    <select id="refComp" bind:value={asmtData.referralCompany} required>
    {#each referralCompanies as company}
        <option value={company}>{company.commonName}</option>
    {/each}
    </select>
    <br>

    <label for="type">Assessment Type:</label>
    <select id="type" bind:value={asmtData.asmtType} required multiple>
    {#each asmtTypes as type}
        <option value={type}>{type.commonName}</option>
    {/each}
    </select>
    <br>

    <button type="submit">Press me to get file</button>

    <p>{status}</p>


    <br>

    <hr/>

    {#if asmtData.asmtType == "AC.docx"}
        <AC bind:acData={asmtData.asmtSpecifics.ac}/>
    {/if}

</form>

<script lang="ts">

    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'
    import AC from './components/ac.svelte'

    onMount(() => {
        invoke('get_assessors').then((assessorOpts) => assessors = assessorOpts as any[]);
        assessors = assessors;
        invoke('get_companies').then((comps) => referralCompanies = comps as any[]);
        referralCompanies = referralCompanies;
        invoke('get_assessment_types').then((types) => asmtTypes = types as any[]);
        asmtTypes = asmtTypes;
    });

    async function submitPost() {
        try {

            const send = JSON.stringify(asmtData);

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

    let referralCompanies: any[] = new Array();

    let asmtTypes: any[] = new Array();

    let asmtData = {
        "asmtType": "",
        "assessor": {
            "registationId": "",
            "title": "",
            "firstName": "",
            "lastName": "",
            "email": "",
            "qualificationsParagraph": ""
        },
        "adjuster": "",
        "insuranceCompany": "",
        "claimNumber": "",
        "referralCompany": {
            "uniqueId": "",
            "commonName": "",
            "name": "",
            "address": {
                "address": "",
                "city": "",
                "province": "",
                "provinceAb": "",
                "postalCode": "",
                "country": "",
                "addressLong": "",
            },
            "phone": "",
            "fax": "",
            "email": ""
        },
        "dateOfAssessment": "",
        "seidenFileNumber": "",
        "claimant": {
            "firstName": "",
            "lastName": "",
            "gender": {
                "title": "",
                "pronouns": {
                    "p0Lower": "",
                    "p1Lower": "",
                    "p2Lower": "",
                    "p3Lower": "",
                    "p0Upper": "",
                    "p1Upper": "",
                    "p2Upper": "",
                    "p3Upper": "",
                }
            },
            "dateOfBirth": "",
            "age": "",
            "youth": "false",
            "dateOfLoss": "",
            "address": {
                "address": "",
                "city": "",
                "province": "",
                "provinceAb": "ON",
                "postalCode": "",
                "country": "Canada",
                "addressLong": "",
            },
        },
        "asmtSpecifics": <any>{
        },
        "questions": [
        ]
    }

</script>

