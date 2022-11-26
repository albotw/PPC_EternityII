#include "utils.h"

void println()
{
    printf("\n");
}

void displayArray(void* array, int size, int type)
{
    void* ptr = array;
    for (int i = 0; i < size; i++)
    {
        switch(type)
        {
            case 1:
                printf("%d", *(int *)ptr);
                break;
            case 2:
                printf("%f", *(float *)ptr);
                break;
            case 3:
                printf("%c", *(char *)ptr);
                break;
            case 4:
                printf("%ld", *(long *)ptr);
                break;
        }
        ptr++;
    }
    println();
}

void printArraySize(void* array)
{
    printf("%ld\n", strlen(array));
}

void updateRandomSeed()
{
    srand(time(NULL));
}

int RandomizedInt(int a, int b)
{
    b++;
    return (rand() % (b - a)) + a;
}