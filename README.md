# bsn-gen-rs

#### Genereert een `all_bsns.csv` bestand met daarin alle 90 miljoen mogelijke (valide) BSNs.

## Gebruik

- `cargo run --release`

of:

- `cargo build --release`  
- `./target/release/bsn-gen-rs`

***

## Info

### Waarom?
Gewoon

### Hoe groot is de CSV en hoe lang duurt het om te genereren?
Het bestand is ruim 900 MB.

Omdat dit programma letterlijk 1 miljard getallen door de [elfproef](https://nl.wikipedia.org/wiki/Elfproef#Burgerservicenummer) gooit,
kan het tamelijk lang duren om dit programma te draaien.

Zonder threading duurde het bij mij enkele minuten,
maar nu duurt het met threading op 8 threads ongeveer 20 seconden.
