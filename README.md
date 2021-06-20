# cake
A small tool to inspect docker images

Usage -
`$ cake IMAGE`

```shell
$ cake debian:bullseye-slim

Analysis Report:
  Efficiency score: 40 %
  Total size: 76.6 MB
  Wasted Space: 45.2 MB

Inefficient Files:
Count  Wasted Space  File Path

Packages:
APT - apt-get/aptitude
All packages:    122.1 MB
Optional pkgs:    45.2 MB
Cache:              0.0 B (/var/lib/apt/lists/)
```
