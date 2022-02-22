setup:
	mkdir ./bin

clean:
	rm -rf ./bin

hello_world: clean setup
	clang -framework Foundation hello_world.m -o ./bin/hello_world

autorelease_usage: clean setup
	clang -framework Foundation autorelease_usage.m -o ./bin/autorelease_usage

app_menu_items: clean setup
	clang -framework Foundation -framework Cocoa app_menu_items.m -o ./bin/app_menu_items
