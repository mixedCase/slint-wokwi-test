# Slint on Wokwi

How to run:

- Install dependencies: flake.nix should contain most of what you need except for:
    - wokwi-server, which you can download from https://github.com/MabezDev/wokwi-server/releases
    - Rust for ESP32-C3, which you can install with the rustup tool the flake makes available:
        - `rustup install nightly`
        - `rustup component add rust-src --toolchain nightly`
        - `rustup target add rsicv32imc-esp-espidf`
- If you don't have Nix, open flake.nix and hunt everything down listed in the "buildInputs" section.
- Create a wokwi project with its diagram.json set just like the one shipped in the root of this repository.
- Export an environment variable `WOKWI_PROJECT_ID` with your project ID set as the value. You can use direnv to export it every time you cd to this repo.
- `./scripts/run-wokwi.sh`, hopefully it should open and run in a new browser page.
