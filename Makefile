BIN ?= blinky-gpio-direct

build:
	AVR_CPU_FREQUENCY_HZ=16000000 cargo build -Z build-std=core --target avr-atmega328p.json --release

flash:
	avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/$(BIN).elf:e

flash-maker-uno:
	avrdude -patmega328p -carduino -P/dev/ttyUSB1 -b115200 -D -Uflash:w:target/avr-atmega328p/release/$(BIN).elf:e

set-nightly:
	rustup override set nightly-2021-01-07 