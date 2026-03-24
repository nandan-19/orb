# Orb

Orb is my learning project to understand how real-world file sharing works.
I built it to learn:

- how UDP discovery works
- how thread spawning works
- how TCP handshakes and streaming work
- how shared memory can help avoid race conditions
- how to use a finite state machine in a real project
- how to structure a Rust project
- how to maintain global state with `Arc` and `RwLock`

It has been a great learning experience.
The code is still completely synchronous/blocking for now, and I plan to improve it further in my free time.

On every pushed tag, GitHub Actions builds binaries for Linux, macOS, and Windows.
Check the Releases page to download the binaries.

Orb was designed as a learning foundation for a bigger project, Ouroboros.
I am continuing that journey here:
https://github.com/nandan-19/ouroboros
