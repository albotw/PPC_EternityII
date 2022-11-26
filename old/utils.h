#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

void displayArray(void* array, int size, int type);
//? type == 1 ~> int
//? type == 2 ~> float
//? type == 3 ~> char
//? type == 4 ~> long

void println();
//* DONE

void updateRandomSeed();
//* DONE

int RandomizedInt(int a, int b);
///* DONE
//? B > A && return E [a;b]

void printArraySize(void* array);
//* DONE