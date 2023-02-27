# Documentation projet :

    Le projet est composé d'un client complet fonctionnel qui communique avec le serveur de référence.
    Un serveur custom a commencé d'être implémente mais il n'est fonctionnel que jusqu'au niveau de l'inscription des joueurs.
    Le projet est divisé en plusieurs modules.

    Le challenge resolu par le client est MD5HashCash.

# Demarche d'élaboration du projet :

    Le projet a ete constituie dans l'ordre suivante :
         - création des structures des données utilisées pour la communication client - server;
         - implementation de trait challenge et la résolution de problèmes MD5HashCash;
         - construction du client en utilisant le serveur de référence, message par message;
         - avant de commencer le serveur custom, une division par modules a été effectué;
         - construction du serveur jusqu'au point de gestion des inscriptions;
         - refactoring, optimisation de gestion d'erreurs (élimination panic!, unwrap(), et warnings de compilation)


# Les bonus :

    - La stratégie employée par le client pour résoudre le challenge est : le client regarde pour sa prochaine cible, le joueur       qui a le moins de points.
    - Le nombre d'unwrap et panic a ete reduit à 0. Comme warning, 1 warning est affiché. Les mut sont réduites au maximum.



