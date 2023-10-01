<div>
    <label for="type">Assessment Type:</label>
    <select id="type" bind:value={asmData.type}>
    {#each asmTypes as type}
        <option value={type}>{type}</option>
    {/each}
    </select>
    <br>

    <label for="therapist">Therapist:</label>
    <select id="therapist" bind:value={asmData.therapist}>
    {#each therapists as therapist}
        <option value={therapist}>{therapist.salutation}. {therapist.first_name} {therapist.last_name}</option>
    {/each}
    </select>
    <br>

    <label for="claimFirst">Claimaint First Name:</label>
    <input class="textbox" id="claimFirst" type="text" bind:value={asmData.claimant.firstName}/>
    <br>

    <label for="claimLast">Claimaint Last Name:</label>
    <input class="textbox" id="claimLast" type="text" bind:value={asmData.claimant.lastName}/>
    <br>

    <label for="claimGender">Claimaint Gender:</label>
    <select id="claimGender" bind:value={asmData.claimant.gender}>
    {#each genders as gender}
        <option value={gender}>{gender}</option>
    {/each}
    </select>
    <br>

    <label for="claimDOB">Claimant DOB:</label>
    <input type="text" id="claimDOB" bind:value={asmData.claimant.dateOfBirth}>
    <br>

    <label for="claimAge">Claimaint Age:</label>
    <input class="textbox" id="claimAge" type="text" bind:value={asmData.claimant.age}/>
    <br>

    <label for="claimDOL">Claimant DOL:</label>
    <input type="text" id="claimDOL" bind:value={asmData.claimant.dateOfLoss}>
    <br>

    <label for="claimAddress">Claimaint Address:</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmData.claimant.addressLong}/>
    <br>

    <label for="adjuster">Adjuster:</label>
    <input class="textbox" id="adjuster" type="text" bind:value={asmData.adjuster}/>
    <br>

    <label for="insuranceComp">Insurance Company:</label>
    <input class="textbox" id="insuranceComp" type="text" bind:value={asmData.insCompany}/>
    <br>

    <label for="claimNO">Claim Number:</label>
    <input class="textbox" id="claimNO" type="text" bind:value={asmData.claimNumber}/>
    <br>

    <label for="doa">Date of assessment:</label>
    <input class="textbox" id="doa" type="text" bind:value={asmData.dateOfAssessment}/>
    <br>

    <label for="refComp">Referral Company:</label>
    <input class="textbox" id="refComp" type="text" bind:value={asmData.referralCompany}/>
    <br>

    <button on:click={submitPost}>Press me to get file</button>

    <p>{status}</p>

</div>
<script>

    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'

    onMount(() => {
        invoke('print_assessors').then((assessors) => therapists = assessors);
        therapists = therapists;
    });

    async function submitPost() {
        try {

            let map = {
                "ADJUSTER": asmData.adjuster,
                "INSURANCE COMPANY": asmData.insCompany,
                "CLIENT FIRST": asmData.claimant.firstName,
                "CLIENT LAST": asmData.claimant.lastName,
                "DOB": asmData.claimant.dateOfBirth,
                "CLAIM NUMBER": asmData.claimNumber,
                "DOL": asmData.claimant.dateOfLoss,
                "DOA": asmData.dateOfAssessment,
                "CLIENT AGE": asmData.claimant.age,
                "REFERRAL COMPANY": asmData.referralCompany,
                "CLIENT ADDRESS": asmData.claimant.addressLong,
                "Template": asmData.type,
                "OCCUPATIONAL THERAPIST": asmData.therapist.salutation + ". " + asmData.therapist.first_name + " " + asmData.therapist.last_name,
                "HE---SHE_Lower": "hello",
                "MALE---FEMALE_Lower": "",
                "HIS---HER_Lower": "",
                "MALE---FEMALE_LOWER": "",
                "HE---SHE_Upper": "",
                "HIM---HER_Lower": "",
                "CLIENT SALUTATION": "",
                "Image": ""
            };


            if(asmData.claimant.gender == "male") {
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

            switch(asmData.therapist.first_name) {

                case "Joan":
                map["Image"] = "JS.png";
                break;

                case "Montana":
                map["Image"] = "MM.png";
                break;

                case "Anghela":
                map["Image"] = "AS.png";
                break;

                case "Josh":
                map["Image"] = "JM.png";
                break;

                default:
                map["Image"] = "JS.png";

            }

            // const send = JSON.stringify(Object.fromEntries(map));
            const send = JSON.stringify(map);

            invoke('test', {test: send});
            status="Saved.";

        } catch (exceptionVar){
            asmData.claimant.firstName = "did not work";
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
    let asmData = {
        "type": "",
        "path": "",
        "therapist": {
            "salutation": "",
            "first_name": "",
            "last_name": ""
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

