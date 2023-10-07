<div class="body-text">
    <slot/>
</div>

<nav class="sidebar">
    <a href="/">Home</a>
    <a href="/docgen">Doc Gen</a>
</nav>

<style>
.sidebar {
height: 100%;
width: 150px;
position: absolute;
left: 0;
top: 0;
padding-top: 40px;
background-color: lightsalmon;
}

.sidebar a {
padding: 8px;
font-size: 24px;
display: block;
}

.body-text {
margin-left: 150px;
font-size: 18px;
}
</style>
