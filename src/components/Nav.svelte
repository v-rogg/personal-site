<script>

    import { lang } from '../stores';

    export let segment;
</script>

<style lang="scss">

    ul {
    	margin: 0 -4em 0 -4em;
        padding: 0 3em;
    }



    .flex {
        display: flex;
        justify-content: start;
    }

    .nav {
    }

    li {
        padding: 1em 1em 1em;
    }

    /* clearfix */
    ul::after {
        content: '';
        display: block;
        clear: both;
    }

    /*li {*/
    /*    display: block;*/
    /*    float: left;*/
    /*}*/

    [aria-current] {
        position: relative;
        display: inline-block;
    }

    [aria-current]::after {
        position: absolute;
        content: '';
        width: calc(100% - .5em);
        height: 3px;
        display: block;
        bottom: -7px;
        margin-left: .25em;
    }

    .a::after {
        background-color: #FF8000;
    }

    .b::after {
        background-color: #00BFA8;
    }

    /*.c::after {*/
    /*    background-color: #FF4040;*/
    /*}*/

</style>

<nav class="container">
    <ul class="flex nav">
        {#if $lang === 'de'}
            <li><a aria-current='{segment === undefined ? "page" : undefined}' class='a' href='.'>Projekte</a></li>
        {/if}
        {#if $lang === 'en'}
            <li><a aria-current='{segment === undefined ? "page" : undefined}' class='a' href='.'>Projects</a></li>
        {/if}
        {#if $lang === 'de'}
            <li><a aria-current='{segment === "about" ? "page" : undefined}' class='b' href='about'>Über mich</a></li>
        {/if}
        {#if $lang === 'en'}
            <li><a aria-current='{segment === "about" ? "page" : undefined}' class='b' href='about'>About</a></li>
        {/if}
<!--        <li><a aria-current='{segment === "repertoire" ? "page" : undefined}' class='{segment === "repertoire" ? "c" : undefined}' href='repertoire'>Repertoire</a></li>-->
    </ul>
</nav>
