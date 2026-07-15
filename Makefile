TARGET = NumberPad
SERVICE = numberpad.service

build:
	cargo build --release

run:
	sudo ./target/release/$(TARGET)

install: build
	sudo cp ./target/release/$(TARGET) /usr/local/bin/$(TARGET)
	sudo cp $(SERVICE) /etc/systemd/system/

uninstall:
	sudo systemctl stop $(SERVICE) || true
	sudo systemctl disable $(SERVICE) || true
	sudo rm -f /etc/systemd/system/$(SERVICE)
	sudo rm -f /usr/local/bin/$(TARGET)
	sudo systemctl daemon-reload

clean:
	cargo clean

.PHONY: build run install uninstall clean
