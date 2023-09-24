<div>
    <label for="type">Assessment Type:</label>
    <select id="type" bind:value={asmData.type}>
    {#each asmTypes as type}
        <option value={type}>{type}</option>
    {/each}
    </select>
    <br>

    <label for="therapist">Therapist:</label>
    <select id="threapist" bind:value={asmData.therapist}>
    {#each therapists as therapist}
        <option value={therapist}>{therapist}</option>
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
    <input type="date" id="claimDOB" bind:value={asmData.claimant.dateOfBirth}>
    <br>

    <label for="claimAge">Claimaint Age:</label>
    <input class="textbox" id="claimAge" type="text" bind:value={asmData.claimant.age}/>
    <br>

    <label for="claimDOL">Claimant DOL:</label>
    <input type="date" id="claimDOL" bind:value={asmData.claimant.dateOfLoss}>
    <br>

    <label for="claimAddress">Claimaint Address:</label>
    <input class="textbox" id="claimAddress" type="text" bind:value={asmData.claimant.addressLong}/>
    <br>

    <button on:click={submitPost}>Press me to get file</button>

</div>
<script>

    import {invoke} from '@tauri-apps/api/tauri'
    import {writeBinaryFile, writeFile} from '@tauri-apps/api/fs'
    import axios from 'axios';

    let fileLocation = {
        "baseTemplate": "/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/CAT.docx"
    }
    
    let path = "/home/ben/Downloads/test.docx"
    let url = 'http://localhost:5056/api/DocumentRequest'

    async function submitPost() {
        try {

        const data = JSON.stringify(fileLocation);

        // axios.post(options)
        axios.post(url, data, {responseType: 'blob', headers: {'content-type': 'application/json'}})
        .then((response) => {

            // var reader = new FileReader();
            // reader.readAsDataURL(response.data);
            // reader.onloadend = function () {
            //     var base64 = reader.result;
            //     invoke('save_bytes', {base64});
            // }

            const file = new Uint8Array(response.data);
            // const test = Array.from(response.data);
            // invoke('save_bytes', {test});


            // writeBinaryFile(path, file);

           //  // create file link in browser's memory
           //  const href = URL.createObjectURL(response.data);
           // // create "a" HTML element with href to file & click
           //  const link = document.createElement('a');
           //  link.href = href;
           //  link.setAttribute('download', 'file.docx'); //or any other extension
           //  document.body.appendChild(link);
           //  link.click();
           //
           //  // clean up "a" element & remove ObjectURL
           //  document.body.removeChild(link);
           //  URL.revokeObjectURL(href);
        })

        } catch (exceptionVar){
            asmData.claimant.firstName = "did not work";
        }
    }

    let greetMsg = ""
    let name = ""
    let genders = [
        "male",
        "female",
        "other"
    ]
    let therapists = [
        "Ms. Joan Saunders",
        "Ms. Montana Mullane",
        "Ms. Anghela Sivananthan",
        "Mr. Josh Melo"
    ]
    let asmTypes = [
        "AC",
        "AC MRB",
        "CAT",
        "CAT AC",
        "CAT AC MRB",
        "CAT CAT GOSE",
        "CAT GOSE",
        "CAT MRB_accidentally AC",
        "MRB",
        "NEB"
    ]
    let asmData = {
        "type": "",
        "therapist": {
            "salutation": "",
            "firstName": "",
            "lastName": "",
            "registationNumber": "",
            "qualifications": "",
        },
        "adjuster": "",
        "insCompany": "",
        "claimNumber": "",
        "dateOfAssessment": "",
        "seidenFileNumber": null,
        "claimant": {
            "salutation": "",
            "firstName": "TestFirstName",
            "lastName": "",

            "gender": "",
            "male-female": "",
            "he-she": "",
            "his-her": "",
            "himself-herself": "",

            "youth": false,
            "dateOfBirth": "",
            "age": 50,
            "dateOfLoss": "",
            "addressLong": "",
            "country": "",
            "province": "",
            "street": "",
            "streeNum": 100,
            "postalCode": ""
        },
        "questions": {
            "1": "",
            "2": "",
        }
    }

    async function greet() {
        greetMsg = await invoke('greet', {name})
    }

</script>

