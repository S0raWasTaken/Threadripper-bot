debug:
	cargo build; mkdir -p test/; rm -f ./test/bot; mv target/debug/threadripper-bot ./test/bot

release:
	cargo build --release
	mkdir -p bin/
	mv target/release/threadripper-bot ./bin/threadripper-bot

clean:
	mkdir -p test/ bin/
	rm -r test/
	rm -r bin/

test: debug
	cd test/; clear; ./bot

drun: clean debug
	cd test/; clear;./bot
