# Huskey
## Rust encrypted password manager
### Cryptography used
* [SHA512](https://fr.wikipedia.org/wiki/SHA-2) : for hashing the users password
* [PBDKF2](https://en.wikipedia.org/wiki/PBKDF2): for reducing vulnerability against brute force attacks and calculating the actual encryption key
* [AES-GCM](https://www.cryptosys.net/pki/manpki/pki_aesgcmauthencryption.html) : for encrypting the database file (AES256 used)
### Improvements
- [ ] Protecting the process against low priviledged attackers ? (using the secmem-proc crate)
- [ ] Support for ChaCha20