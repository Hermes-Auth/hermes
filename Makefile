all:
	gcc -Wall -o hermes.out main.c -lraylib -lm -lpthread -ldl -lrt
