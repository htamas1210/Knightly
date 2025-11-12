use uuid::Uuid;

struct Step {
    from: String,
    to: String,
}

enum ServerEvent {
    PlayerJoined(Uuid),                        //tarolja el a kapcsolatot
    PlayerLeft(Uuid),                          //torolje a jatekos a listabol mert kilepett
    PlayerJoinedQeue(Uuid),                    //online jatekra var
    PlayerJoinedMatch(Uuid),                   //player joined a match
    PlayerRequestAvailableSteps(Uuid, String), //string board
    PlayerSteps(Uuid, Step),                   //player moves piece
    CheckWinner(Uuid, String),                 //player asks server if they won
    PlayerIsInCheck(Uuid, String),             //board state
    PlayerOpponentUpdateUI(Uuid, String),      //board state
    PlayerLost(Uuid),
    PlayerReturnToMenu(Uuid),
}
