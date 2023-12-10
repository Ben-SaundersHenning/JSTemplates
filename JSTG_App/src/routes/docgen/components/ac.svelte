<div>

    <h4>AC + F1</h4>

    <label for="firstAsmt">First Assessment:</label>
    <input type="checkbox" id="firstAsmt" bind:checked={acData.firstAssessment}/>
    <br>

    <label for="DOLA">Date of Last Assessment:</label>
    <input type="text" id="DOLA" bind:value={acData.dateOfLastAssessment} required/>
    <br>

    <label for="monthlyAllowance">Monthly Allowance:</label>
    <input type="text" id="monthlyAllowance" bind:value={acData.currentMonthlyAllowance} required/>
    <br>

</div>
<script>

    import {onMount} from 'svelte'

    onMount(() => {
    });

    //hourly rates are calculated on Rust side.
    export let acData = {
        "firstAssessment": true,
        "currentMonthlyAllowance": "",
        "hourlyRates": [],
        "dateOfLastAssessment": ""
    }

</script>
