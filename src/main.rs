
#[allow(unused_parens)]


pub mod _App{
    use ggez::mint::Point2;
    use ggez::{Context, ContextBuilder, GameResult};
    use ggez::graphics::{self, Color};
    use ggez::event::{self, EventHandler};
    use ggez::graphics::*;
    use ggez::conf::{WindowMode, FullscreenType};

    use std::collections::HashMap;
    use std::str::FromStr;
    use ggez::mint::Vector2;

    use INDA22PlusPlus_antmag_hw3::*;
    use Definitions::*;

    pub struct Tex_resource{
        img : Image,
        path : String,
        key : String,
    }
    pub struct Resource_manager{
        texture_map : HashMap<String, Tex_resource>,
    }

    impl Resource_manager{
        pub fn new(ctx : & Context) -> Resource_manager{
            return Resource_manager{
                texture_map : HashMap::new(),
            };
        }

        pub fn save_tex(&mut self, _ctx : &mut Context, p : &str, key : &str){
            assert!(!self.texture_map.contains_key(key), "Have already added this key!");
            self.texture_map.insert(String::from_str(key).unwrap(), Tex_resource {
                img : Image::new(_ctx, p).expect("Did not manage to load one of the textures!"),
                path : String::from_str(p).unwrap(),
                key : String::from_str(key).unwrap()
            });
        }

        pub fn load_text(&self, key : &str) -> &Image{
            assert!(self.texture_map.contains_key(key), "Had not added this key!");
            return &self.texture_map.get(key).unwrap().img;
        }
    }

    pub struct Graphics_manager{
        marks : Vec<(usize, usize)>
    }
    
    impl Graphics_manager{
        pub fn render_img(&self, _ctx:&mut Context, key : &str, RM : &Resource_manager, pos : (f32, f32), sz : (f32, f32)){
            let img : &Image = RM.load_text(key);
            
            let DP = DrawParam{
                src : Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0},
                color : Color::WHITE,
                trans : Transform::Values {
                    dest: Point2 { x: pos.0, y: pos.1 }, 
                    rotation: 0.0, 
                    scale: Vector2 { x: 1.0/(img.dimensions().w) * sz.0, y: 1.0/(img.dimensions().h) * sz.1}, 
                    offset: Point2 { x: 0.0, y: 0.0}
                }
            };
            graphics::draw(_ctx, img, DP).expect("Failed!");
        }

        pub fn render_piece(&self, _ctx : &mut Context, p : &Piece, pos : (usize, usize), RM : &Resource_manager){
            let key : &str = match p.piece_type {
                PieceType::Pawn => {
                    match p.color {
                        Definitions::Color::Black => "b_pawn",
                        Definitions::Color::White => "w_pawn"
                    }
                },

                PieceType::Knight => {
                    match p.color {
                        Definitions::Color::Black => "b_knight",
                        Definitions::Color::White => "w_knight"
                    }
                },

                PieceType::Bishop => {
                    match p.color {
                        Definitions::Color::Black => "b_bishop",
                        Definitions::Color::White => "w_bishop"
                    }
                },

                PieceType::Rook => {
                    match p.color {
                        Definitions::Color::Black => "b_rook",
                        Definitions::Color::White => "w_rook"
                    }
                },

                PieceType::King => {
                    match p.color {
                        Definitions::Color::Black => "b_king",
                        Definitions::Color::White => "w_king"
                    }
                },

                PieceType::Queen => {
                    match p.color {
                        Definitions::Color::Black => "b_queen",
                        Definitions::Color::White => "w_queen"
                    }
                }
            };

            self.render_img(
                _ctx, key, RM, 
                ((pos.0 as f32)/8.0, (pos.1 as f32)/8.0), 
                (1.0/8.0, 1.0/8.0)
            )
        }

        pub fn render_markings(&self, _ctx : &mut Context, RM : &Resource_manager){
            println!("Rendering {} markings!", self.marks.len());
            for i in 0..self.marks.len(){
                self.render_img(_ctx, "cell_marking", RM, 
                    ((self.marks[i].0 as f32)/8.0, (self.marks[i].1 as f32)/8.0),
                    (1.0/8.0, 1.0/8.0)
                );
            }
        }

        pub fn render_board(&self, _ctx : &mut Context, game : &Game, RM : &Resource_manager){
            self.render_img(_ctx, "chess_grid", &RM, (0.0,0.0), (1.0,1.0));

            for i in 0..8{
                for j in 0..8{
                    let c = game.get_content((i,j));
                    match c {
                        Content::Empty => continue,
                        Content::Occupied(p) => {
                            self.render_piece(_ctx, &p, (i,j), RM);
                        }
                    }
                }
            }
        }

        pub fn add_marking(&mut self, mark : (usize, usize)){
            self.marks.push(mark);
        }

        pub fn marking_wipe(&mut self){
            self.marks.clear();
        }

    }

    pub struct Window_settings{
        pub width : f32,
        pub height : f32
    }

    pub fn new(){
        //Creating the context
        let (mut ctx, event_loop) = ContextBuilder::new("my_game", "cool_author")
        .build()
        .expect("Aeeeie!, could not create the ggez context!");

        let WS : Window_settings = Window_settings{
            width : 1200.0,
            height : 1200.0
        };

        let app = App::new(&mut ctx, WS);
        event::run(ctx, event_loop, app);
    }

    pub struct App{
        RM : Resource_manager,
        WS : Window_settings,
        GM : Graphics_manager,

        prev_click_pos : Option<(usize, usize)>,
        clicked_piece : bool,

        game : Game
    }

    impl App{
        pub fn new(_ctx : &mut Context, WS : Window_settings) -> App{
            graphics::set_mode(_ctx, WindowMode{
                width : WS.width,
                height : WS.height,
                maximized : false,
                fullscreen_type : FullscreenType::Windowed,
                borderless : false,
                min_width : 0.0,
                max_width : 0.0,
                min_height : 0.0,
                max_height : 0.0,
                resizable : false,
                visible : true, 
                resize_on_scale_factor_change : false
            }).expect("Could not configure the window settings!");
            set_screen_coordinates(_ctx, Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 }).expect("Coudl not set ");

            let mut new_app = App {  
                RM : Resource_manager::new(_ctx),
                WS : WS,
                GM : Graphics_manager {
                    marks : vec![]
                },
                prev_click_pos : None,
                clicked_piece : false,

                game : create_game()
            };

            new_app.save_textures(_ctx);
            return new_app;
        }

        pub fn save_textures(&mut self, _ctx : &mut Context){
            let to_load:Vec<(&str, &str)> = vec![
                ("/chess_grid.png", "chess_grid"),
                ("/cell_marking.png", "cell_marking"),

                ("/chess_piece_images/black/bishop.png", "b_bishop"),
                ("/chess_piece_images/black/king.png", "b_king"),
                ("/chess_piece_images/black/knight.png", "b_knight"),
                ("/chess_piece_images/black/pawn.png", "b_pawn"),
                ("/chess_piece_images/black/queen.png", "b_queen"),
                ("/chess_piece_images/black/rook.png", "b_rook"),

                ("/chess_piece_images/white/pawn.png", "w_pawn"),
                ("/chess_piece_images/white/bishop.png", "w_bishop"),
                ("/chess_piece_images/white/knight.png", "w_knight"),
                ("/chess_piece_images/white/rook.png", "w_rook"),
                ("/chess_piece_images/white/king.png", "w_king"),
                ("/chess_piece_images/white/queen.png", "w_queen")
            ];

            for i in 0..to_load.len(){
                self.RM.save_tex(_ctx, to_load[i].0, to_load[i].1);
            }
        }

        pub fn render(&self, _ctx : &mut Context){
            self.GM.render_board(_ctx, &self.game, &self.RM);
            self.GM.render_markings(_ctx, &self.RM);
        }

        pub fn set_clicked_piece(&mut self, val : bool){
            self.clicked_piece = val;
        }

        pub fn check(&mut self, from : (usize, usize), to : (usize, usize), fill : bool) -> bool{
            let d : Destinations = self.game.get_destinations(from);
            match d {
                Destinations::None => return false, 
                Destinations::Exists(arr) => {
                    for pos in arr{
                        if(fill == true) {self.GM.add_marking(pos)};
                        if(pos == to) {return true;}
                    }
                    return false;
                }
            }
        }

    }

    impl EventHandler for App{
        fn update(&mut self, _ctx : &mut Context) -> GameResult<()>{
            //println!("Updated!");
            Ok(())
        }

        fn draw(&mut self, _ctx : &mut Context) -> GameResult<()> {
            graphics::clear(_ctx, Color::WHITE);
            
            self.render(_ctx);
            graphics::present(_ctx)
        }

        fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: ggez::event::MouseButton, _x: f32, _y: f32) {
            println!("Clicked x: {}, y: {}", _x,_y);
            let cell : (usize, usize) = ((_x / self.WS.width * 8.0).floor() as usize , (_y / self.WS.height * 8.0).floor() as usize);
            println!("{}, {}", cell.0, cell.1);

            let is_piece = self.game.coordinates_playable((cell.0, cell.1));
            if let None = self.prev_click_pos {
                if(is_piece) {self.check(cell, (0,0), true);}
                self.prev_click_pos = Some(cell);
            } else{
                let prev_pos = self.prev_click_pos.unwrap();
                //let prev_col = self.game.get_content(prev_pos);

                if(self.check(prev_pos, cell, false)) {
                    println!("Moving a piece!");
                    self.game.move_from_to(prev_pos, cell);
                }else{
                    self.prev_click_pos = None;
                }
                self.GM.marking_wipe();
            }
        }
    }

}

pub mod _Net{
    use std::{net::TcpStream, io::Read};
    use INDA22PlusPlus_antmag_hw3::Game;
    use prost::Message;
    type Net_buffer = [u8; 128];

    enum NET_STATE{
        WAITING = 0,
        MY_TURN = 1
    }

    enum NET_TYPE{
        CLIENT = 0,
        SERVER = 1
    }

    struct Net{
        fen : String,

        typ : NET_TYPE,
        state : NET_STATE,

        connection : TcpStream
    }

    impl Net{

        fn new(){

        }

        fn check_connection(&self) -> Vec<u8>{
            let mut buffer : Vec<u8> = vec![];
            self.connection.read(&mut buffer).expect("Could not properly read the buffer?");
            let t : Box<dyn Message> =  Message::decode(buffer.as_mut()).unwrap();
            return buffer;
        }

        

    }

    pub fn from_fen() -> Game{
        
        return ()
    }

    pub fn to_fen(g : &Game) -> String{

        return ()
    }

}


fn proto_build(){
    std::env::set_var("OUT_DIR", "/Users/antonmagnusson/Desktop/_prog/ru/INDA22PlusPlus-antmag-hw3/src");
    std::env::set_var("PROTOC", "/Users/antonmagnusson/Downloads/protoc-21/bin/protoc");
    prost_build::compile_protos(&["chess.proto"], &["src/", "src/chess_networking/"]).unwrap();
}


fn main() {
    println!("Hello, world!");
    

    _App::new();
}