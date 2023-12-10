<div>

    <h4>CAT</h4>

    <label for="date">OCF-19 Date:</label>
    <input type="text" id="date" bind:value={catData.date} required/>
    <br>

    <label for="assessor">OCF-19 Assessor:</label>
    <input type="text" id="assessor" bind:value={catData.assessor} required/>
    <br>

</div>
<script>

    import {onMount} from 'svelte'

    onMount(() => {
    });

    export let catData = {
        date: "",
        assessor: ""
    }

</script>
