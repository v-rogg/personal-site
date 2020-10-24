<style lang="sass">
    @import "./src/styles/theme"
</style>

<h3>
    Dart-Punkteerkennung mithilfe eines LiDAR-Sensors
</h3>
<h4>
    Robin Hood
</h4>
<p>
    Dart ist ein allgemein beliebter Sport.
    Jedoch beeinflussen fast alle gängigen elektronischen Punktzählsysteme das Spielerlebnis.
    So wird die Wettkampf-Dart-Scheibe, welche normalerweise aus einer Mischung aus Sisal-Agave Fasern und Holz besteht, durch eine Plastik-Scheibe ersetzt.
    Auf diese kann auch nicht mit den normalen Metall-Darts geworfen werden, sondern nur mit speziellen Plastik-Darts.
    <br />
    <br />
    Bislang gibt es nur ein System, welches auch bei Benutzung einer Wettkampfausstattung, automatisch Punkte zählen kann.
    Dieses erkennt die Darts mithilfe 3 Kameras.
    Um eine gute Kalibrierung zu garantieren, kann nur auf Dartscheiben mit dem klassischen Schwarz/Weiß/Rot/Grün-Färbung gespielt werden.
    Außerdem ist die Vorrichtung sehr auffällig und mit mindestens 750€ bis hin zu 1500€ nicht gerade preiswert.
    https://scoliadarts.com/scolia-pro/
    <br />
    <br />
    Mit „Robin Hood“ möchte ich eine, auf einem LiDAR-Sensor basierende, Alternativlösung bauen, welche kostengünstig und unauffällig.
    Außerdem funktioniert soll sie mit klassischer Ausstattung funktionieren und einfach nachrüstbar sein.
    <br />
    <br />
    Mein System basiert auf einem RPLiDAR A1, welcher an einen Raspberry Pi 4B angeschlossen wird. Der RPLiDAR A1 ist ein einfacher 360° LiDAR-Sensor.
    Ein LiDAR-Sensor ist vergleichbar zu Radar-Systemen, benutzt jedoch einen Infrarot-Laser, um die Entfernung zu Objekten zu messen.
    Es handelt sich dabei um einen Time-of-Flight-Sensor.
    Der Sensor dreht sich sechs Mal die Sekunde um die eigene Achse.
    Damit kann er für SLAM (Simultaneous Localization and Mapping) Aufgaben benutzt werden.
    Auf der nächsten Seite sieht man eine Schnappschussmessung mit dem Sensor platziert auf meinem Tisch.
    <br />
    <br />
    Begleitenden zur Hardware und der Punkteerfassung wird im Kurs „Netzwerktechnologien 2“ eine Web-Applikation programmiert, um Spiele, z.B. 501 zu 0, zu spielen oder um Spielerprofile zu verwalten.
</p>
