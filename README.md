# cake
A small tool to inspect docker images

Usage -
`$ cake IMAGE`

```shell
$ cake debian:bullseye-slim

Analysis Report:
  Efficiency score: 40 % (Only an indicative number)
  Total size: 76.6 MB
  Wasted Space: 45.2 MB  (Using size reported by apt)

Inefficient Files:
Count  Wasted Space  File Path
<Files which are duplicate>

Packages:
APT - apt-get/aptitude
All packages:    122.1 MB (Reported by apt)
Optional pkgs:    45.2 MB
Cache:              0.0 B (/var/lib/apt/lists/)
```

Leaves `image.tar` and `image/` artifact in the present working directory, need cleaning manually.
