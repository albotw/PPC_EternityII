vérification de victoire:
    cas critique: 56 pièces = 56 * 4 cotés / 2 tests simultanés == beaucoup pour pas grand chose

PRESENCE DU POINTEUR TAB IMPLICITE: PAS D'APPEL DANS LES PARAMETRES

A faire:
    Generateur de tableau
    IA
    debogage fuite mémoire draw.
    verification et test
    --> fonction capable de tester / générer une pièce en fonction de ses alentours.

### Notes:

* #### mapping 2D / 1D

  * _i = x + width * y;
  * x = _i % width;
  * y = _i / width;

* ### mapping 3D / 1D

  * _i = x + width * y + width * height * z;
  * x = _i % width;
  * y = (_i / width) % height;
  * z = _i / (width / height);

  n
w # e
  s

  B    A    C    D  
A # CC # DD # BB # A
  D    B    A    C