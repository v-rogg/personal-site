---
title: Wassererbe Augsburg
subtitle: Interaktive Website zur Entwicklung der Wassernutzung einer Stadt
slug: Wassererbe Augsburg
date: 2022-02-01
tags:
  - information design
  - ui/ux
  - historic data
  - svelte
  - svg
width: 4
---

# Wassererbe Augsburg: Interaktive Website zur Geschichte einer wasserreichen Stadt

Augsburg ist seit jeher eine Stadt, die vom Wasser geprägt ist. Mit über **530 Brücken** und einem weitverzweigten Netz aus **100 Kilometern Kanälen** erzählt das Stadtbild von einer engen Verbindung zu dieser lebenswichtigen Ressource. Dieses Zusammenspiel von Stadt und Wasser wollten wir mit dem Projekt **„Wassererbe Augsburg“** interaktiv erfahrbar machen. Entstanden ist die Website in Zusammenarbeit mit **Nadine Keller** im Rahmen des Kurses **„Datenvisualisierung“** im Studiengang **„Interaktive Mediensysteme“** an der Technischen Hochschule Augsburg.

## Von der Idee zur Umsetzung

Eine wichtige Inspiration für die Website war das Brettspiel **„Die Siedler von Catan“**, dessen hexagonale Spielfeldstruktur uns die Grundlage für die Karte lieferte. Die Orientierung der Karte **nach Westen** basiert auf alten historischen Stadtplänen Augsburgs, die ebenfalls in diese Richtung ausgerichtet waren. Ziel war es, eine lebendige und zugleich übersichtliche Darstellung der städtischen Wassernutzung von **1890 bis 2022** zu schaffen.

Die Karte bildet den Mittelpunkt der Website. Hier laufen alle Daten und Interaktionen zusammen: Jedes **Hexagon** steht für eine Zone der Stadt und kann durch die hinterlegten Informationen farblich hervorgehoben werden. Die Umsetzung basiert auf einer **SVG-Datei**, in der jedes Hexagon eine eindeutige ID erhalten hat. So lassen sich Animationen, wie das **Ein- und Ausblenden von Beschreibungen oder Flussanimationen**, gezielt steuern.

## Technische Details: Dynamik mit SvelteKit

Die Website wurde mit dem Framework **SvelteKit** realisiert. Die Wahl fiel darauf, weil es die Entwicklung von dynamischen, komponentenbasierten Anwendungen erleichtert.

Alle Informationen – sei es die Nutzung der Stadtzonen oder die Geschichten zur Entwicklung Augsburgs – sind in **JSON-Dateien** gespeichert. Diese Trennung von Daten und Darstellung hat uns Flexibilität gegeben: Neue Zeiträume oder Geschichten können einfach ergänzt werden, ohne den bestehenden Code zu verändern.

Der **Zeitstrahl**, der die Jahre von 1890 bis 2022 abdeckt, generiert sich ebenfalls dynamisch aus den JSON-Daten. Mithilfe eines Svelte <code>tweened</code> Store haben wir einen flüssigen „Zeitraffer-Effekt“ realisiert, bei dem die Karte zwischen den Jahren animiert wechselt.

## Herausforderungen bei historischen Daten

Die Arbeit mit historischen Daten brachte einige Herausforderungen mit sich. So war das **Klassifizieren alter Karten** teilweise schwierig, da die Zonen auf diesen Karten oft ungenau oder uneindeutig markiert waren. Auch das Bereinigen der **Open Data**, die für das Projekt genutzt wurden, nahm viel Zeit in Anspruch – offene Daten sind nicht immer sauber und erfordern eine gründliche Aufbereitung. Dennoch war es spannend, mit diesen Quellen zu arbeiten und die historische Entwicklung der Stadt Stück für Stück nachzuzeichnen.

## Veröffentlichung und Performance

Für die Veröffentlichung wird eine **Cloudflare Pages Pipeline** genutzt. Änderungen im GitHub-Repository werden automatisch erkannt, der Code wird optimiert und weltweit über das Cloudflare CDN bereitgestellt. Das sorgt für schnelle Ladezeiten, egal von wo die Seite aufgerufen wird.

## Flexibel auf allen Geräten

Dank der modularen Struktur der Website konnten wir auch eine **mobile Ansicht** umsetzen. Die Inhalte werden dabei in ein **vertikales Layout** umsortiert, und die Geschichtsauswahl wird über ein Hamburgermenu zugänglich.

## Rückblick

Das Projekt **„Wassererbe Augsburg“** war eine spannende Erfahrung, die mir viel über den Umgang mit Daten und die Verbindung von Technologie und Geschichte beigebracht hat. Besonders der modulare Aufbau und die Entkopplung von Daten und Visualisierung haben uns ermöglicht, flexibel zu bleiben, das Projekt zukunftssicher zu gestalten und vor allem entkoppelt zu zweit in einem Team daran zu arbeiten.
