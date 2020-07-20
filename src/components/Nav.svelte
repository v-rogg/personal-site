<script>
    import { lang } from '../stores';

    export let segment;
</script>

<style lang="scss">
    @import "../../static/styles/theme.scss";

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

    a {
        font-size: 1.2rem;
        color: black;
        text-decoration: none;
        padding: 0;
        /*display: block;*/
        position: relative;
        display: inline-block;

        &:hover {
            &:after {
                position: absolute;
                content: '';
                width: calc(100% - .5em);
                height: 1.5px;
                display: block;
                bottom: -7px;
                margin-left: .25em;
                /*background-color: #bfbfbf;*/
                /*background-color: #FF8000;*/
            }
        }

        &:hover {
            &:after {
                position: absolute;
                content: '';
                width: calc(100% - .5em);
                height: 3px;
                display: block;
                bottom: -7px;
                margin-left: .25em;
                /*background-color: #bfbfbf;*/
                /*background-color: #00BFA8;*/
            }
        }
    }

    [aria-current] {
        position: relative;
        display: inline-block;
    }

    [aria-current]::after {
        position: absolute;
        content: '';
        width: calc(100% - .5em);
        height: 1.5px;
        display: block;
        bottom: -7px;
        margin-left: .25em;
    }

    .a::after {
        // background-color: $orange;
        background-color: #551A8B;
    }

    .b::after {
        // background-color: $blue;
        background-color: #551A8B;
    }

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
    </ul>
</nav>
