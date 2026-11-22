Faire communiquer les threads avec mpsc

Une fois que le thread secondaire a récupéré la sortie de MATLAB dans une variable result_text, il faut renvoyer ce texte vers self.matlab_output pour l'afficher.

En Rust, deux threads n'ont pas le droit de modifier directement la même variable sans protection, car cela provoquerait des corruptions mémoire (data races). À la place, Rust privilégie le passage de messages (Message Passing) : "Ne communiquez pas en partageant la mémoire ; partagez la mémoire en communiquant."

C'est le rôle du module std::sync::mpsc, qui signifie Multi-Producer, Single-Consumer (Plusieurs producteurs, un seul consommateur) :
Rust

let (tx, rx) = mpsc::channel();

Imagine un tube pneumatique à sens unique :

    tx (Transmitter — l'émetteur) : C'est l'entrée du tube. On le donne au thread secondaire (via move). Quand MATLAB a fini, le thread glisse le résultat dedans avec tx.send(result_text). (On appelle ça Multi-Producer car on pourrait cloner tx pour que plusieurs threads envoient des messages dans le même tube).

    rx (Receiver — le récepteur) : C'est la boîte aux lettres à la sortie du tube. On la garde dans notre structure d'interface (self.rx = Some(rx)). Il n'y a qu'un seul récepteur (Single-Consumer).

4. Lire le résultat sans bloquer et réveiller egui
Pourquoi try_recv() et pas recv() ?

Dans fn ui(), on veut vérifier si un message est arrivé dans rx :

    rx.recv() est bloquant : s'il n'y a pas encore de message, il met le thread en pause en attendant qu'un message arrive. L'utiliser dans fn ui() gèlerait donc l'interface exactement comme avant.

    rx.try_recv() est non-bloquant : il jette un coup d'œil instantané dans la boîte aux lettres et répond tout de suite :

        Ok(output) : un message est arrivé ! On le récupère, on l'enregistre dans self.matlab_output, et on remet self.rx = None (ce qui indique à l'interface que le calcul est terminé et réactive le bouton Run MATLAB).

        Err(...) : la boîte est encore vide (MATLAB travaille toujours). fn ui() ignore et continue de dessiner l'interface normalement.

À quoi sert ctx.request_repaint() ?

Pour économiser le processeur et la batterie, egui est événementiel : si tu ne bouges pas la souris et que tu ne tapes pas au clavier, egui met la fenêtre en veille et n'appelle plus fn ui().

Si MATLAB termine son calcul pendant que ta souris est immobile, le message reste dans rx sans être lu jusqu'à ce que tu bouges la souris.

L'objet egui::Context (ctx) est une poignée légère (un pointeur atomique interne, d'où le .clone() très peu coûteux) que l'on peut envoyer dans n'importe quel thread. Appeler ctx.request_repaint() juste après tx.send(...) envoie un signal au thread principal : "Réveille-toi et exécute fn ui() une fois immédiatement, il y a du nouveau à afficher."
