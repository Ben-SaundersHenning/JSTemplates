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

    async function submitPost() {
        try {

            let test = {};
            let map = {};

            test["hello"] = asmData.adjuster;

            map["OCCUPATIONAL THERAPIST"] = asmData.therapist.salutation + asmData.therapist.firstName + asmData.therapist.lastName;
            map["ADJUSTER"] = asmData.adjuster;
            map["INSURANCE COMPANY"] = asmData.insCompany;
            map["CLIENT SALUTATION"] = asmData.claimant.salutation;
            map["CLIENT FIRST"] = asmData.claimant.firstName;
            map["CLIENT LAST"] = asmData.claimant.lastName;
            map["DOB"] = asmData.claimant.dateOfBirth;
            map["CLAIM NUMBER"] = asmData.claimNumber;
            map["DOL"] = asmData.claimant.dateOfLoss;
            map["DOA"] = asmData.dateOfAssessment;
            map["CLIENT AGE"] = asmData.claimant.age;
            map["REFERRAL COMPANY"] = asmData.referralCompany;

                map["HE---SHE_Lower"] = "";
                map["MALE---FEMALE_Lower"] = "";
                map["HIS---HER_Lower"] = "";
                map["HE---SHE_Upper"] = "";
                map["HIM---HER_Lower"] = "";

            if(asmData.claimant.gender == "male") {
                map["HE---SHE_Lower"] = "he";
                map["MALE---FEMALE_Lower"] = "male";
                map["HIS---HER_Lower"] = "his";
                map["HE---SHE_Upper"] = "He";
                map["HIM---HER_Lower"] = "him";
            } else {
                map["HE---SHE_Lower"] = "she";
                map["MALE---FEMALE_Lower"] = "female";
                map["HIS---HER_Lower"] = "her";
                map["HE---SHE_Upper"] = "She";
                map["HIM---HER_Lower"] = "her";
            }

            map["CLIENT ADDRESS"] = asmData.claimant.addressLong;

            // const send = JSON.stringify(Object.fromEntries(map));
            const send = JSON.stringify(map);

            invoke('test', {test: send});

        } catch (exceptionVar){
            asmData.claimant.firstName = "did not work";
        }
    }

    // let map = new Object();

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
        "type": "blah",
        "therapist": {
            "salutation": "blah",
            "firstName": "blah",
            "lastName": "blah",
            "registationNumber": "blah",
            "qualifications": "blah",
        },
        "adjuster": "blah",
        "insCompany": "blah",
        "claimNumber": "blah",
        "referralCompany": "blah",
        "dateOfAssessment": "blah",
        "seidenFileNumber": null,
        "claimant": {
            "salutation": "blah",
            "firstName": "TestFirstName",
            "lastName": "blah",

            "gender": "blah",
            "male-female": "blah",
            "he-she": "blah",
            "his-her": "blah",
            "himself-herself": "blah",

            "youth": "false",
            "dateOfBirth": "blah",
            "age": "50",
            "dateOfLoss": "blah",
            "addressLong": "blah",
            "country": "blah",
            "province": "blah",
            "street": "blah",
            "streeNum": "100",
            "postalCode": "blah"
        },
        "questions": {
            "1": "blah",
            "2": "blah",
        }
    }

</script>

