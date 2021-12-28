use anchor_lang::prelude::*;
use std::fmt;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tictactoe {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        base_account.games.push(Game { 
            board: vec![None; 9],
            o_player: *user.to_account_info().key,
            x_player: None,
            turn: Player::O,
            winner: None
        });

        Ok(())
    }

    pub fn join_game(ctx: Context<JoinGame>, game_index: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let joiner = &mut ctx.accounts.joiner;
        println!("Received joiner {:?}", joiner);
        let game = match base_account.games.get_mut(game_index as usize) {
            Some(game) => game,
            None => return Err(ErrorCode::GameNotFound.into())
        };

        println!("Received game");

        game.x_player = Some(*joiner.to_account_info().key);

        Ok(())
    }

    pub fn play_move(ctx: Context<PlayMove>, game_index: u64, position: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let player = &mut ctx.accounts.player;
        let game = match base_account.games.get_mut(game_index as usize) {
            Some(game) => game,
            None => return Err(ErrorCode::GameNotFound.into())
        };
        if let Some(_) = game.winner {
            return Err(ErrorCode::GameFinished.into());
        }

        let player_key = *player.to_account_info().key;

        if game.x_player != Some(player_key) && game.o_player != player_key {
            return Err(ErrorCode::PlayerNotInGame.into());
        }
        let piece = match game.x_player == Some(player_key) {
            true => Player::X,
            false => Player::O
        };
        if piece != game.turn {
            return Err(ErrorCode::NotPlayerTurn.into());
        }
        if let Some(_) = game.board[position as usize] {
            return Err(ErrorCode::IllegalMove.into());
        }
        game.turn = match piece {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        game.board[position as usize] = Some(piece);
        if let Some(winner) = compute_winner(&game.board) {
            game.winner = Some(winner);
        }

        Ok(())
    }
}

fn compute_winner(board: &Vec<Option<Player>>) -> Option<Player> {
    let lines = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    for line in lines {
        if board[line[0]] == None {
            continue;
        }
        if board[line[0]] == board[line[1]] && board[line[1]] == board[line[2]] {
            return board[line[0]];
        }
    }

    None
}

struct BoardDisplay<'a>(&'a Option<Player>);

trait CustomBoardDisplay {
    fn display<'a>(&'a self) -> BoardDisplay<'a>;
}

impl CustomBoardDisplay for Option<Player> {
    fn display<'a>(&'a self) -> BoardDisplay<'a> {
        BoardDisplay(self)
    }
}

impl<'a> fmt::Display for BoardDisplay<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref square) => write!(formatter, "{}", square),
            None => write!(formatter, " "),
        }
    }
}


fn print_board(board: &Vec<Option<Player>>) {
    println!("-------");
    println!("|{}|{}|{}|", board[0].display(), board[1].display(), board[2].display());
    println!("-------");
    println!("|{}|{}|{}|", board[3].display(), board[4].display(), board[5].display());
    println!("-------");
    println!("|{}|{}|{}|", board[6].display(), board[7].display(), board[8].display());
    println!("-------");
}

// https://stackoverflow.com/questions/30554203/how-to-print-an-optionboxstruct
// impl std::fmt::Display for Option<Player> {

#[test]
fn test_position() {
    print_board(&vec![
                None, Some(Player::X), None, 
                Some(Player::X), Some(Player::O), None, 
                None, None, None]);
}

#[test]
fn test_winner() {
    let result = compute_winner(&vec![
                None, Some(Player::X), None, 
                Some(Player::X), Some(Player::O), None, 
                None, None, None]);
    assert_eq!(result, None)
}

#[test]
fn test_x_winner() {
    let result = compute_winner(&vec![
                None, Some(Player::X), None, 
                Some(Player::X), Some(Player::X), None, 
                None, Some(Player::X), None]);
    assert_eq!(result, Some(Player::X))
}

#[test]
fn test_o_winner() {
    let result = compute_winner(&vec![
                None, Some(Player::O), None, 
                Some(Player::X), Some(Player::O), None, 
                None, Some(Player::O), None]);
    assert_eq!(result, Some(Player::O))
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub joiner: Signer<'info>,
}

#[derive(Accounts)]
pub struct PlayMove<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
}


#[account]
pub struct BaseAccount {
    pub games: Vec<Game>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Game {
    pub board: Vec<Option<Player>>,
    pub o_player: Pubkey,
    pub x_player: Option<Pubkey>,
    pub turn: Player,
    pub winner: Option<Player>
}

#[derive(Debug, Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum Player {
    X,
    O
}

impl fmt::Display for Player {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(formatter, "X"),
            Player::O => write!(formatter, "O")
        }
    }
}

#[error]
pub enum ErrorCode {
    #[msg("You cannot make that move")]
    IllegalMove,
    #[msg("Game not found")]
    GameNotFound,
    #[msg("Player does not belong to game")]
    PlayerNotInGame,
    #[msg("Not the player's turn")]
    NotPlayerTurn,
    #[msg("Game has finished")]
    GameFinished,
}
