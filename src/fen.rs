use crate::board::{Piece, PlayerPiece, COLUMNS, ROWS};

fn get_character_for_piece(piece: Piece) -> char {
  match piece {
    Piece::Rook => 'r',
    Piece::Bishop => 'b',
    Piece::Knight => 'n',
    Piece::Queen => 'q',
    Piece::King => 'k',
    Piece::Pawn => 'p',
    _ => ' ',
  }
}

pub fn generate_fen_from_board(board: [[PlayerPiece; 8]; 8], player: usize) {
  let mut fen_string = String::new();
  let mut empty_block_count = 0;
  for i in 0..COLUMNS {
    for j in 0..ROWS {
      let curr = board[i][j];
      match curr.piece_idx {
        Piece::None => empty_block_count += 1,
        _ => {
          if empty_block_count > 0 {
            fen_string.push_str(empty_block_count.to_string().as_str());
            empty_block_count = 0;
          }
          let character = get_character_for_piece(curr.piece_idx);
          fen_string.push(if curr.player == 0 {
            character
          } else {
            character.to_ascii_uppercase()
          });
        }
      }
    }
    if empty_block_count > 0 {
      fen_string.push_str(empty_block_count.to_string().as_str());
      empty_block_count = 0;
    }
    fen_string.push(if i < COLUMNS - 1 { '/' } else { ' ' });
  }
  fen_string.push_str(if player == 0 { "b " } else { "w " });
  fen_string += " - 0 1";

  println!("{fen_string}");
}

// variants
// castling
// load from fen