<div>

    <h4>AC + F1</h4>

</div>
<script>

    import {onMount} from 'svelte'

    onMount(() => {
    });

    // export let acData = {}
    export let acData = {
        "firstAssessment": true,
        "currentMonthlyAllowance": "",
        "hourlyRates": { 
            "part1": "",
            "part2": "",
            "part3": ""
        },
        "dateOfLastAssessment": ""
    }

    acData.firstAssessment = false;
    acData.currentMonthlyAllowance = "1045";

</script>
