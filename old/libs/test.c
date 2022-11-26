#include <stdio.h>
#include <stdlib.h>

#include "dynamicMemoryManager.h"

int main()
{
    initDMM();
    for(int i = 0; i < 5; i++)
    {
        void* ptr = _malloc(10000*sizeof(char));
        _free(ptr);
    }
    return 0;
}