#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "libs/dynamicMemoryManager.h"

typedef struct{
    char N;
    char E;
    char S;
    char W;
}piece;

piece* tab;

int cote;   
//? taille du côté du tableau

int nbPieces;


void generateTab(int size, int mode); 
//* ~DONE
// TODO: randomisation des pièces
//? mode 0 = mode simple, mode 1 = mode complexe

void rotate(int posX, int posY);   
//* DONE
//? tourne d'un seul quart de tour a la fois 
// TODO: rotation de plusieurs quarts de tour d'un seul coup

void swap(int x1,int y1, int x2, int y2); 
//* DONE
//! ATTENTION INTERVALLE PART DE 0 

piece getPieceAt(int x, int y); 
//* DONE

void setPieceAt(int x, int y, piece p); 
//* DONE

void draw(); 
//* DONE
//! Fuite mémoire possible avec le retour de formatChar, pas moyen de free les malloc dans draw.

int checkConflicts();   
//* DONE
//! Optimisation possible mais code plus complexe

char generateFaceFromContext(int x, int y, char face);
// TODO:

char* formatChar(char c);   
//* DONE
// TODO: Voir pour l'utilisation de polices unicodes

void AI();      
// TODO:

int choixTailleTableau();
//* DONE
//? Interface

int rejouer();
//* DONE
//? Interface

int selectionMode();