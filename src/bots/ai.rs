mod space_shooter_types;

enum BotTactics {
    Safe,
    Risky,
    Weird,
    TeamPlayer,
}

enum BotInteligence {
    Dumb,
    Normal,
    Smart,
    Cheater,
    YoWhatTheFuck,
}

struct BotShip {
    tactic: BotTactics,
    dificulty: BotInteligence,
    state: GShipState,
}
