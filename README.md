# Exporter rs


This project is a PoC to learn rust. The idea is create a log collector, parse this logs and send to (kafka,loki,postgres).

## ROADMAP todo

- parse the events related with log.file (should be Modify)?
- get paths and foreach all logs files and read content
- parse all lines (using patterns or regex) (format apache, nginx, IIS) also JSON?
- Use a database (like sqlite) to store last line check before close file (create struct)
- Send data to different sources (kafka, loki, postgresql, should be elastic)
- Create configuration file
- Create dockerfile for this app and convert to daemon set for k8s.