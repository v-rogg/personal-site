<script>
    import VanillaTilt from 'vanilla-tilt';

    VanillaTilt.init(document.querySelector('#sapper'), {
        max: 15,
        speed: 400,
        gyroscopeMinAngleY: -1,
        gyroscopeMaxAngleY: 1,
    });
</script>
