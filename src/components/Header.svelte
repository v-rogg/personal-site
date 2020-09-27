<style lang="scss">
    @import "./src/styles/theme";

    header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        //margin: 3rem auto;
        margin: 0 auto 2rem;
    }

    @media (max-width: 991.98px) {
        header {
            flex-direction: column-reverse;
        }

        img {
            margin: 0 0 2rem;
        }

        .information {
            text-align: center;
        }

        a {
            justify-content: center;
        }
    }

    h1, h2 {
        font-family: $font-family-sans;
        margin: 0;
    }

    h1 {
        font-size: $font-size-h1;
        position: relative;

        &:before {
            position: absolute;
            content: '~~';
            left: calc(-1em - 1rem);
            font-family: 'JetBrains Mono', sans-serif;
            top: 50%;
            line-height: 0;
            opacity: 0;
            transition: opacity linear 50ms;
        }

        &:hover {
            color: $magenta;

            &:before {
                opacity: 1;
            }
        }
    }

    h2 {
        font-size: $font-size-h2;
    }

    img {
        max-height: 256px;
    }

    .information {
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        align-self: stretch;
    }

    #title {
        margin-bottom: 2rem;
        position: relative;
    }

    #contact {

        color: lighten(#4f4948, 30);

        a {
            display: flex;
            align-items: center;

            &:hover {
                font-weight: $font-weight-bold;

                svg {
                    //stroke: black;
                    //stroke-width: 1.5px;
                }
            }

            svg {
                margin-right: .5rem;
                height: 1em;
                width: 1em;
            }
        }

        div {
            margin-top: 1rem;
        }
    }
</style>

<header class="container">
    <div class="information">
        <div id="title" class="header">
            <h1 class="tilt"><a rel=external href="/">Valentin Rogg</a></h1>
            <h2>Entrepreneur aus Augsburg</h2>
        </div>
        <div id="contact">
            <a href="mailto:mail@valentinrogg.de">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd" d="M3.83302 4C3.52772 6.1111 3.2268 11.8126 7.7071 16.2929C12.1874 20.7732 17.8889 20.4723 20 20.167V15.8471L15.5641 15.1078L14.8944 16.4472C14.725 16.786 14.3788 17 14 17V16C14 17 13.9995 17 13.999 17L13.9979 17L13.9957 17L13.9907 17L13.9783 16.9998C13.9691 16.9997 13.9579 16.9994 13.9448 16.999C13.9186 16.9981 13.8847 16.9966 13.8436 16.9939C13.7615 16.9884 13.6506 16.9781 13.5148 16.9587C13.2435 16.9199 12.8719 16.8447 12.4338 16.6987C11.5541 16.4054 10.4151 15.8293 9.29289 14.7071C8.17073 13.5849 7.59455 12.4459 7.30131 11.5662C7.15527 11.1281 7.08005 10.7565 7.0413 10.4852C7.02191 10.3494 7.01159 10.2385 7.00612 10.1564C7.00338 10.1153 7.00185 10.0814 7.00101 10.0552C7.00058 10.0421 7.00033 10.0309 7.00018 10.0217L7.00004 10.0093L7.00001 10.0043L7 10.002L7 10.001C7 10.0005 7 10 8 10H7C7 9.66565 7.1671 9.35342 7.4453 9.16795L9.77018 7.61803L8.32296 4H3.83302ZM9.07364 10.4861L10.8796 9.28213C11.6665 8.75751 11.9784 7.75338 11.6271 6.87525L10.1799 3.25722C9.87619 2.4979 9.14077 2 8.32296 2H3.78077C2.87226 2 2.01708 2.63116 1.86794 3.6169C1.52863 5.8595 1.06678 12.481 6.29289 17.7071C11.519 22.9332 18.1405 22.4714 20.3831 22.1321C21.3688 21.9829 22 21.1277 22 20.2192V15.8471C22 14.8694 21.2932 14.0351 20.3288 13.8743L15.8929 13.135C15.0269 12.9907 14.1679 13.4281 13.7753 14.2134L13.4288 14.9064C13.3199 14.8796 13.1983 14.8453 13.0662 14.8013C12.4459 14.5946 11.5849 14.1707 10.7071 13.2929C9.82927 12.4151 9.40545 11.5541 9.19868 10.9338C9.14281 10.7662 9.10259 10.6154 9.07364 10.4861Z" fill="#282828"></path></svg>
                mail@valentinrogg.de
            </a>
            <a href="tel:00491757090991">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd" d="M2 6C2 4.89543 2.89543 4 4 4H20C21.1046 4 22 4.89543 22 6V18C22 19.1046 21.1046 20 20 20H4C2.89543 20 2 19.1046 2 18V6ZM5.51859 6L12 11.6712L18.4814 6H5.51859ZM20 7.32877L12.6585 13.7526C12.2815 14.0825 11.7185 14.0825 11.3415 13.7526L4 7.32877V18H20V7.32877Z" fill="#282828"></path></svg>
                +49 175 70 90 991
            </a>
        </div>
    </div>

    <img src="me-512.webp" alt="Valentin Rogg Profilbild">
</header>
