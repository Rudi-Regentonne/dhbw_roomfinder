# DHBW Roomfinder

## Beschreibung

Dieses Tool lädt alle Raumkalender der DHBW aus [dhbw.app](https://dhbw.app) herunter und zeigt die aktuell verfügbaren Räume an. So findest du schnell einen freien Raum zum Lernen, Arbeiten oder für Gruppenmeetings.

## Features

- Automatischer Download aller Raumkalender der DHBW
- Anzeige der aktuell freien Räume
- Ausgabe des nächsten verfügbaren Raums
- Optional: Filter nach Datum
- Optional: Start- und Endzeit angeben, um eine Verfügbarkeit in einem Zeitfenster zu prüfen

## Startargumente

Das Programm unterstützt folgende Argumente (siehe auch `--help`):

| Kurz | Lang           | Beschreibung                                                    | Beispiel        |
| ---- | -------------- | --------------------------------------------------------------- | --------------- |
| `-r` | `--room`       | Bevorzugten Raum (z.B. `A244`) angeben                          | `-r A244`       |
| `-f` | `--refetch`    | Kalenderdaten neu herunterladen, auch wenn sie schon existieren | `-f`            |
| `-d` | `--date`       | Datum (TT.MM.JJJJ), für das gesucht werden soll                 | `-d 13.11.2025` |
| `-t` | `--start-time` | Startzeit (HH:MM) des gewünschten Zeitfensters                  | `-t 09:30`      |
| `-e` | `--end-time`   | Endzeit (HH:MM) des gewünschten Zeitfensters                    | `-e 11:15`      |
| `-h` | `--help`       | Zeigt die Hilfe an                                              | `-h`            |
| `-V` | `--version`    | Zeigt die Programmversion an                                    | `-V`            |

Hinweis:

- Wird keine Zeit angegeben, gilt der aktuelle Zeitpunkt für die Verfügbarkeitsprüfung.

## WIP

Das Projekt ist noch in Arbeit und es werden noch mehr Features kommen. Momentan sind nur Termine drin, die einem Kurs zugeordnet sind. Manche Termine sind keinem Kurs zugeordnet und werden somit nicht berücksichtigt.
