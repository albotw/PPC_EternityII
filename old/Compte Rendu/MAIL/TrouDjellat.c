/**
 * Projet d'info4A 2020
 * @author Yann TROU et Wassim DJELLAT
 * @version 1.3 (approx. d'apres le dépot git)
 * @see TrouDjellat.pdf
*/

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

//! GESTION MéMOIRE DYNAMIQUE ====================================================================

#define maxSize 50000   //? Taille max en Octets
#define maxBlocks 5000  //? Nombre max de blocs allouables.

/**
 * * structure servant de conteneur à un bloc dans la mémoire dynamique
 * 
 * @param adr adresse du bloc dans la mémoire
 * @param size taille en octets du bloc dans la mémoire.
 */
typedef struct{
    void* adr;
    unsigned size;
}container;

unsigned dmSize;    //? taille actuelle en octets du tas
unsigned nbBlocks;  //? nb de blocs instanciés dans le tas
container** cIndex;   //? table d'indexation de tous les containers existants
//! ATTENTION index existe dans string.h sous macOS ==> erreur de compilation.

int verbose;    //? afficher les retours console ou non.

int find(int mode, void* adr);

void initDMM(int verboseMode);

void* _malloc(unsigned size);

void _free(void* adr);

//! UTILITAIRE =============================================================

void println(); 

void updateRandomSeed();    

int RandomizedInt(int a, int b);

//! MAIN ===================================================================

/**
 * * représentation d'une pièce dans la mémoire
 * 
 * @param N face haut
 * @param E face droite
 * @param S face bas
 * @param W face gauche
 */
typedef struct{
    char N;
    char E;
    char S;
    char W;
}piece;


piece* tab;
//? tableau contenant toutes les pièces.

int cote;   
//? taille du côté du tableau

int nbPieces;
//? nombre de pièces contenues dans le tableau.

void generateTab(int size, int mode); 

void rotate(int posX, int posY);   

void swap(int x1,int y1, int x2, int y2); 

piece getPieceAt(int x, int y); 

void setPieceAt(int x, int y, piece p); 

void draw(); 

int checkConflicts();   

char generateFaceFromContext(int x, int y, char face);

char* formatChar(char c); 

int selectionMode();

int choixTailleTableau();

int rejouer();


//! *****************************************************
//! *               IMPLEMENTATION                      *
//! *****************************************************


//! GESTION TAS ============================================================
/**
 * * recherche dans les blocs de mémoire.
 * ? boucle de parcours de tableau standard avec tests, rien de particulier.
 * ? pour optimiser, utiliser la recherche dichotomique (pas fait).
 * 
 * @param mode type de recherche, 1er bloc libre ou de l'adresse adr
 * @param adr  adresse du bloc recherché 
 * 
 * @return int position de l'objet recherché dans le tableau blocks
 */ 
int find(int mode, void* adr) 
{
    int output = -1;
    if (mode == 0)
    {
        do
        {
            output++;
        }while(cIndex[output] != NULL && output < maxBlocks);
    }
    else if (mode == 1)
    {
        if (adr != NULL){
            int found = 0;
            for(int i = 0; i < nbBlocks; i++)
            {
                void* ptr = cIndex[i]->adr;
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

/**
 * * allocation de bloc dans le tas
 * ? vérifie qu'on ne surcharge pas la taille autorisée dans les #define, puis crée un conteneur, le lie avec le bloc effectif et le place 
 * ? dans le tableau d'indexation.
 * 
 * @param size taille de la zone a allouer
 * 
 * @return pointeur vers la zone allouée.
 */
void* _malloc(unsigned size)
{
    if (dmSize + size <= maxSize && nbBlocks + 1 <= maxBlocks)
    {
        dmSize = dmSize + size;
        nbBlocks++;

        container* c = calloc(1, sizeof(container));
        c->adr = calloc(1, size);
        c->size = size;

        int position = find(0, NULL);
        cIndex[position] = c;
        
        if (verbose) printf("Allocated %u bytes @ %p / %d available\n", c->size, c->adr, maxSize - dmSize);

        return c->adr;
    }
    else
    {
        if (verbose) printf("Error: dynamic memory saturated\n");
        
        return NULL;
    }
}

/**
 * * suppression de bloc mémoire dans le tas
 * ? cherche si le bloc est référencé dans le tableau d'indexation, supprime le bloc effectif et le conteneur 
 * ? puis met a jour les valeurs globales d'utilisation ainsi que le tableau d'indexation.
 * 
 * @param adr adresse du bloc a supprimer.
 */
void _free(void* adr)
{
    
    int position = find(1, adr);
    if (position != -1)
    {
        container* c = cIndex[position];
        dmSize = dmSize - c->size;
        nbBlocks--;

        if (verbose) printf("Released %u bytes @ %p / %d available\n", c->size, c->adr, maxSize - dmSize);
        
        free(c->adr);
        free(cIndex[position]);
        cIndex[position] = NULL;
    }
    else
    {
        if (verbose) printf("Error: invalid adress\n");
    }
}

/**
 * * initialisation de la mémoire dynamique
 * ? crée le tableau d'indexation avec la constante maxBlocks
 * ? et met a jour la variable globale pour la verbose.
 * 
 * @param verboseMode détailler les actions de gestion du tas ou non.
 */
void initDMM(int verboseMode)
{
    verbose = verboseMode;
    cIndex = calloc(maxBlocks, sizeof(container));
}

//! UTILITAIRE ===========================================================

/**
 * * affiche un retour a la ligne dans la console. 
 * ? pourrait être passé en #define
 */
void println()
{
    printf("\n");
}

/**
 * * met a jour la graine de rand a l'aide du temps actuel
 */
void updateRandomSeed()
{
    srand(time(NULL));
}

/**
 * * retourne un entier aléatoire entre [a;b] ou [b;a]
 */
int RandomizedInt(int a, int b)
{
    if (a < b)
    {
        b++;
        return (rand() % (b - a)) + a;
    }
    else
    {
        a++;
        return (rand() % (a - b)) + b;
    }
}

//! MAIN ===================================================================

/**
 * * compte le nombre de conflits dans les pièces du tableau tab
 * ? optimisation probable, on peut faire mieux qu'une double boucle
 * 
 * @return le nombre de conflits entre les pièces du tableau.
 */
int checkConflicts()
{
    int conflits = 0;
    for(int i = 0; i < cote; i++)
    {
        for (int j = 0; j < cote; j++)
        {
            piece p = getPieceAt(j, i);

            if (getPieceAt(j - 1, i).E != p.W)
            {
                conflits++;
            }
            if (getPieceAt(j, i - 1).S != p.N)
            {
                conflits++;
            } 
            if (getPieceAt(j + 1, i).W != p.E){
                conflits++;
            }
            if (getPieceAt(j, i + 1).N != p.S)
            {
                conflits++;
            }
        }
    }
    return conflits / 2;
}

/**
 * * choisit le caractère a donner a la face pour la pièce en cours de création par rapport a ses voisins dans tab, sinon aléatoirement.
 * 
 * @param x abscisse de la pièce en cours de création dans tab
 * @param y ordonnée de la pièce en cours de création dans tab
 * @param face face qui doit être traitée [N, S, E, W]
 * 
 * @return caractère a mettre dans la face 
 */
char generateFaceFromContext(int x, int y, char face)
{
    char temp = '\0';
    switch(face)
    {
        case 'N':
            temp = getPieceAt(x, y-1).S;
            break;

        case 'S':
            temp = getPieceAt(x, y+1).N;
            break;

        case 'E':
            temp = getPieceAt(x+1, y).W;
            break;

        case 'W':
            temp = getPieceAt(x-1, y).E;
            break; 
    }

    if (temp != '\0')
    {
        return temp;
    }
    else 
    {
        return 'A'+RandomizedInt(0, 3);
    }
}

/**
 * * génère un tableau résolu puis le mélange
 * 
 * @param size dimension du coté du tableau a générer
 * @param mode choix entre le mode facile (juste des rotations) et complexe (rotations et échanges)
 */
void generateTab(int size, int mode)
{
    printf("####  Génération du tableau en cours  ####\n");
    
    nbPieces = size * size;
    cote = size;

    tab = (piece* )_malloc(nbPieces * sizeof(piece));
    printf("   Taille: %d x %d, %d pièces, %ld octets en mémoire\n", cote, cote, nbPieces, nbPieces * sizeof(piece));

    for (int i = 0; i < cote; i++)
    {
        for (int j = 0; j < cote; j++)
        {
            piece p =
            {
                generateFaceFromContext(j, i, 'N'),
                generateFaceFromContext(j, i, 'E'),
                generateFaceFromContext(j, i, 'S'),
                generateFaceFromContext(j, i, 'W')
            };
            
            setPieceAt(j, i, p);
        }
    }
    printf("   Tableau résolu généré: %d conflits\n", checkConflicts());

    int nbMoves = RandomizedInt(5, 50);
    int rotations = 0;
    int swaps = 0;

    for (int i = 0; i < nbMoves; i++)
    {
        int x = RandomizedInt(0, cote);
        int y = RandomizedInt(0, cote);

        int action = RandomizedInt(0, mode);

        if (action == 0)
        {
            rotate(x, y);
            rotations++;
        }
        else if (action == 1)
        {
            int x2 = RandomizedInt(0, cote);
            int y2 = RandomizedInt(0, cote);

            swap(x, y, x2, y2);
            swaps++;
        }
    }
    printf("   Fin du mélange: %d mouvements dont %d rotations et %d échanges\n", nbMoves, rotations, swaps);
    printf("####  Fin de la génération du tableau  ####\n");
    println();
}

/**
 * * retourne la pièce contenue dans tab aux coordonnées x,y
 * 
 * @param x abscisse de la pièce a chercher
 * @param y ordonnée de la pièce a chercher
 * 
 * @return pièce trouvée (peut être NULL)
 */
piece getPieceAt(int x, int y)
{
    if (x < 0)
    {
        x = cote -1;
    }
    else if (x > cote - 1)
    {
        x = 0;
    }

    if (y < 0)
    {
        y = cote - 1;
    } 
    else if (y > cote - 1)
    {
        y = 0;
    }
    return tab[(x + (cote * y))];
}

/**
 * * place la pièce passée en paramètre dans tab en [x,y]
 * 
 * @param x abscisse de la position ou placer la pièce
 * @param y ordonnée de la position ou placer la pièce
 */
void setPieceAt(int x, int y, piece p)
{
    tab[(x + (cote * y))] = p;
}

/**
 * * échange deux pièces contenues dans tab
 * ? crée deux pièces temporaires correspondant aux params puis écrase les originales dans tab
 * 
 * @param x1 abscisse de la 1ere pièce
 * @param y1 ordonnée de la 1ere pièce
 * @param x2 abscisse de la 2e pièce
 * @param y2 ordonnée de la 2e pièce
 */
void swap(int x1, int y1, int x2, int y2)
{
    piece temp = getPieceAt(x1, y1);
    piece temp2 = getPieceAt(x2, y2);
    setPieceAt(x1, y1, temp2);
    setPieceAt(x2, y2, temp);    
}

/**
 * * rotation d'une pièce dans tab
 * ? récupère la pièce, crée une nouvelle avec la rotation puis écrase l'ancienne dans tab.
 * 
 * @param x absisse de la pièce a tourner
 * @param y ordonnée de la pièce a tourner
 */
void rotate(int x, int y)
{
    piece origine = getPieceAt(x, y);
    piece new = {origine.W, origine.N, origine.E, origine.S};
    setPieceAt(x, y, new);
}


/**
 * * formate l'affichage de la console pour l'affichage d'un caractère (pour les faces des pièces)
 * ? crée un string dans le tas puis copie le formatage ANSI en fonction de c avant de le retourner
 * 
 * @param c caractère a formater
 * 
 * @return pointeur vers la chaine contenant le formatage
 */
char* formatChar(char c)
{
    char* S = _malloc(25 * sizeof(char));
    switch (c)
    {
        case 'A':
        {
            strcpy(S, "\033[48;5;9m\033[38;5;0mA\033[0m");
            break;
        }
        case 'B':
        {
            strcpy(S, "\033[48;5;13m\033[38;5;0mB\033[0m");
            break;
        }
        case 'C':
        {
            strcpy(S, "\033[48;5;11m\033[38;5;0mC\033[0m");
            break;
        }
        case 'D':
        {
            strcpy(S, "\033[48;5;14m\033[38;5;0mD\033[0m");
            break;
        }
    }

    return S;
}

/**
 * * affiche tab dans la console, avec couleurs et indices.
 * ? traite tab ligne par ligne. pour chaque ligne :
 * ? utilise trois chaines, une pour la ligne contenant les faces N, une pour les faces E et W et une dernière pour les faces S.
 * ? chaque chaine est remplie avec formatage des faces a partir des pièces contenues dans tab
 */
void draw()
{
    for (int i = 0; i < cote; i++)
    {
        //? 30 * coté pour prendre en comptre la taille du formatage
        char top[(30*cote)];
        char mid[(60*cote)];
        char bot[(30*cote)];

        top[0] = '\0';
        mid[0] = '\0';
        bot[0] = '\0';

        //? sprintf au lieu de strcat a cause des %s plus loin.
        sprintf(top+strlen(top), "   ");
        sprintf(mid+strlen(mid), " %d ",i); 
        sprintf(bot+strlen(bot), "   ");

        if (i == 0)
        {
            printf("   ");
        }

        char* c = NULL;
        for (int j = 0; j < cote; j++)
        {   
            if (i == 0)
            {
                printf("  %c   ", 'A' + j);
            }

            piece p = getPieceAt(j, i);

            c = formatChar(p.N);
            sprintf(top + strlen(top), "  %s   ", c);
            _free(c);

            c = formatChar(p.W);
            sprintf(mid + strlen(mid), "%s # ", c);
            _free(c);

            c = formatChar(p.E);
            sprintf(mid + strlen(mid), "%s ", c);
            _free(c);

            c = formatChar(p.S);
            sprintf(bot + strlen(bot), "  %s   ", c);
            _free(c);
        }
        
        if (i == 0){println();}

        printf("%s\n", top);
        printf("%s\n", mid);
        printf("%s\n", bot);
    }
}

/**
 * * lit une commande rentrée par l'utilisateur et l'applique.
 * ? utilise la longueur de la commande pour en déduire le type, la transpose en entiers pour les coordonnées, 
 * ? teste les valeurs et applique la commande, sinon indique une erreur.
 */
void readCommand()
{
    println();
    printf("usage: XY ~> rotation | X1Y1X2Y2 ~> echange | ctrl+c ~> quitter\n");

    char* command = calloc(5, sizeof(char));
    scanf("%s", command);

    if (strlen(command) == 2)
    {
        int x = command[0] - 'A';
        int y = command[1] - '0';
        printf("Rotation: %d|%d == %c|%c\n", x, y, command[0], command[1]);
        if (0 <= x && x <= cote && 0 <= y && y <= cote)
        {
            rotate(x, y);
        }
        else
        {
            printf("Erreur: coordonnées invalides\n");
        }
    }
    else if(strlen(command) == 4)
    {
        int x1 = command[0] - 'A';
        int y1 = command[1] - '0';
        int x2 = command[2] - 'A';
        int y2 = command[3] - '0';

        printf("Swap: %d|%d ~> %d|%d == %c|%c ~> %c|%c\n", x1, x2, y1, y2, command[0], command[1], command[2], command[3]);
        if (0 <= x1 && x1 <= cote && 0 <= y1 && y1 <= cote && 0 <= x2 && x2 <= cote && 0 <= y2 && y2 <= cote)
        {
            swap(x1, y1, x2, y2);
        }
        else
        {
            printf("Erreur: coordonnées invalides \n");
        }
    }
    else
    {
        printf("Erreur: commande non reconnue \n");
    }
    free(command);
}

/**
 * * boucle principale du programme
 * ? appelle la génération de tableau a partir des choix de l'utilisateur, puis tant que le tableau n'est pas résolu, 
 * ? affiche le nombre de conflits et appelle le traitement d'une commande. 
 * ? une fois que le tableau est résolu, propose à l'utilisateur de rejouer
 * 
 * @return 0, fin d'éxécution normale si jamais il n'y a pas de faute de segmentation...
 */
int main()
{
    updateRandomSeed();
    initDMM(0);
    int restart;

    do
    {
        restart = 0;
        int mode = selectionMode();
        int taille = choixTailleTableau();
        generateTab(taille + 3, mode - 1);
        
        int victory;
        do{
            draw();
            victory = checkConflicts();
            println();
            printf("Il y a \033[48;5;9m%d\033[0m conflits restants\n", victory);
            if (victory != 0)
            {
                readCommand();
            }

            println();
        }while(victory != 0);
           
        restart = rejouer();
        if (restart)
        {
            _free(tab);
            tab = NULL;
        }
    }
    while(restart);
    return 0;
}

/**
 * * laisse l'utilisateur choisir le mode de jeu
 * ? affiche les options dans une boucle tant que l'utilisateur n'a pas choisi une valeur correcte.
 * 
 * @return mode choisi
 */
int selectionMode()
{
    int choix;
    int loop = 0;
    do
    {
        loop = 0;
        printf("Quel mode de jeu allez vous choisir ?\n");
        printf("1: mode facile, que des rotations  ");
        printf("2: mode complexe, rotations et échanges  ");
        println();
        scanf("%d", &choix);

        if (1 > choix || choix > 2)
        {
            loop = 1;
            printf("Erreur, option invalide\n");
        }
    } while (loop);
    println();
    return choix;
}

/**
 * * laisse l"utilisateur choisir la taille de tab.
 * ? affiche les options dans une boucle tant que l'utilisateur n'a pas choisi de valeur correcte.
 * 
 * @return taille choisie.
 */
int choixTailleTableau()
{
    int y;
    int loop = 0;
    do
    {
        loop = 0;
        printf("Quelle taille de tableau allez vous prendre ?\n");
        printf("1: 4x4 ");
        printf("2: 5x5 ");
        printf("3: 6x6 ");
        printf("4: 7x7 ");
        println();
        scanf("%d", &y);

        if (1 > y || y > 4)
        {
            loop = 1;
            printf("Erreur: option invalide\n");
        }
    } while (loop);
    println();
    return y;
}


/**
 * * laisse l'utilisateur choisir si il veut rejouer.
 * ? afiche les options puis lis l'entrée utilisateur
 * 
 * @return valeur choisie.
 */
int rejouer()
{
    int temp;
    printf("Rejouer ?\n");
    printf("1: Oui\n");
    printf("2: Non\n");
    scanf("%d", &temp);

    if (temp == 2)
    {
        temp = 0;
    }
    else 
    {
        temp = 1;
    }
    return temp;
}
