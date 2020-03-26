<script>

    import { lang } from '../stores';

    export let segment;
</script>

<style lang="scss">
    /*nav {*/
    /*    border-bottom: 1px solid rgba(255,62,0,0.1);*/
    /*    font-weight: 300;*/
    /*    padding: 0 1em;*/
    /*    ul {*/
    /*        margin: 0;*/
    /*        padding: 0;*/
    /*    }*/
    /*}*/

    ul {
    	margin: 0 -1em 0 -1em;
    	padding: 0;
    }

    a {
        font-size: 1.2rem;
        color: black;
        text-decoration: none;
        padding: 0;
        display: block;
    }

    .flex {
        display: flex;
        justify-content: start;
    }

    li {
        padding: 1em 1em 2em;
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
        /*font-weight: 700;*/
    }

    [aria-current]::after {
        position: absolute;
        content: '';
        width: calc(100% - .5em);
        height: 3px;
        background-color: black;
        display: block;
        bottom: -7px;
        margin-left: .25em;
    }

    /*a {*/
    /*    text-decoration: none;*/
    /*    padding: 1em 0.5em;*/
    /*    display: block;*/
    /*}*/
</style>

<nav class="container">
    <ul class="flex">
        {#if $lang === 'de'}
            <li><a aria-current='{segment === undefined ? "page" : undefined}' href='.'>Projekte</a></li>
        {/if}
        {#if $lang === 'en'}
            <li><a aria-current='{segment === undefined ? "page" : undefined}' href='.'>Projects</a></li>
        {/if}
        {#if $lang === 'de'}
            <li><a aria-current='{segment === undefined ? "page" : undefined}' href='.'>Bio</a></li>
        {/if}
        {#if $lang === 'en'}
            <li><a aria-current='{segment === "about" ? "page" : undefined}' href='about'>Bio</a></li>
        {/if}
        <!-- for the blog link, we're using rel=prefetch so that Sapper prefetches
             the blog data when we hover over the link or tap it on a touchscreen -->
        		<li><a rel=prefetch aria-current='{segment === "blog" ? "page" : undefined}' href='blog'>What did I teach my Mom today?</a></li>
    </ul>
</nav>
