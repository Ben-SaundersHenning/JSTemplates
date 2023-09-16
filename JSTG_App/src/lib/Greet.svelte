<script>
    
    import {invoke} from '@tauri-apps/api/tauri'

    let name = ''
    let greetMsg = 'Hello, ----'
    let count = 1

    async function greet() {
        greetMsg = await invoke('greet', {name})
    }

    async function double() {
        count = await invoke('double', {count})
    }

</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}"/>
    <button on:click="{greet}">Greet</button>
    <p>{greetMsg}</p>
    <p>This is a test</p>
    <button on:click="{double}">Double this num: {count}</button>
</div>
