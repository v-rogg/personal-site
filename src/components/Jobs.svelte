<style lang="scss">
    @import "static/styles/theme.scss";

    ul {
        margin: 0;
        padding: 0;
        list-style: none;
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: .5rem;
        z-index: -1;

        li {
            margin: 0;
            padding: 0;
            /*font-weight: 700;*/
            /*font-size: 1.25rem;*/
            &:nth-child(3n-2) {
                color: $orange;
            }
            &:nth-child(3n-1) {
                color: $blue;
            }
            &:nth-child(3n) {
                color: $red;
            }
        }
    }

    section {
        /*<!--margin-top: -3rem;-->*/
        margin-bottom: 5rem;
    }

    /* Shrink */
    .hvr-shrink {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
        -webkit-transition-duration: 0.3s;
        transition-duration: 0.3s;
        -webkit-transition-property: transform;
        transition-property: transform;
    }
    .hvr-shrink:hover, .hvr-shrink:focus, .hvr-shrink:active {
        -webkit-transform: scale(0.9);
        transform: scale(0.9);
    }

    /* Pulse */
    @-webkit-keyframes hvr-pulse {
        25% {
            -webkit-transform: scale(1.1);
            transform: scale(1.1);
        }
        75% {
            -webkit-transform: scale(0.9);
            transform: scale(0.9);
        }
    }
    @keyframes hvr-pulse {
        25% {
            -webkit-transform: scale(1.1);
            transform: scale(1.1);
        }
        75% {
            -webkit-transform: scale(0.9);
            transform: scale(0.9);
        }
    }
    .hvr-pulse {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
    }
    .hvr-pulse:hover, .hvr-pulse:focus, .hvr-pulse:active {
        -webkit-animation-name: hvr-pulse;
        animation-name: hvr-pulse;
        -webkit-animation-duration: 1s;
        animation-duration: 1s;
        -webkit-animation-timing-function: linear;
        animation-timing-function: linear;
        -webkit-animation-iteration-count: infinite;
        animation-iteration-count: infinite;
    }

    /* Rotate */
    .hvr-rotate {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
        -webkit-transition-duration: 0.3s;
        transition-duration: 0.3s;
        -webkit-transition-property: transform;
        transition-property: transform;
    }
    .hvr-rotate:hover, .hvr-rotate:focus, .hvr-rotate:active {
        -webkit-transform: rotate(4deg);
        transform: rotate(4deg);
    }

    /* Hang */
    @-webkit-keyframes hvr-hang {
        0% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
        50% {
            -webkit-transform: translateY(4px);
            transform: translateY(4px);
        }
        100% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
    }
    @keyframes hvr-hang {
        0% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
        50% {
            -webkit-transform: translateY(4px);
            transform: translateY(4px);
        }
        100% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
    }
    @-webkit-keyframes hvr-hang-sink {
        100% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
    }
    @keyframes hvr-hang-sink {
        100% {
            -webkit-transform: translateY(8px);
            transform: translateY(8px);
        }
    }
    .hvr-hang {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
    }
    .hvr-hang:hover, .hvr-hang:focus, .hvr-hang:active {
        -webkit-animation-name: hvr-hang-sink, hvr-hang;
        animation-name: hvr-hang-sink, hvr-hang;
        -webkit-animation-duration: .3s, 1.5s;
        animation-duration: .3s, 1.5s;
        -webkit-animation-delay: 0s, .3s;
        animation-delay: 0s, .3s;
        -webkit-animation-timing-function: ease-out, ease-in-out;
        animation-timing-function: ease-out, ease-in-out;
        -webkit-animation-iteration-count: 1, infinite;
        animation-iteration-count: 1, infinite;
        -webkit-animation-fill-mode: forwards;
        animation-fill-mode: forwards;
        -webkit-animation-direction: normal, alternate;
        animation-direction: normal, alternate;
    }

    /* Skew */
    .hvr-skew {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
        -webkit-transition-duration: 0.3s;
        transition-duration: 0.3s;
        -webkit-transition-property: transform;
        transition-property: transform;
    }
    .hvr-skew:hover, .hvr-skew:focus, .hvr-skew:active {
        -webkit-transform: skew(-10deg);
        transform: skew(-10deg);
    }

    /* Wobble Horizontal */
    @-webkit-keyframes hvr-wobble-horizontal {
        16.65% {
            -webkit-transform: translateX(8px);
            transform: translateX(8px);
        }
        33.3% {
            -webkit-transform: translateX(-6px);
            transform: translateX(-6px);
        }
        49.95% {
            -webkit-transform: translateX(4px);
            transform: translateX(4px);
        }
        66.6% {
            -webkit-transform: translateX(-2px);
            transform: translateX(-2px);
        }
        83.25% {
            -webkit-transform: translateX(1px);
            transform: translateX(1px);
        }
        100% {
            -webkit-transform: translateX(0);
            transform: translateX(0);
        }
    }
    @keyframes hvr-wobble-horizontal {
        16.65% {
            -webkit-transform: translateX(8px);
            transform: translateX(8px);
        }
        33.3% {
            -webkit-transform: translateX(-6px);
            transform: translateX(-6px);
        }
        49.95% {
            -webkit-transform: translateX(4px);
            transform: translateX(4px);
        }
        66.6% {
            -webkit-transform: translateX(-2px);
            transform: translateX(-2px);
        }
        83.25% {
            -webkit-transform: translateX(1px);
            transform: translateX(1px);
        }
        100% {
            -webkit-transform: translateX(0);
            transform: translateX(0);
        }
    }
    .hvr-wobble-horizontal {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
    }
    .hvr-wobble-horizontal:hover, .hvr-wobble-horizontal:focus, .hvr-wobble-horizontal:active {
        -webkit-animation-name: hvr-wobble-horizontal;
        animation-name: hvr-wobble-horizontal;
        -webkit-animation-duration: 1s;
        animation-duration: 1s;
        -webkit-animation-timing-function: ease-in-out;
        animation-timing-function: ease-in-out;
        -webkit-animation-iteration-count: 1;
        animation-iteration-count: 1;
    }

    /* Buzz Out */
    @-webkit-keyframes hvr-buzz-out {
        10% {
            -webkit-transform: translateX(3px) rotate(2deg);
            transform: translateX(3px) rotate(2deg);
        }
        20% {
            -webkit-transform: translateX(-3px) rotate(-2deg);
            transform: translateX(-3px) rotate(-2deg);
        }
        30% {
            -webkit-transform: translateX(3px) rotate(2deg);
            transform: translateX(3px) rotate(2deg);
        }
        40% {
            -webkit-transform: translateX(-3px) rotate(-2deg);
            transform: translateX(-3px) rotate(-2deg);
        }
        50% {
            -webkit-transform: translateX(2px) rotate(1deg);
            transform: translateX(2px) rotate(1deg);
        }
        60% {
            -webkit-transform: translateX(-2px) rotate(-1deg);
            transform: translateX(-2px) rotate(-1deg);
        }
        70% {
            -webkit-transform: translateX(2px) rotate(1deg);
            transform: translateX(2px) rotate(1deg);
        }
        80% {
            -webkit-transform: translateX(-2px) rotate(-1deg);
            transform: translateX(-2px) rotate(-1deg);
        }
        90% {
            -webkit-transform: translateX(1px) rotate(0);
            transform: translateX(1px) rotate(0);
        }
        100% {
            -webkit-transform: translateX(-1px) rotate(0);
            transform: translateX(-1px) rotate(0);
        }
    }
    @keyframes hvr-buzz-out {
        10% {
            -webkit-transform: translateX(3px) rotate(2deg);
            transform: translateX(3px) rotate(2deg);
        }
        20% {
            -webkit-transform: translateX(-3px) rotate(-2deg);
            transform: translateX(-3px) rotate(-2deg);
        }
        30% {
            -webkit-transform: translateX(3px) rotate(2deg);
            transform: translateX(3px) rotate(2deg);
        }
        40% {
            -webkit-transform: translateX(-3px) rotate(-2deg);
            transform: translateX(-3px) rotate(-2deg);
        }
        50% {
            -webkit-transform: translateX(2px) rotate(1deg);
            transform: translateX(2px) rotate(1deg);
        }
        60% {
            -webkit-transform: translateX(-2px) rotate(-1deg);
            transform: translateX(-2px) rotate(-1deg);
        }
        70% {
            -webkit-transform: translateX(2px) rotate(1deg);
            transform: translateX(2px) rotate(1deg);
        }
        80% {
            -webkit-transform: translateX(-2px) rotate(-1deg);
            transform: translateX(-2px) rotate(-1deg);
        }
        90% {
            -webkit-transform: translateX(1px) rotate(0);
            transform: translateX(1px) rotate(0);
        }
        100% {
            -webkit-transform: translateX(-1px) rotate(0);
            transform: translateX(-1px) rotate(0);
        }
    }
    .hvr-buzz-out {
        display: inline-block;
        vertical-align: middle;
        -webkit-transform: perspective(1px) translateZ(0);
        transform: perspective(1px) translateZ(0);
        box-shadow: 0 0 1px rgba(0, 0, 0, 0);
    }
    .hvr-buzz-out:hover, .hvr-buzz-out:focus, .hvr-buzz-out:active {
        -webkit-animation-name: hvr-buzz-out;
        animation-name: hvr-buzz-out;
        -webkit-animation-duration: 0.75s;
        animation-duration: 0.75s;
        -webkit-animation-timing-function: linear;
        animation-timing-function: linear;
        -webkit-animation-iteration-count: 1;
        animation-iteration-count: 1;
    }
</style>

<section class="container">
    <ul>
        <li class="hvr-pulse"><h3>IoT Engineer</h3></li>
        <li class="hvr-wobble-horizontal"><h3>Project Manager</h3></li>
        <li class="hvr-shrink"><h3>Full-Stack Web</h3></li>
        <li class="hvr-rotate"><h3>DevSecOps</h3></li>
        <li class="hvr-skew"><h3>Designer</h3></li>
        <li class="hvr-hang"><h3>Motion Artist</h3></li>
        <li class="hvr-buzz-out"><h3>Musician</h3></li>
    </ul>
</section>
