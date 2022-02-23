SOURCES=$(shell ls *.v)
EXECUTABLES = $(addprefix bin/,$(SOURCES:.v=))
$(shell mkdir -p bin)
all: $(EXECUTABLES)
bin/% : %.v
	v -cg -o $@ $<
clean:
	rm -rf bin/

