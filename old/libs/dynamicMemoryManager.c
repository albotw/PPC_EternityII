#include "dynamicMemoryManager.h"

int find(int mode, void* adr) //? utiliser rechercher dichotomique
{
    int output = -1;
    if (mode == 0)
    {
        do
        {
            output++;
        }while(blocks[output] != NULL && output < maxBlocks);
    }
    else if (mode == 1)
    {
        if (adr != NULL){
            int found = 0;
            for(int i = 0; i < nbBlocks; i++)
            {
                void* ptr = blocks[i]->adr;
                if (found == 0 && ptr == adr)
                {
                    output = i;
                    found = 1;
                }
            }
        }
    }
    return output;
}

void* _malloc(unsigned size)
{
    if (dmSize + size <= maxSize && nbBlocks + 1 <= maxBlocks)
    {
        dmSize = dmSize + size;
        nbBlocks++;

        block* b = calloc(1, sizeof(block));
        b->adr = calloc(1, size);
        b->size = size;

        int position = find(0, NULL);
        blocks[position] = b;
        
        if (verbose) printf("Allocated %u bytes @ %p / %d available\n", b->size, b->adr, maxSize - dmSize);

        return b->adr;
    }
    else
    {
        if (verbose) printf("Error: dynamic memory saturated\n");
        
        return NULL;
    }
}

void _free(void* adr)
{
    
    int position = find(1, adr);
    if (position != -1)
    {
        block* b = blocks[position];
        dmSize = dmSize - b->size;
        nbBlocks--;

        if (verbose) printf("Released %u bytes @ %p / %d available\n", b->size, b->adr, maxSize - dmSize);
        
        free(b->adr);
        free(blocks[position]);
        blocks[position] = NULL;
    }
    else
    {
        if (verbose) printf("Error: invalid adress\n");
    }
}

void initDMM(int verboseMode)
{
    verbose = verboseMode;
    blocks = calloc(maxBlocks, sizeof(block));
}
