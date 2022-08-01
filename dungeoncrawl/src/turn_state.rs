#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwatingInput,
    PlayerTurn,
    MonsterTurn
}