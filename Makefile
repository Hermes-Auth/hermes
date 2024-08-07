CFLAGS := $(shell pkg-config --cflags gtk+-3.0)
LIBS := $(shell pkg-config --libs gtk+-3.0)

all:
	gcc $(CFLAGS) -o hello-world-gtk main.c $(LIBS)
