Projet PCC Eternity II

Recherche Locale Stochastique:

Paysage de recherche qui comprend toutes les configurations possibles
Partir d'une configuration initiale aléatoire et appliquer une fonction de transition vers une autre configuration. Répéter jusqu'a validation.
Avant chaque changement, déterminer le cout de la nouvelle configuration ainsi que de la config actuelle. prendre la nouvelle si <=.

Critère de Métropolis:

P = exp(f(C) - f(V) / T) avec
    f fonction de cout
    C configuration actuelle
    V configuration potentielle
    T constante: Température.

T ~= 0 => transitions défavorables rejetées
T >> TMAX => mode alétaoire
A calibrer expérimentalement (CF recuit simulé)

la fonction cout doit pouvoir prendre en compte des bassins d'attraction ( == se baser sur plusieurs valeurs)
=> Corrélation voisinage-coût et corrélation distance-cout.
=> visualisation du paysage de recherche ??

