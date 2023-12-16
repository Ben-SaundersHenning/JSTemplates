<div>

    <h4>AC + F1</h4>

    <label for="firstAsmt">First Assessment:</label>
    <input type="checkbox" id="firstAsmt" bind:checked={acData.first_assessment}/>
    <br>

    <label for="DOLA">Date of Last Assessment:</label>
    <input type="text" id="DOLA" bind:value={acData.date_of_last_assessment}/>
    <br>

    <label for="monthlyAllowance">Monthly Allowance:</label>
    <input type="text" id="monthlyAllowance" bind:value={acData.current_monthly_allowance}/>
    <br>

</div>
<script>

    import {onMount} from 'svelte'

    onMount(() => {
    });

    //hourly rates are calculated on Rust side.
    export let acData = {
        first_assessment: false,
        current_monthly_allowance: "",
        hourly_rates: [],
        date_of_last_assessment: ""
    }

</script>
