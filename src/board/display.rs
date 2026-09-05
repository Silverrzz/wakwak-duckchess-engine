use colored::Colorize;
use crate::board::{Board, CastlingDirection};
use crate::common::{Color, File, Rank, Square};

impl Board  {
    #[inline]
    pub fn display(&self, frc: bool) {
        println!("╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗");

        for &rank in Rank::ALL.iter().rev() {
            print!("║");

            for &file in File::ALL {
                let sq = Square::new(file, rank);

                if let Some(piece) = self.piece_on(sq) {
                    let piece: char = piece.into();

                    if self.color_on(sq).unwrap() == Color::White {
                        print!(" {}", String::from(piece.to_ascii_uppercase()).bright_green());
                    } else {
                        print!(" {}", String::from(piece).bright_blue());
                    }
                } else if self.duck == Some(sq) {
                    print!(" {}", String::from("D").bright_yellow());
                } else {
                    print!("  ");
                }

                if file == File::H {
                    print!(" ║");
                } else {
                    print!(" │");
                }
            }

            println!(" {}", rank);
            if rank == Rank::First {
                println!("╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝");
            } else {
                println!("╟───┼───┼───┼───┼───┼───┼───┼───╢");
            }
        }

        for &file in File::ALL {
            print!("  {} ", char::from(file).to_ascii_uppercase());
        }

        println!("\n\n{}: {}", String::from("FEN").bright_green(), self.to_fen(frc));
        println!(
            "{}: {:#016X}",
            String::from("Zobrist Key").bright_green(),
            self.hash()
        );

        let mut castling_rights = String::new();
        if let Some(file) = self.castling_rights[Color::White].get(CastlingDirection::Short) {
            castling_rights.push(if frc {
                char::from(file).to_ascii_uppercase()
            } else {
                'K'
            });
        }
        if let Some(file) = self.castling_rights[Color::White].get(CastlingDirection::Long) {
            castling_rights.push(if frc {
                char::from(file).to_ascii_uppercase()
            } else {
                'Q'
            });
        }
        if let Some(file) = self.castling_rights[Color::Black].get(CastlingDirection::Short) {
            castling_rights.push(if frc {
                char::from(file).to_ascii_lowercase()
            } else {
                'k'
            });
        }
        if let Some(file) = self.castling_rights[Color::Black].get(CastlingDirection::Long) {
            castling_rights.push(if frc {
                char::from(file).to_ascii_lowercase()
            } else {
                'q'
            });
        }

        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        println!(
            "{}: {}",
            String::from("Castling Rights").bright_green(),
            castling_rights
        );

        if let Some(ep) = self.en_passant() {
            println!(
                "{}: {:?}",
                String::from("En Passant").bright_green(),
                Square::new(ep.file(), Rank::Sixth.relative_to(self.stm))
            );
        }

        println!(
            "{}: {:?}",
            String::from("Halfmove Clock").bright_green(),
            self.hmc()
        );
        println!(
            "{}: {:?}",
            String::from("Fullmove Count").bright_green(),
            self.fmc()
        );
        println!(
            "{}: {:?}",
            String::from("Side To Move").bright_green(),
            self.stm()
        );
    }
}