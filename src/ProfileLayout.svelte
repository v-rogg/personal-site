<script>
    import Header from './Header.svelte';
    import CV from './Profile/CV.svelte';
    export let currentRoute;
</script>

<style>
    .rect {
        position: absolute;
        z-index: -1;
        top: 70px;
        left: auto;
        right: 20%;
        width: 480px;
        height: 480px;
        background: linear-gradient(to right, rgba(49, 207, 171, 0.5), #50e3c2);
        transform: none;
        transition: all 250ms ease-out;
    }
    .rect:hover {
        transform: translateY(-15%) translateX(-10%) rotate(5deg);
    }
    .circle {
        position: absolute;
        top: 270px;
        left: auto;
        right: 17%;
        width: 240px;
        height: 240px;
        border-radius: 50%;
        background: linear-gradient(to right, #2336b6, rgba(31, 38, 103, 0.9));
        transform: none;
        transition: all 250ms ease-out;
        transition-delay: 50ms;
        cursor: cell;
    }
    .circle:hover {
        transform: translateY(5%) translateX(10%);
    }
    .circle:hover + .rect {
        transform: translateY(-15%) translateX(-10%) rotate(5deg);
    }
</style>

<div>
    <div id="BG">
        <div class="circle"></div>
        <div class="rect"></div>
    </div>
    <Header {currentRoute}/>
    <CV {currentRoute}/>
</div>
