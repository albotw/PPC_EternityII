# projet_info4A_2019_2020
### projet réalisé avec Eclipse, compilé avec gcc

Codage d'un jeu de puzzle en C dans le style d'eternity II

## Description
C'est un jeu de puzzle composé de pièces carrées avec un motif sur chaque coté. les pièces peuvent êtres déplacées et tournées sur elles mêmes.

l'objectif est de placer/tourner ces pièces de façon à ce que les côtés contigus de deux pièces aient le même motif. On considère que les bords sont liés entre eux.

Le plateau de jeu est disponibles en différentes tailles: 4x4(16 pièces), 5x5(25 pièces), 6x6(36 pièces) voire 7x7(56 pièces).

## Variantes
  On peut faire deux variantes, une simple et une autre plus complexe.

* ### Puzzle simple:
  * les pièces sont bien orientées mais pas à la bonne place. les positions initiales sont aléatoires. solution possible sans rotation.

  * Commande de déplacement: b2c4 --> échange les pièces en b2 et c4

  * Le programme doit indiquer le nombre de conflits, sinon affiche "Solution trouvée" ainsi que le nombre de coups.
  
* ### Puzzle complexe:
  * Dans cette version les pièces sont mal orientées ET mal placées. 
  
  * Commande de rotation: c3 --> la pièce en c3 tourne sur elle même dans le sens des aiguilles d'une montre.
  
  * Sinon, fonctionnement identique au puzzle simple.
  
## IA
Une petite IA peut être codée permettant de trouver la solution automatiquement. On peut (doit) utiliser le principe de la recherche locale qui consiste à essayer de réaliser des modifications(échange/rotation) permettant de réduire progressivement le nombre de conflits.

Il faut tout de même prévoir un hasard et permettre de faire des modifications augmentant le nombre de conflits afin d'éviter tout blocage potentiel.

## Contraintes
Logiquement, le code doit être fonctionnel, compilable et éxécutable. Tout le code doit être dans un unique fichier source. Faire attention à l'indentation et utiliser des commentaires pour les sections critiques.

il est aussi demandé de faire un mini dossier (5 pages max EN LATEX) qui détaille les structures de données utilisées.

#### DATE DE LIVRAISON MAX: MERCREDI 15 AVRIL 2020.
