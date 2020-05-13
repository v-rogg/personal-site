<script>
    import Lang from './Lang.svelte';
</script>

<style>
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-top: 1rem;
        /*background: rgba(255, 255, 255, 1);*/
    }

    .pointer {
        cursor: pointer;
    }

    h1 {
        text-transform: uppercase;
        font-size: 2rem;
        font-weight: 700;
    }

    a {
        width: 30%;
    }

    .logo {
        width: 40%;
        transform: translateX(1.5px);
    }

    @media (max-width: 991.98px) {
        .header {
            flex-wrap: wrap;
        }

        a {
            order: 2;
            width: 70%;
            /*flex: 2 0 50%;*/
        }
        .logo {
            order: 1;
            width: 100%;
            min-width: 100%;
            margin-bottom: 1rem;
            /*padding-bottom: 1rem;*/
        }
    }

    a {
        color: black;
        text-decoration: none;
    }
</style>


<section class="header container">
    <a class="pointer" href='.'><h1>Valentin Rogg</h1></a>

<!--    <svg class="logo" style="max-width: 12rem; max-height: 3rem;" version="1.1" id="Valentin Rogg" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 338 100" xml:space="preserve">-->
<!--        <path style="fill:#FF8000;" d="M97,3l0,94H3L3,3H97 M100,0H0l0,100h100L100,0L100,0z"/>-->
<!--        <rect x="0" y="0" style="opacity:0.33; fill:#FF8000;" width="100" height="100"/>-->
<!--        <path style="fill:#FF8000;" d="M63.6,36.4l0,27.1H36.4l0-27.1H63.6 M66.6,33.4H33.4l0,33.1h33.1L66.6,33.4L66.6,33.4z"/>-->
<!--        <rect x="33.4" y="33.4" style="fill:#FF8000; opacity:0.33;" width="33.1" height="33.1"/>-->
<!--	    <circle style="opacity:0.33;fill:#00BFA8;" cx="166.8" cy="50" r="50"/>-->
<!--		<path style="fill:#00BFA8;" d="M166.8,3c25.9,0,47,21.1,47,47s-21.1,47-47,47s-47-21.1-47-47S140.9,3,166.8,3 M166.8,0c-27.6,0-50,22.4-50,50s22.4,50,50,50c27.6,0,50-22.4,50-50S194.4,0,166.8,0L166.8,0z"/>-->
<!--	    <circle style="opacity:0.33;fill:#00BFA8;" cx="166.8" cy="50" r="17"/>-->
<!--		<path style="fill:#00BFA8;" d="M166.8,36c7.7,0,14,6.3,14,14s-6.3,14-14,14s-14-6.3-14-14S159.1,36,166.8,36 M166.8,33c-9.4,0-17,7.6-17,17s7.6,17,17,17c9.4,0,17-7.6,17-17S176.2,33,166.8,33L166.8,33z"/>-->
<!--		<path style="fill:#FF4040;" d="M279.6,6l52.5,91H227L279.6,6 M279.6,0l-57.7,100h115.5L279.6,0L279.6,0z"/>-->
<!--		<polygon style="opacity:0.33; fill:#FF4040;" points="279.6,0 221.8,100 337.3,100"/>-->
<!--		<path style="fill:#FF4040;" d="M279.6,50.5l13.3,23.1h-26.7L279.6,50.5 M279.6,44.5l-18.5,32.1h37.1L279.6,44.5L279.6,44.5z"/>-->
<!--		<polygon style="opacity:0.33; fill:#FF4040;" points="279.6,44.5 261.1,76.6 298.1,76.6"/>-->
<!--    </svg>-->

    <Lang/>
</section>
