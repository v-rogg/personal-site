<style>
    @import url('https://fonts.googleapis.com/css?family=Source+Sans+Pro:300,400,600,700&display=swap');

    :global(main) {
        font-family: 'Source Sans Pro', sans-serif;
    }

    :global(.mono) {
        font-family: 'JetBrains Mono', sans-serif;
    }

    :global(h2) {
        text-transform: uppercase;
        margin-top: .5rem;
        margin-bottom: 0;
        font-weight: 400;
        line-height: 1.5rem;
        font-size: 1.5rem;
    }

    :global(h3) {
        text-transform: uppercase;
        font-weight: 400;
        font-size: 1.2rem;
    }

    :global(h4) {
        text-transform: uppercase;
        font-weight: 400;
        font-size: 1.125rem;
    }

    :global(b) {
        font-weight: 700;
    }

    :global(.container){
        max-width: 1140px;
        margin: 0 auto;
        padding: 0 1rem;
    }
    @media (min-width: 1200px) {
        :global(.container) {
            max-width: 1140px;
        }
    }
    @media (min-width: 992px) and (max-width: 1199.98px) {
        :global(.container) {
            max-width: 960px;
        }
    }
    @media (min-width: 768px) and (max-width: 991.98px) {
        :global(.container) {
            max-width: 720px;
        }
    }
    @media (min-width: 576px) and (max-width: 767.98px)  {
        :global(.container) {
            max-width: 540px;
        }
    }

</style>