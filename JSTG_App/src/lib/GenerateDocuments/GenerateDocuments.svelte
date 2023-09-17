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
    <input type="date" id="claimDOB" bind:value={asmData.claimant.gender}>
    <br>

    <label for="claimAge">Claimaint Age:</label>
    <input class="textbox" id="claimAge" type="text" bind:value={asmData.claimant.age}/>
    <br>

            "age": 50,
            "dateOfLoss": "",
            "addressLong": "",
            "country": "",
            "province": "",
            "street": "",
            "streeNum": 100,
            "postalCode": ""

</div>
<script>

    import {invoke} from '@tauri-apps/api/tauri'

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

