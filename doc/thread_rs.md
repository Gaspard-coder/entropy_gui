Le problème : le Thread Principal et le blocage

Quand tu lances ton programme, le système d'exploitation crée un fil d'exécution unique appelé le Thread Principal (Main Thread ou UI Thread).

La boucle de rendu d'egui (fn ui(&mut self, ...)) tourne sur ce thread principal. Pour que la fenêtre soit fluide (à 60 images par seconde), chaque exécution de fn ui doit prendre moins de 16 millisecondes. Pendant ce minuscule laps de temps, Rust dessine les boutons, vérifie si la souris a bougé et met à jour le texte.

Or, la méthode .output() de std::process::Command est bloquante :

    Quand Rust exécute .output(), le thread s'arrête complètement et attend que MATLAB démarre, exécute le script, ferme la figure et rende la main.

    Si tu appelles .output() directement dans fn ui, le thread principal est figé pendant 5, 10 ou 30 secondes. egui ne peut plus redessiner la fenêtre ni répondre aux clics : le système d'exploitation affiche alors "L'application ne répond pas".

2. Déléguer le travail avec std::thread::spawn

Pour éviter de figer l'interface, on demande au système d'exploitation de créer un thread secondaire (worker thread) en parallèle :
Rust

thread::spawn(move || {
    // Tout ce qui est ici s'exécute en parallèle sur un autre cœur du processeur
});

Que signifie le mot-clé move ?

La closure (|| { ... }) passée à thread::spawn va continuer à vivre en arrière-plan après que la fonction run_matlab se soit terminée.

En Rust, le compilateur interdit qu'un thread secondaire emprunte des variables locales (&matlab_cmd) qui risquent d'être détruites à la fin de run_matlab. Le mot-clé move force la closure à prendre possession (ownership) des variables qu'elle utilise (matlab_cmd, tx, ctx). Elles sont déplacées à l'intérieur du thread et restent valides jusqu'à ce que MATLAB ait fini.
3. Faire communiquer les threads avec mpsc

Une fois que le thread secondaire a récupéré la sortie de MATLAB dans une variable result_text, il faut renvoyer ce texte vers self.matlab_output pour l'afficher.

En Rust, deux threads n'ont pas le droit de modifier directement la même variable sans protection, car cela provoquerait des corruptions mémoire (data races). À la place, Rust privilégie le passage de messages (Message Passing) : "Ne communiquez pas en partageant la mémoire ; partagez la mémoire en communiquant."

C'est le rôle du module std::sync::mpsc, qui signifie Multi-Producer, Single-Consumer (Plusieurs producteurs, un seul consommateur) :
Rust

let (tx, rx) = mpsc::channel();

Imagine un tube pneumatique à sens unique :

    tx (Transmitter — l'émetteur) : C'est l'entrée du tube. On le donne au thread secondaire (via move). Quand MATLAB a fini, le thread glisse le résultat dedans avec tx.send(result_text). (On appelle ça Multi-Producer car on pourrait cloner tx pour que plusieurs threads envoient des messages dans le même tube).

    rx (Receiver — le récepteur) : C'est la boîte aux lettres à la sortie du tube. On la garde dans notre structure d'interface (self.rx = Some(rx)). Il n'y a qu'un seul récepteur (Single-Consumer).
