
use std::collections::HashMap;

use conf::WindowMode;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Drawable, Image, Mesh, Rect}; // Ensure `graphics` and `Color` are imported correctly
use ggez::{Context, ContextBuilder, GameResult};
use ggez::*;
//use skye_chess::*;
mod temp_mod;
use crate::temp_mod::*;

const SQUARE_SIZE: f32 = 90.0;
const BOARD_SIZE: f32 = SQUARE_SIZE*(8 as f32);


fn match_piece_with_image_key(piece: ChessPiece) -> String{
    match piece.kind {
        ChessPieceKind::Pawn => match piece.colour {
            ChessColour::Black => "black_pawn".to_string(),
            _ => "white_pawn".to_string(),
            
        }
        ChessPieceKind::Rook => match piece.colour {
            ChessColour::Black => "black_rook".to_string(),
            _ => "white_rook".to_string(),
            
        }
        ChessPieceKind::Knight => match piece.colour {
            ChessColour::Black => "black_knight".to_string(),
            _ => "white_knight".to_string(),
            
        }
        ChessPieceKind::Bishop => match piece.colour {
            ChessColour::Black => "black_bishop".to_string(),
            _ => "white_bishop".to_string(),
            
        }
        ChessPieceKind::Queen => match piece.colour {
            ChessColour::Black => "black_queen".to_string(),
            _ => "white_queen".to_string(),
            
        }
        ChessPieceKind::King => match piece.colour {
            ChessColour::Black => "black_king".to_string(),
            _ => "white_king".to_string(),
            
        }
    }
}


fn r_c_to_index_usize(r: usize, c: usize) -> usize {
    r*8 + c

}

fn bitboard_to_positions(bitboard: u64) -> Vec<u32> {
    let mut positions = Vec::new();
    let mut current_bitboard = bitboard;

    while current_bitboard != 0 {
        
        let pos = current_bitboard.trailing_zeros();
        positions.push(pos);

        
        current_bitboard &= current_bitboard - 1;
    }

    positions
}

fn position_to_bitboard(pos: u32) -> u64 {
    if pos > 63 {
        panic!("Invalid position: must be between 0 and 63.");
    }
    1u64 << pos
}



struct ChessGame{
    pub images: HashMap<String, Image>,
    selected_square: Option<(usize, usize)>,
    previous_selected_square: Option<(usize, usize)>,
    just_moved_piece: bool,
    highlighted_squares: Vec<(usize, usize)>,
    is_highlighting: bool,
}

impl ChessGame{

    //GameResult<ChessGame> instead of just ChessGame because of error handling
    //?; instead of just ; because it could be an error (picture not loading for example)
    //It goes up the chain and if it reaches main it will print default error message in console

    fn new(ctx: &mut Context) -> GameResult<ChessGame>{
        let mut images = HashMap::new();
        images.insert("black_pawn".to_string(), Image::from_path(ctx, "/black_pawn.png")?);
        images.insert("white_pawn".to_string(), Image::from_path(ctx, "/white_pawn.png")?);
        images.insert("black_rook".to_string(), Image::from_path(ctx, "/black_rook.png")?);
        images.insert("white_rook".to_string(), Image::from_path(ctx, "/white_rook.png")?);
        images.insert("black_knight".to_string(), Image::from_path(ctx, "/black_knight.png")?);
        images.insert("white_knight".to_string(), Image::from_path(ctx, "/white_knight.png")?);
        images.insert("black_bishop".to_string(), Image::from_path(ctx, "/black_bishop.png")?);
        images.insert("white_bishop".to_string(), Image::from_path(ctx, "/white_bishop.png")?);
        images.insert("black_queen".to_string(), Image::from_path(ctx, "/black_queen.png")?);
        images.insert("white_queen".to_string(), Image::from_path(ctx, "/white_queen.png")?);
        images.insert("black_king".to_string(), Image::from_path(ctx, "/black_king.png")?);
        images.insert("white_king".to_string(), Image::from_path(ctx, "/white_king.png")?);
        images.insert("no_image".to_string(), Image::from_path(ctx, "/no_image.png")?);

        

        let selected_square = None;
        let previous_selected_square = None;
        let just_moved_piece = false;
        let highlighted_squares = vec![];
        let is_highlighting = false;

        Ok(ChessGame {images, selected_square, previous_selected_square, just_moved_piece,
            highlighted_squares, is_highlighting})
    }
}

impl EventHandler for ChessGame {


    //GameResult -> If there was error or not
    //Context -> mouse, sound, timers and so on
    fn update(&mut self, ctx: &mut Context) -> GameResult { 


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        //Initialisera canvas så den kan användas i draw
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        let mut scale_factor = 0.0;

        let mut board = ChessBoard::new(); 
       

        //Highlight squares
        let highlight_color = Color::from_rgb(0, 0, 0);
        let mut highlight: Mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(4.0),
            Rect::new(0 as f32 * SQUARE_SIZE, 0 as f32 * SQUARE_SIZE,
                 SQUARE_SIZE, SQUARE_SIZE),
            highlight_color)?;



        
        let mut image = self.images.get("no_image");

        //Sätta färger för rutor
        let white_square_color = Color::from_rgb(245, 245, 245);
        let green_square_color = Color::from_rgb(143,188,143);

        //Skapa kvadrat
        let mut square = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, 0.0, 0.0),  
            Color::from_rgb(0, 0, 0),
        )?;

        //Börjar rita rutor
        for r in 0..8{
            for c in 0..8{

                //Välja färg
                let mut color = Color::from_rgb(0, 0, 0);
                //Vit om båda jämna eller båda udda
                if (r % 2 == 0 && c % 2 == 0) || (r % 2 != 0 && c % 2 != 0){
                    color = white_square_color;
                }
                else {
                    color = green_square_color;
                }

                //Sätt kvadrat rätt
                square = Mesh::new_rectangle(ctx, DrawMode::fill(), 
                Rect::new(SQUARE_SIZE*(r as f32), SQUARE_SIZE*(c as f32),
                 SQUARE_SIZE, SQUARE_SIZE),
                 color)?;

                //Rita kvadrat
                square.draw(&mut canvas, graphics::DrawParam::default());



                //Rita bild


                let mut image_string = "".to_string();
                let piece = board.get_piece_at(r_c_to_index_usize(r, c)); 
                if piece.is_none() {
                }
                else if !piece.unwrap().is_captured{ 
                    image_string = match_piece_with_image_key(*piece.unwrap()); //Dereference with *
                    image = self.images.get(image_string.as_str());
                    scale_factor = SQUARE_SIZE / image.unwrap().width() as f32;
                    image.unwrap().draw(&mut canvas,graphics::DrawParam::default()
                        .dest([SQUARE_SIZE*(r as f32), SQUARE_SIZE*(c as f32)])
                        //Divides SQUARE_SIZE with width of image so result is maybe 2
                        .scale([scale_factor, scale_factor]));
                }
                else{
                }

                


            }

        }

        //Implementera move piece
        //Om förra sqr var piece och nuvarande inte piece -> move_piece()

        let mut from_c: u64 = 0;
        let mut to_c: u64 = 0;
        if self.previous_selected_square.is_some() && self.selected_square.is_some(){
            from_c = r_c_to_index_usize(self.previous_selected_square.unwrap().0, 
            self.previous_selected_square.unwrap().1) as u64;
            to_c = r_c_to_index_usize(self.selected_square.unwrap().0, 
            self.selected_square.unwrap().1) as u64;
        }


        if board.get_piece_at(from_c as usize).is_some(){
            println!("get is true");
        }
        else{
            println!("get is false");
        }
        if  board.get_piece_at(from_c as usize).is_some() //&& board.move_piece(from_c, to_c)
        && board.simulate_move_piece(from_c, to_c)
        {

            //Assuimng the move_piece is wrong and thats why its not working
            board.move_piece(from_c, to_c);
            println!("moving piece");
        }


        

        //Highlighta valid moves GÖR DETTA!!!
        //if board.get_piece_at(from_c as usize).is_some() 
        //&& self.selected_square != self.previous_selected_square{
        //    for m in bitboard_to_positions(
        //        get_moves(*board.get_piece_at(from_c as usize).unwrap(), board)){
        //            highlight = Mesh::new_rectangle(
        //                ctx,
        //                DrawMode::stroke(4.0),
        //                Rect::new(m as f32 * SQUARE_SIZE, m as f32 * SQUARE_SIZE,
        //                     SQUARE_SIZE, SQUARE_SIZE),
        //                highlight_color)?;
        //            highlight.draw(&mut canvas, graphics::DrawParam::default());
        //    }
        //    
        //}



        //This works
        if self.selected_square == Some((6 as usize, 6 as usize)) {
            highlight = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(4.0),
                Rect::new(6.0 * SQUARE_SIZE, 6.0 as f32 * SQUARE_SIZE,
                     SQUARE_SIZE, SQUARE_SIZE),
                highlight_color)?;
            highlight.draw(&mut canvas, graphics::DrawParam::default());
        }


        //Avluta ritande
        canvas.finish(ctx);
    



        //Mesh används för att rita former

        //DrawParam används för att ge parametrar till former
        //Eller bara default om man vill det


        Ok(())

    }



    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {

        let mut canvas = Canvas::from_frame(_ctx, Color::WHITE);
        

        match _button {
            MouseButton::Left => { 

                self.previous_selected_square = self.selected_square;

                //Get square coordinates instead of mouse coordinates
                self.selected_square = Some(((_x / SQUARE_SIZE )as usize, (_y / SQUARE_SIZE) as usize));
                if self.selected_square == Some((5 as usize, 5 as usize)){
                    println!("Sel_sqr is (5, 5)");

                }
                else if self.previous_selected_square == Some((5 as usize, 5 as usize)) {
                    println!("Prev_sel_sqr is (5, 5)");

                }
                else {
                    println!("None of them are (5, 5)");
                }
            }

            _ => {

            }

        }
        canvas.finish(_ctx);

            

        Ok(())   
    }


}


//GameResult is an alias for Result<(), GameError>
//GameResult<()> means main will return either Ok() or Err(GameError)
fn main() -> GameResult<()>  {
    


    let (mut ctx, event_loop) = 
    ContextBuilder::new("hello_ggez", "awsome_person")
    .window_mode(WindowMode::default().dimensions(BOARD_SIZE, BOARD_SIZE))
    .add_resource_path("./resources")
    .build()
    .unwrap(); //Using context to create new game /event_loop

    let game = ChessGame::new(&mut ctx)?;

    event::run(ctx, event_loop, game); 
}
