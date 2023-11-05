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
    <select id="claimGender" bind:value={asmtData.claimant.gender} required>
    {#each genders as gender}
        <option value={gender}>{gender}</option>
    {/each}
    </select>
    <br>

    <label for="claimDOB">claimant DOB:</label>
    <input type="text" id="claimDOB" bind:value={asmtData.claimant.dateOfBirth} required>
    <br>

    <label for="claimAge">Claimaint Age:</label>
    <input class="textbox" id="claimAge" type="text" bind:value={asmtData.claimant.age} required/>
    <br>

    <label for="claimDOL">claimant DOL:</label>
    <input type="text" id="claimDOL" bind:value={asmtData.claimant.dateOfLoss} required>
    <br>

    <label for="claimAddress">Claimaint Address:</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmtData.claimant.addressLong} required/>
    <br>

    <label for="adjuster">Adjuster:</label>
    <input class="textbox" id="adjuster" type="text" bind:value={asmtData.adjuster}/>
    <br>

    <label for="insuranceComp">Insurance Company:</label>
    <input class="textbox" id="insuranceComp" type="text" bind:value={asmtData.insCompany} required/>
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
    <select id="type" bind:value={asmtData.asmtType} required>
    {#each asmTypes as type}
        <option value={type}>{type}</option>
    {/each}
    </select>
    <br>

    <button type="submit">Press me to get file</button>

    <p>{status}</p>

    <hr/>

    {#if asmtData.asmtType == "AC.docx"}
        <AC/>
    {/if}

</form>
<script>

    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'
    import AC from './components/ac.svelte'

    onMount(() => {
        invoke('get_assessors').then((assessorOpts) => assessors = assessorOpts);
        assessors = assessors;
        invoke('get_companies').then((comps) => referralCompanies = comps);
        referralCompanies = referralCompanies;
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

    let assessors = new Array();

    let referralCompanies = new Array();

    //TODO: these should be retrieived dynamically.
    let asmTypes = [
        "AC.docx",
        "AC MRB.docx",
        "CAT.docx",
        "CAT AC.docx",
        "CAT AC MRB.docx",
        "CAT CAT GOSE.docx",
        "CAT GOSE.docx",
        "CAT MRB_accidentally AC.docx",
        "MRB.docx",
        "NEB.docx"
    ]

    // TO THIS OBJ, NEED TO ADD:
    //
    // dol, doa, dob, in formattable datetimes
    // client address 
    // client city 
    // client province 
    // client provinceAB
    // client postal code 

    let asmtData = {
        "asmtType": "",
        "assessor": {
            "registationId": "",
            "firstName": "",
            "lastName": "",
        },
        "adjuster": "",
        "insCompany": "",
        "claimNumber": "",
        "referralCompany": {
            "uniqueId": "",
            "commonName": "",
        },
        "dateOfAssessment": "",
        "seidenFileNumber": "",
        "claimant": {
            "salutation": "",
            "firstName": "",
            "lastName": "",

            "gender": "",
            "male-female": "",
            "he-she": "",
            "his-her": "",
            "himself-herself": "",

            "youth": "",
            "dateOfBirth": "",
            "age": "",
            "dateOfLoss": "",
            "addressLong": "",
            "country": "",
            "province": "",
            "street": "",
            "streetNum": "",
            "postalCode": ""
        },
        "questions": {
        }
    }

</script>

