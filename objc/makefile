setup:
	mkdir ./bin

clean:
	rm -rf ./bin

hello_world: clean setup
	clang -framework Foundation hello_world.m -o ./bin/hello_world

autorelease_usage: clean setup
	clang -framework Foundation autorelease_usage.m -o ./bin/autorelease_usage
