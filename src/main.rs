
#[allow(unused_parens)]


pub mod _App{
    use ggez::{Context, ContextBuilder, GameResult};
    use ggez::mint::Point2;
    use ggez::graphics::{self, Color};
    use ggez::event::{self, EventHandler, EventLoop};
    use ggez::graphics::*;
    use ggez::conf::{WindowMode, FullscreenType};

    use std::collections::HashMap;
    use std::str::FromStr;
    use ggez::mint::Vector2;

    use INDA22PlusPlus_antmag_hw3::*;
    use Definitions::*;

    //Used to send a moves from application layer to network layer
    #[derive(Clone, Copy)]
    pub struct Move_channel{
        pub from : (usize, usize),
        pub to : (usize, usize)
    }

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
            //println!("Rendering {} markings!", self.marks.len());
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

    pub struct App{
        RM : Resource_manager,
        WS : Window_settings,
        GM : Graphics_manager,

        prev_click_pos : Option<(usize, usize)>,
        clicked_piece : bool,

        game : Game
    }

    impl App{
        pub fn new(_ctx : &mut Context) -> App{
            let WS : Window_settings = Window_settings{
                width : 1200.0,
                height : 1200.0
            };
            
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

                game : create_game(),
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

        pub fn render_frame(&self, _ctx : &mut Context) -> GameResult<()>{ 
            graphics::clear(_ctx, Color::WHITE);
            self.GM.render_board(_ctx, &self.game, &self.RM);
            self.GM.render_markings(_ctx, &self.RM);
            graphics::present(_ctx)
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

        pub fn make_move(&mut self, from : (usize, usize), to : (usize, usize)){
            self.game.move_from_to(from, to);
        }

        pub fn is_valid_move(&mut self, from : (usize, usize), to : (usize, usize)) -> bool{
            return self.game.is_valid_move(from, to);
        }

    }

    impl /*EventHandler for*/ App{
        fn update(&mut self, _ctx : &mut Context) -> GameResult<()>{
            //println!("Updated!");
            Ok(())
        }

        fn draw(&mut self, _ctx : &mut Context) -> GameResult<()> {
            graphics::clear(_ctx, Color::WHITE);
            self.render_frame(_ctx).unwrap();
            graphics::present(_ctx)
        }

        pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: ggez::event::MouseButton, _x: f32, _y: f32) -> Option<Move_channel>{
            println!("Clicked x: {}, y: {}", _x,_y);
            let cell : (usize, usize) = ((_x / self.WS.width * 8.0).floor() as usize , (_y / self.WS.height * 8.0).floor() as usize);
            println!("{}, {}", cell.0, cell.1);

            let is_piece = self.game.coordinates_playable((cell.0, cell.1));
            let mut return_val = None;

            if let None = self.prev_click_pos {
                if(is_piece) {self.check(cell, (0,0), true);}
                self.prev_click_pos = Some(cell);
            } else{
                let prev_pos = self.prev_click_pos.unwrap();
                //let prev_col = self.game.get_content(prev_pos);

                if(self.check(prev_pos, cell, false)) {
                    println!("Moving a piece!");
                    return_val = Some(Move_channel { from: prev_pos, to: cell})
                    //self.game.move_from_to(prev_pos, cell);
                }else{
                    self.prev_click_pos = None;
                }
                self.GM.marking_wipe();
            }
            return return_val;
        }
    }

}


pub mod Net_app{
    use std::{net::{TcpStream, TcpListener}, io::{Read, Write}, str::FromStr};
    use INDA22PlusPlus_antmag_hw3::net_packet::{self, s2c_message::Msg, Move};
    use INDA22PlusPlus_antmag_hw3::net_packet::*;
    use ggez::{Context, ContextBuilder, GameResult, event::EventHandler, GameError};
    use super::_App::*;

    use crate::{Net_app::c2s_message::Msg::ConnectRequest, _App};
    use INDA22PlusPlus_antmag_hw3::Game;
    use prost::Message;

    use std::io::Cursor;

    type Net_buffer = [u8; 512];

    const SERVER_IP_PORT : &str = "127.0.0.1:1337";
    const SERVER_INIT_STATE : NET_STATE = NET_STATE::WAITING;

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum NET_STATE{
        WAITING = 0,
        MY_TURN = 1,
        NOT_ESTABLISHED = 2
    }

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum NET_TYPE{
        CLIENT = 0,
        SERVER = 1
    }

    pub struct Net_app{
        app : _App::App,

        fen : String,
        typ : NET_TYPE,
        state : NET_STATE,
        pub mv_cache : Option<Move_channel>,
        
        connection : TcpStream,
        established_connection : bool
    }

    impl Net_app{
        pub fn new(ctx : &mut Context) -> Self{
            let (stream, typ) = {
                let mut args = std::env::args();
                args.next();
                let host_or_client = args.next().expect("Expected a --server or --client 'ip' args");
                match host_or_client.as_str() {
                    "--server" => {
                        let listener = TcpListener::bind(SERVER_IP_PORT).unwrap();
                        (listener.incoming().next().unwrap().unwrap(), NET_TYPE::SERVER)
                    },

                    "--client" => {
                        let ip = args.next().expect("Expected an --ip");
                        let stream = TcpStream::connect(ip).expect("Failed to connect to server!");

                        (stream, NET_TYPE::CLIENT)
                    }

                    _ => panic!("Unknown command: {}", host_or_client)
                }
            };
            
            return Net_app{
                app : App::new(ctx),

                fen : String::new(),
                state : if(typ == NET_TYPE::SERVER){SERVER_INIT_STATE} else {NET_STATE::NOT_ESTABLISHED}, 
                typ : typ,
                mv_cache : None,

                connection : stream,
                established_connection : false
            }
        }

        pub fn ConnectRequest(&mut self){
            assert!(self.typ == NET_TYPE::CLIENT, "The server tried to send a ConnectRequest!");

            let new_connect_request = C2sConnectRequest{
                game_id : 0,
                spectate : false
            };
            let msg_construction = c2s_message::Msg::ConnectRequest(new_connect_request);
            let abstracted = C2sMessage {
                msg : Some(msg_construction)
            };
            let buff = abstracted.encode_to_vec();
            self.connection.write(&buff).expect("Could not send a connection request from client!");
        }

        fn on_ConnectAck(&mut self, packet : S2cConnectAck){
            assert!(self.typ == NET_TYPE::CLIENT, "This should only be recieved by a client?");
            if(packet.success){
                println!("Client did successfully connect to the server!");
            }else{
                panic!("The client's connection request was rejected!")
            }

            self.established_connection = true;
            self.state = NET_STATE::WAITING;

            if(packet.client_is_white() == true){
                self.state = NET_STATE::MY_TURN;
            }
        }

        fn on_MoveAck(&mut self, packet : S2cMoveAck){
            assert!(self.typ == NET_TYPE::CLIENT, "This should only be recieved by a client?");
            println!("Getting move confirmation...");
            if(packet.legal == true){
                println!("My move was legal!");
                if let Some(mv_channel) = self.mv_cache{
                    self.app.make_move(mv_channel.from, mv_channel.to);
                    self.state = NET_STATE::WAITING;
                }else {
                    panic!("Could not complete a move as there was no chached available");
                }
            }else{
                self.state = NET_STATE::MY_TURN;
                println!("The server did not accept my move");
            }
        }

        fn check_connection_server(&mut self){
            assert!(self.typ == NET_TYPE::SERVER, "Server only!");

            let mut buffer: Net_buffer = [0_u8; 512];
            let n = self.connection.read(&mut buffer).expect("Could not properly read the buffer?");

            //Is this even correct?
            let packet = net_packet::C2sMessage::decode(&buffer[..n]);
            let msg = packet.expect("Failed to decode the a packet!");

            match msg.msg {
                None => {},
                Some(p) => {
                    match p {
                        c2s_message::Msg::Move(mv) => {
                            self.on_Move_c2s(mv);
                        },

                        ConnectRequest(cnct_req) => {
                            self.on_ConnectRequest(cnct_req);
                        },
                    }
                }
            };
        }

        fn check_connection_client(&mut self){
            assert!(self.typ == NET_TYPE::CLIENT, "Client only!");

            let mut buffer: Net_buffer = [0_u8; 512];
            let n = self.connection.read(&mut buffer).expect("Could not properly read the buffer?");
            let packet = net_packet::S2cMessage::decode(&buffer[..n]);
            let msg = packet.expect("Client failed to decode a packet!");
            
            match msg.msg {
                None => {},
                Some(p) => {
                    match p {
                        Msg::Move(mv) =>{
                            self.on_Move_s2c(mv);
                        },
                        Msg::ConnectAck (packet) => {
                            self.on_ConnectAck(packet);
                        },
                        Msg::MoveAck (packet) => {
                            self.on_MoveAck(packet);
                        }
                    }
                }
            }

        }
        
        fn on_ConnectRequest(&mut self, cnct_req : C2sConnectRequest){
            assert!(self.typ == NET_TYPE::SERVER, "This is not a server! Do not try to connect to it!");
            println!("Got a connection request!");

            self.response_ConnectRequest();
        }

        fn response_ConnectRequest(&mut self){
            assert!(self.typ == NET_TYPE::SERVER, "The client cannot call ok_ConnectRequest");

            let mut res = S2cConnectAck::default();
            res.client_is_white = Some(SERVER_INIT_STATE == NET_STATE::WAITING);
            res.game_id = Some(0);
            res.success = false;
            res.starting_position = Some(
                BoardState { 
                    fen_string: String::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
                }
            );

            if(self.established_connection == false){
                res.success = true;
                self.established_connection = true;
            }

            let msg = Msg::ConnectAck(res);
            let mut s2c_msg : S2cMessage = S2cMessage { 
                msg: Option::Some(msg)
            };
            let buff = s2c_msg.encode_to_vec();

            self.connection.write(&buff).expect("Could not write to stream!");
        }

        fn on_Move_c2s(&mut self, mv: net_packet::Move){
            assert!(self.typ == NET_TYPE::SERVER, "What!!!???");
            let from_pos = ((mv.from_square%8) as usize, (mv.from_square/8) as usize);
            let to_pos = ((mv.to_square%8) as usize, (mv.to_square/8) as usize);
            
            println!("Recieved move request:
                from row: {}, col: {}
                to row: {}, col: {}", 
            from_pos.1, from_pos.0, to_pos.1, to_pos.0);
            if(self.app.is_valid_move(from_pos, to_pos) == false){
                panic!("Server does not think this move allowed!");
            }else{
                self.app.make_move(from_pos, to_pos);
                self.state = NET_STATE::MY_TURN;
                self.on_Move_c2s_response(true);
            }
        }

        fn on_Move_c2s_response(&mut self, legal : bool){
            let response = S2cMoveAck{
                legal : legal,
                board_result : None //We do not use the fen strings for now
            };
            let response_msg = Msg::MoveAck(response);
            let abstracted = S2cMessage{
                msg : Some(response_msg)
            };
            let buff = abstracted.encode_to_vec();
            self.connection.write(&buff).expect("Could not write to stream!");
        }

        fn on_Move_s2c(&mut self, mv: net_packet::Move){
            let from_pos = ((mv.from_square%8) as usize, (mv.from_square/8) as usize);
            let to_pos = ((mv.to_square%8) as usize, (mv.to_square/8) as usize);
            println!("Recieved move request:
                from row: {}, col: {}
                to row: {}, col: {}", 
            from_pos.1, from_pos.0, to_pos.1, to_pos.0);

            if(self.app.is_valid_move(from_pos, to_pos) == false){
                panic!("Client does not think this is a valid move!");
            }else{
                self.app.make_move(from_pos, to_pos);
                self.state = NET_STATE::MY_TURN;
            }
        }

        fn Move_client_request(&mut self, mv : &Move_channel){
            let mv_formatted = net_packet::Move {
                from_square: (mv.from.0 + mv.from.1 * 8) as u32, //xy
                to_square : (mv.to.0 + mv.to.1 * 8) as u32,
                promotion : None
            };
            println!("Client making a move request");
            self.mv_cache = Some(*mv);
            self.state = NET_STATE::WAITING; //This will make the client listen to the server's response

            let mv_msg = c2s_message::Msg::Move(mv_formatted);
            let mv_final = C2sMessage{
                msg : Some(mv_msg)
            };
            let buff = mv_final.encode_to_vec();
            self.connection.write(&buff).expect("Could not write to stream!");
        }

        fn Move_server_request(&mut self, mv : &Move_channel){
            //The server is assumed to be correct, it does not need client validation
            self.state = NET_STATE::WAITING;
            self.app.make_move(mv.from, mv.to);

            let mv_formatted = net_packet::Move {
                from_square: (mv.from.0 + mv.from.1 * 8) as u32, //xy
                to_square : (mv.to.0 + mv.to.1 * 8) as u32,
                promotion : None
            };
            println!("Server making a move request!");
            self.mv_cache = Some(*mv);
            let mv_msg = s2c_message::Msg::Move(mv_formatted);
            let mv_final = S2cMessage{
                msg : Some(mv_msg)
            };
            let buff = mv_final.encode_to_vec();
            self.connection.write(&buff).expect("Could not write to stream!");            
        }


        pub fn get_net_type(&self) -> NET_TYPE{
            return self.typ;
        }

    }
    
    impl EventHandler for Net_app{
        fn update(&mut self, _ctx : &mut Context) -> GameResult<()>{
            //println!("Updated!");
            
            //self.app.render_frame(_ctx).expect("Something wierd happened!");

            if(self.state == NET_STATE::WAITING || self.established_connection == false){
                if(self.get_net_type() == NET_TYPE::SERVER){
                    self.check_connection_server();
                }else{
                    self.check_connection_client();
                }
            }
            
            Ok(())
        }

        fn draw(&mut self, _ctx : &mut Context) -> GameResult<()>{
            return self.app.render_frame(_ctx);
        }

        fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: ggez::event::MouseButton, _x: f32, _y: f32) {
            let res = self.app.mouse_button_down_event(_ctx, _button, _x, _y);
            if let Some(mc) = res{
                if(self.typ == NET_TYPE::SERVER){
                    self.Move_server_request(&mc);
                }else{
                    self.Move_client_request(&mc);
                }
            }

        }

    }

    /* 
    pub fn from_fen() -> Game{
        
        return ()
    }

    pub fn to_fen(g : &Game) -> String{

        return ()
    }
    */

}


fn proto_build(){
    std::env::set_var("OUT_DIR", "/Users/antonmagnusson/Desktop/_prog/ru/INDA22PlusPlus-antmag-hw3/src");
    std::env::set_var("PROTOC", "/Users/antonmagnusson/Downloads/protoc-21/bin/protoc");
    prost_build::compile_protos(&["chess.proto"], &["src/", "src/chess_networking/"]).unwrap();
}


use INDA22PlusPlus_antmag_hw3::net_packet;
use ggez::{Context, ContextBuilder, GameResult, event::EventLoop, event::{*, self}};
use _App::*;

use crate::Net_app::NET_TYPE;

fn main() {
    println!("Hello, world!");
    //proto_build();
    
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "cool_author")
    .build()
    .expect("Aeeeie!, could not create the ggez context!");

    let mut net_app = Net_app::Net_app::new(&mut ctx);
    if(net_app.get_net_type() == NET_TYPE::CLIENT){
        net_app.ConnectRequest();
    }

    event::run(ctx, event_loop, net_app);
}