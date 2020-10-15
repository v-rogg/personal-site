
<style lang="sass">

    @import './src/styles/theme'

    #load
        position: fixed
        z-index: 1100
        width: 100vw
        height: 100vh
        top: 0
        left: 0
        background: white
        opacity: 0
        animation: 1000ms load ease-in-out
        display: flex
        justify-content: center
        align-items: center
        pointer-events: none


    @keyframes load
        0%
            opacity: 1
        100%
            opacity: 0



</style>

<div id="load">
</div>
