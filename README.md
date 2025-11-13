# DHBW Roomfinder

## Beschreibung

Dieses Tool lädt alle Raumkalender der DHBW aus [dhbw.app](https://dhbw.app) herunter und zeigt die aktuell verfügbaren Räume an. So findest du schnell einen freien Raum zum Lernen, Arbeiten oder für Gruppenmeetings.

## Features

- Automatischer Download aller Raumkalender der DHBW
- Anzeige der aktuell freien Räume
- Ausgabe des nächsten verfügbaren Raums

## Startargumente

Das Programm unterstützt folgende Argumente (siehe auch `--help`):

| Argument          | Beschreibung                                                              | Beispiel        |
| ----------------- | ------------------------------------------------------------------------- | --------------- |
| `-r`, `--room`    | Optional: Deinen bevorzugten Raum im Format z.B. `A244` angeben           | `-r A244`       |
| `-f`, `--refetch` | Optional: Kalenderdaten neu herunterladen, auch wenn sie schon existieren | `-f`            |
| `-d`, `--date`    | Optional: Datum an welchem Tag der Raum gesucht werden soll               | `-d 13.11.2025` |
| `-h`, `--help`    | Zeigt die Hilfe an                                                        | `-h`            |
| `-V`, `--version` | Zeigt die Programmversion an                                              | `-V`            |

## WIP

Das Projekt ist noch in Arbeit und es werden noch mehr Features kommen. Momentan sind nur Termine drin, die in einem Kurs sind. Manche Termine sind keinem Kurs zugeordnet und werden somit nicht berücksichtigt.
