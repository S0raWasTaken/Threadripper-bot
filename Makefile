debug:
	cargo build; mkdir -p test/; mv target/debug/threadripper-bot ./test/bot

release:
	cargo build --release
	mkdir -p bin/
	mv target/release/threadripper-bot ./bin/threadripper-bot

clean:
	mkdir -p test/ bin/
	rm -r test/
	rm -r bin/

drun: clean debug
	cd test/; clear;./bot
