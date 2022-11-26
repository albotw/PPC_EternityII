eternity: main.o dynamicMemoryManager.o utils.o
	gcc -Wall -o eternity main.o dynamicMemoryManager.o utils.o

main.o: ../main.c
	gcc -Wall -c ../main.c

dynamicMemoryManager.o: ../libs/dynamicMemoryManager.c
	gcc -Wall -c ../libs/dynamicMemoryManager.c

utils.o: ../utils.c
	gcc -Wall -c ../utils.c

clean:
	rm -f eternity *.o
