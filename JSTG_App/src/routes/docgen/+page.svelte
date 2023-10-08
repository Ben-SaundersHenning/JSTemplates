<div>
    <label for="type">Assessment Type:</label>
    <select id="type" bind:value={asmtData.asmtType}>
    {#each asmTypes as type}
        <option value={type}>{type}</option>
    {/each}
    </select>
    <br>

    <label for="therapist">Therapist:</label>
    <select id="therapist" bind:value={asmtData.therapist}>
    {#each therapists as therapist}
        <option value={therapist}>{therapist.salutation}. {therapist.firstName} {therapist.lastName}</option>
    {/each}
    </select>
    <br>

    <label for="claimFirst">Claimaint First Name:</label>
    <input class="textbox" id="claimFirst" type="text" bind:value={asmtData.claimant.firstName}/>
    <br>

    <label for="claimLast">Claimaint Last Name:</label>
    <input class="textbox" id="claimLast" type="text" bind:value={asmtData.claimant.lastName}/>
    <br>

    <label for="claimGender">Claimaint Gender:</label>
    <select id="claimGender" bind:value={asmtData.claimant.gender}>
    {#each genders as gender}
        <option value={gender}>{gender}</option>
    {/each}
    </select>
    <br>

    <label for="claimDOB">claimant DOB:</label>
    <input type="text" id="claimDOB" bind:value={asmtData.claimant.dateOfBirth}>
    <br>

    <label for="claimAge">Claimaint Age:</label>
    <input class="textbox" id="claimAge" type="text" bind:value={asmtData.claimant.age}/>
    <br>

    <label for="claimDOL">claimant DOL:</label>
    <input type="text" id="claimDOL" bind:value={asmtData.claimant.dateOfLoss}>
    <br>

    <label for="claimAddress">Claimaint Address:</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmtData.claimant.addressLong}/>
    <br>

    <label for="adjuster">Adjuster:</label>
    <input class="textbox" id="adjuster" type="text" bind:value={asmtData.adjuster}/>
    <br>

    <label for="insuranceComp">Insurance Company:</label>
    <input class="textbox" id="insuranceComp" type="text" bind:value={asmtData.insCompany}/>
    <br>

    <label for="claimNO">Claim Number:</label>
    <input class="textbox" id="claimNO" type="text" bind:value={asmtData.claimNumber}/>
    <br>

    <label for="doa">Date of assessment:</label>
    <input class="textbox" id="doa" type="text" bind:value={asmtData.dateOfAssessment}/>
    <br>

    <label for="refComp">Referral Company:</label>
    <input class="textbox" id="refComp" type="text" bind:value={asmtData.referralCompany}/>
    <br>

    <button on:click={submitPost}>Press me to get file</button>

    <p>{status}</p>

    <button on:click={testRequestBuilder}>Press me to test the request builder</button>

    <Test/>

</div>
<script>

    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'
    import Test from './components/test.svelte'

    onMount(() => {
        invoke('get_assessors').then((assessors) => therapists = assessors);
        therapists = therapists;
    });

    function testRequestBuilder() {
        const send = JSON.stringify(asmtData);
        invoke('test', {data: send});
    }

    async function submitPost() {
        try {

            map["ADJUSTER"] = asmtData.adjuster;
            map["INSURANCE COMPANY"] = asmtData.insCompany;
            map["CLIENT FIRST"] = asmtData.claimant.firstName;
            map["CLIENT LAST"] = asmtData.claimant.lastName;
            map["DOB"] = asmtData.claimant.dateOfBirth;
            map["CLAIM NUMBER"] = asmtData.claimNumber;
            map["DOL"] = asmtData.claimant.dateOfLoss;
            map["DOA"] = asmtData.dateOfAssessment;
            map["CLIENT AGE"] = asmtData.claimant.age;
            map["REFERRAL COMPANY"] = asmtData.referralCompany;
            map["CLIENT ADDRESS"] = asmtData.claimant.addressLong;
            map["TEMPLATE"] = asmtData.asmtType;
            map["OCCUPATIONAL THERAPIST"] = asmtData.therapist.salutation + ". " + asmtData.therapist.firstName + " " + asmtData.therapist.lastName;

            if(asmtData.claimant.gender == "male") {
                map["HE---SHE_Lower"] = "he";
                map["MALE---FEMALE_Lower"] = "male";
                map["MALE---FEMALE_LOWER"] = "male";
                map["HIS---HER_Lower"] = "his";
                map["HE---SHE_Upper"] = "He";
                map["HIM---HER_Lower"] = "him";
                map["CLIENT SALUTATION"] = "Mr";
            } else {
                map["HE---SHE_Lower"] = "she";
                map["MALE---FEMALE_Lower"] = "female";
                map["MALE---FEMALE_LOWER"] = "female";
                map["HIS---HER_Lower"] = "her";
                map["HE---SHE_Upper"] = "She";
                map["HIM---HER_Lower"] = "her";
                map["CLIENT SALUTATION"] = "Ms";
            }

            switch(asmtData.therapist.firstName) {

                case "Joan":
                map["IMAGE"] = "JS.png";
                break;

                case "Montana":
                map["IMAGE"] = "MM.png";
                break;

                case "Anghela":
                map["IMAGE"] = "AS.png";
                break;

                case "Josh":
                map["IMAGE"] = "JM.png";
                break;

                default:
                map["IMAGE"] = "JS.png";

            }

            const _ = JSON.stringify(map);

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

    let therapists = new Array();

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

    let map = {
        "ADJUSTER": "",
        "INSURANCE COMPANY": "",
        "CLIENT FIRST": "",
        "CLIENT LAST": "",
        "DOB": "",
        "CLAIM NUMBER": "",
        "DOL": "",
        "DOA": "",
        "CLIENT AGE": "",
        "REFERRAL COMPANY": "",
        "CLIENT ADDRESS": "",
        "TEMPLATE": "",
        "OCCUPATIONAL THERAPIST": "",
        "HE---SHE_Lower": "",
        "MALE---FEMALE_Lower": "",
        "HIS---HER_Lower": "",
        "MALE---FEMALE_LOWER": "",
        "HE---SHE_Upper": "",
        "HIM---HER_Lower": "",
        "CLIENT SALUTATION": "",
        "TEMPLATE PATH": "",
        "IMAGE PATH": "",
        "IMAGE": ""
    };

    let asmtData = {
        "asmtType": "",
        "therapist": {
            "salutation": "",
            "firstName": "",
            "lastName": "",
            "qualifications": "",
        },
        "adjuster": "",
        "insCompany": "",
        "claimNumber": "",
        "referralCompany": "",
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

