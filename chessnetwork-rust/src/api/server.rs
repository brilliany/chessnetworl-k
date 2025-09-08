use std::collections::hash_map::{Values, ValuesMut};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http, cookie};
use config::Config;
use std::collections::HashMap;
use lazy_static::lazy_static;
use chrono::Utc;
use cookie::Cookie;
use std::sync::Mutex;
use std::{thread, time};

use crate::engine::Engine;
use crate::{BLACK, get_config_value, WHITE};
use crate::movegenerator::{get_bishop_moves, get_king_moves, get_knight_moves, get_pawn_moves, get_queen_moves, get_rook_moves};
use crate::r#move::Move;
use crate::chessboard::Chessboard;
const INVALID_MOVE_VAL: u8 = 8;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, Session>> = Mutex::new(HashMap::new());
}

pub(crate) async fn setup_single_http_server(config: Config) -> std::io::Result<()> {
    let addr = get_config_value(&config, "http_server_address");
    let port = get_config_value(&config, "http_server_port");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(styles)
            .service(play_engine)
            .service(chessboard_js)
            .service(populate_board)
            .service(possible_moves)
            .service(move_piece)
            .service(make_engine_move)
            .service(get_session_object)
            .service(engine_icon)
            .service(default_icon)
            //piece images
            .service(get_white_pawn_png)
            .service(get_white_knight_png)
            .service(get_white_bishop_png)
            .service(get_white_rook_png)
            .service(get_white_queen_png)
            .service(get_white_king_png)
            .service(get_black_pawn_png)
            .service(get_black_knight_png)
            .service(get_black_bishop_png)
            .service(get_black_rook_png)
            .service(get_black_queen_png)
            .service(get_black_king_png)

            .route("/invalid-request", web::get().to(invalid_req))
    })
        .bind(format!("{}:{}", addr, port))?
        .run()
        .await
}

#[actix_web::main]
pub(crate) async fn main(config: Config) -> std::io::Result<()> {
    let mode = get_config_value(&config, "mode");
    init_sessions();
    match mode.as_str() {
        "HttpServerContinuous" => {
            setup_continuous_http_server();
        }
        "HttpServerSingle" => {
            setup_single_http_server(config).await?;
        }
        _ => {
            println!("Invalid mode");
        }
    }
    Ok(())
}



pub(crate) fn setup_continuous_http_server() {
    // implementation
}

#[get("/")]
async fn hello() -> impl Responder {
    //respond with the web/welcome.html file
    let path = format!("web/welcome.html");
    let file_contents = read_file(&path);
    HttpResponse::Ok().content_type("text/html").body(file_contents)
}

#[get("/css/style.css")]
async fn styles() -> impl Responder {
    //respond with the web/css/style.css file
    let path = format!("web/css/style.css");
    let file_contents = read_file(&path);
    HttpResponse::Ok().content_type("text/css").body(file_contents)
}

#[get("/play-engine")]
async fn play_engine(req: HttpRequest) -> impl Responder {
    return if let Some(session_cookie) = req.cookie("session_id") {
        println!("Session cookie found: {}", session_cookie.value());
        //respond with the web/play-engine.html file
        let path = format!("web/play-engine.html");
        let file_contents = read_file(&path);
        HttpResponse::Ok().content_type("text/html").body(file_contents)
    } else {
        println!("No session cookie found");
        //create new session
        let session = create_new_session();
        //respond with the web/play-engine.html file and set the session_id cookie
        let path = format!("web/play-engine.html");
        let file_contents = read_file(&path);
        let mut response = HttpResponse::Ok().content_type("text/html").body(file_contents);
        response.add_cookie(
            &Cookie::build("session_id", session)
                .path("/")
                .secure(false)
                .http_only(false)
                .finish()
        ).expect("Failed to set cookie");
        response
    }
}

//get js/ts files
/*
#[get("/js/play-engine.js")]
async fn play_engine_js() -> impl Responder {
    //respond with the web/js/play-engine.js file
    let path = format!("web/js/play-engine.js");
    let file_contents = read_file(&path);
    HttpResponse::Ok().content_type("text/javascript").body(file_contents)
}
*/

#[get("/js/chessboard.js")]
async fn chessboard_js() -> impl Responder {
    //respond with the web/js/chessboard.js file
    let path = format!("web/js/chessboard.js");
    let file_contents = read_file(&path);
    HttpResponse::Ok().content_type("text/javascript").body(file_contents)
}

/*
#[get("/js/web-utils.js")]
async fn web_utils_js() -> impl Responder {
    //respond with the web/js/chessboard.js file
    let path = format!("web/js/web-utils.js");
    let file_contents = read_file(&path);
    HttpResponse::Ok().content_type("script/javascript").body(file_contents)
}
*/

//api operations
#[get("/api/populate-board")]
async fn populate_board(req: HttpRequest) -> impl Responder {
    //get session_id from cookie
    let session = check_session(&req);
    //return board state
    let mut board = session.clone().board;
    println!("Currently there are {} sessions", get_session_count());
    println!("Sending board state: for session: {}", session.id);
    board.print_board();
    let json = board.convert_to_json();
    HttpResponse::Ok().json(json)
}

fn read_file(path: &String) -> String {
    let file_contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            String::from("Error reading file")
        }
    };
    file_contents
}
//all piece images
#[get("/pieces/white_pawn.png")]
async fn get_white_pawn_png() -> impl Responder {
    //respond with the web/pieces/white_pawn.png file
    let path = format!("web/pieces/white_pawn.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/white_knight.png")]
async fn get_white_knight_png() -> impl Responder {
    //respond with the web/pieces/white_knight.png file
    let path = format!("web/pieces/white_knight.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/white_bishop.png")]
async fn get_white_bishop_png() -> impl Responder {
    //respond with the web/pieces/white_bishop.png file
    let path = format!("web/pieces/white_bishop.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/white_rook.png")]
async fn get_white_rook_png() -> impl Responder {
    //respond with the web/pieces/white_rook.png file
    let path = format!("web/pieces/white_rook.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/white_queen.png")]
async fn get_white_queen_png() -> impl Responder {
    let path = format!("web/pieces/white_queen.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/white_king.png")]
async fn get_white_king_png() -> impl Responder {
    let path = format!("web/pieces/white_king.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_pawn.png")]
async fn get_black_pawn_png() -> impl Responder {
    //respond with the web/pieces/black_pawn.png file
    let path = format!("web/pieces/black_pawn.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_knight.png")]
async fn get_black_knight_png() -> impl Responder {
    //respond with the web/pieces/black_knight.png file
    let path = format!("web/pieces/black_knight.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_bishop.png")]
async fn get_black_bishop_png() -> impl Responder {
    //respond with the web/pieces/black_bishop.png file
    let path = format!("web/pieces/black_bishop.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_rook.png")]
async fn get_black_rook_png() -> impl Responder {
    //respond with the web/pieces/black_rook.png file
    let path = format!("web/pieces/black_rook.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_queen.png")]
async fn get_black_queen_png() -> impl Responder {
    let path = format!("web/pieces/black_queen.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}
#[get("/pieces/black_king.png")]
async fn get_black_king_png() -> impl Responder {
    let path = format!("web/pieces/black_king.png");
    let file_contents = get_image(&path);
    HttpResponse::Ok().content_type("image/png").body(file_contents)
}

#[get("/api/possible-moves")]
async fn possible_moves(req: HttpRequest) -> impl Responder {
    /* request:
    fetch("/api/get-possible-moves", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
            body: {
                "session_id": sessionId,
                "x": squareElement.cellIndex,
                "y": squareElement.parentElement.rowIndex
                "piece": pieceElement.id
            }
     */
    let session = check_session(&req);

    let queries = req.query_string();
    let queries = queries.split("&");
    let mut x = INVALID_MOVE_VAL;
    let mut y= INVALID_MOVE_VAL;
    let mut piece: String = "".to_string();
    for query in queries {
        let query = query.split("=");
        let mut query = query.into_iter();
        let key = query.next().unwrap();
        let value = query.next().unwrap();
        match key {
            "x" => {
                x = value.parse::<u8>().unwrap();
            }
            "y" => {
                y = value.parse::<u8>().unwrap();
            }
            "piece" => {
                piece = value.to_string();
            }
            _ => {
                println!("Invalid query key: {}", key);
                return invalid_req().await;
            }
        }
    }

    let mut possible_moves: Vec<Move> = Vec::new();
    let mut new_moves: Vec<Move> = Vec::new();
    let chessboard = &session.board;
    match piece.as_str() {
        "white_pawn" => {
            get_pawn_moves(chessboard, WHITE, &mut possible_moves);
            for pawnmove in possible_moves.iter() {
                if pawnmove.get_from_x() == x && pawnmove.get_from_y() == y {
                    println!("Adding move: {}{}{}{}", pawnmove.get_from_x(), pawnmove.get_from_y(), pawnmove.get_to_x(), pawnmove.get_to_y());
                    new_moves.push(*pawnmove);
                }
            }
        }
        "white_knight" => {
            get_knight_moves(chessboard, WHITE, &mut possible_moves);
            for knightmove in possible_moves.iter() {
                if knightmove.get_from_x() == x && knightmove.get_from_y() == y {
                    new_moves.push(*knightmove);
                }
            }
        }
        "white_bishop" => {
            get_bishop_moves(chessboard, WHITE, &mut possible_moves, None);
            for bishopmove in possible_moves.iter() {
                if bishopmove.get_from_x() == x && bishopmove.get_from_y() == y {
                    new_moves.push(*bishopmove);
                }
            }
        }
        "white_rook" => {
            get_rook_moves(chessboard, WHITE, &mut possible_moves, None);
            for rookmove in possible_moves.iter() {
                if rookmove.get_from_x() == x && rookmove.get_from_y() == y {
                    new_moves.push(*rookmove);
                }
            }
        }
        "white_queen" => {
            get_queen_moves(chessboard, WHITE, &mut possible_moves);
            for queenmove in possible_moves.iter() {
                if queenmove.get_from_x() == x && queenmove.get_from_y() == y {
                    new_moves.push(*queenmove);
                }
            }
        }
        "white_king" => {
            get_king_moves(chessboard, WHITE, &mut possible_moves);
            for kingmove in possible_moves.iter() {
                if kingmove.get_from_x() == x && kingmove.get_from_y() == y {
                    new_moves.push(*kingmove);
                }
            }
        }
        "black_pawn" => {
            get_pawn_moves(chessboard, BLACK, &mut possible_moves);
            for pawnmove in possible_moves.iter() {
                if pawnmove.get_from_x() == x && pawnmove.get_from_y() == y {
                    new_moves.push(*pawnmove);
                }
            }
        }
        "black_knight" => {
            get_knight_moves(chessboard, BLACK, &mut possible_moves);
            for knightmove in possible_moves.iter() {
                if knightmove.get_from_x() == x && knightmove.get_from_y() == y {
                    new_moves.push(*knightmove);
                }
            }
        }
        "black_bishop" => {
            get_bishop_moves(chessboard, BLACK, &mut possible_moves, None);
            for bishopmove in possible_moves.iter() {
                if bishopmove.get_from_x() == x && bishopmove.get_from_y() == y {
                    new_moves.push(*bishopmove);
                }
            }
        }
        "black_rook" => {
            get_rook_moves(chessboard, BLACK, &mut possible_moves, None);
            for rookmove in possible_moves.iter() {
                if rookmove.get_from_x() == x && rookmove.get_from_y() == y {
                    new_moves.push(*rookmove);
                }
            }
        }
        "black_queen" => {
            get_queen_moves(chessboard, BLACK, &mut possible_moves);
            for queenmove in possible_moves.iter() {
                if queenmove.get_from_x() == x && queenmove.get_from_y() == y {
                    new_moves.push(*queenmove);
                }
            }
        }
        "black_king" => {
            get_king_moves(chessboard, BLACK, &mut possible_moves);
            for kingmove in possible_moves.iter() {
                if kingmove.get_from_x() == x && kingmove.get_from_y() == y {
                    new_moves.push(*kingmove);
                }
            }
        }
        _ => {
            println!("Invalid piece");
        }
    };
    HttpResponse::Ok().json(new_moves)
}
#[get("/other/engine-icon.png")]
async fn engine_icon() -> impl Responder {
    let path = format!("web/other/engine-icon.png");
    let image = get_image(&path);
    HttpResponse::Ok()
        .content_type("image/png")
        .body(image)
}
#[get("/other/default-icon.png")]
async fn default_icon() -> impl Responder {
    let path = format!("web/other/default-icon.png");
    let image = get_image(&path);
    HttpResponse::Ok()
        .content_type("image/png")
        .body(image)
}

#[post("/move-piece")]
async fn move_piece(req: HttpRequest) -> impl Responder {
    //    let queries = [
    //         ["from_x", x],
    //         ["from_y", y],
    //         ["to_x", toX],
    //         ["to_y", toY],
    //         ["piece", square],
    //     ]
    let session = check_session(&req);

    let queries = req.query_string();
    let queries = queries.split("&");
    let mut from_x= INVALID_MOVE_VAL;
    let mut from_y= INVALID_MOVE_VAL;
    let mut to_x= INVALID_MOVE_VAL;
    let mut to_y= INVALID_MOVE_VAL;
    let mut piece = String::new();
    for query in queries {
        let query = query.split("=");
        let mut query = query.into_iter();
        let key = query.next().unwrap();
        let value = query.next().unwrap();
        match key {
            "from_x" => {
                from_x = value.parse::<u8>().unwrap();
            }
            "from_y" => {
                from_y = value.parse::<u8>().unwrap();
            }
            "to_x" => {
                to_x = value.parse::<u8>().unwrap();
            }
            "to_y" => {
                to_y = value.parse::<u8>().unwrap();
            }
            "piece" => {
                piece = value.to_string();
            }
            _ => {
                println!("Invalid key");
            }
        }
    }
    println!("From x: {}, From y: {}, To x: {}, To y: {}, Piece: {}", from_x, from_y, to_x, to_y, piece);
    session.make_move(Move::new(from_x, from_y, to_x, to_y));

    println!("Currently there are {} sessions", get_session_count());
    println!("Session: {} board now looks like this:", session.get_id());
    session.get_board_state().print_board();

    HttpResponse::Ok()
}

#[get("/api/make-engine-move")]
async fn make_engine_move(req: HttpRequest) -> impl Responder {
    let session = check_session(&req);
    let mut board = session.get_board_state();
    //for now just make a new_single engine and ask for a move
    //freeze for 5 seconds for debugging
    thread::sleep(time::Duration::from_secs(5));
    let mut engine = Engine::new_single(6, BLACK);
    let engine_move = engine.get_best_move(&mut board).unwrap();
    println!("Engine move: {}, {}, {}, {}", engine_move.get_from_x(), engine_move.get_from_y(), engine_move.get_to_x(), engine_move.get_to_y());
    session.make_move(engine_move);
    HttpResponse::Ok().json(engine_move)
}
#[get("/api/get-session")]
async fn get_session_object(req: HttpRequest) -> impl Responder {
    let session = check_session(&req);
    let board = session.get_board_state().convert_to_json();
    let turn = session.get_turn();
    let user_color = session.get_user_color();
    let opponent = if user_color == WHITE {
        BLACK
    } else {
        WHITE
    };
    let session_id = session.get_id();

    let session_object = String::new() + "{\"board\":" + &board + ",\"turn\":" + &turn.to_string() + ",\"opponent\":\"" + &opponent.to_string() +  "\",\"user\":\"" + &user_color.to_string() + "\",\"session_id\":\"" + &session_id + "\"}";
    HttpResponse::Ok().json(session_object)
}


fn get_image(path: &String) -> Vec<u8> {
    let file_contents = match std::fs::read(&path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            Vec::new()
        }
    };
    file_contents
}

async fn invalid_req() -> HttpResponse {
   //send invalid_request.html
    let path = format!("web/invalid_request.html");
    let file_contents = read_file(&path);
    HttpResponse::BadRequest().content_type("text/html").body(file_contents)
}



/**
 * Session struct
    * id: String (current time) - identifier for the session, game can be saved with this id
    * board: Chessboard - responsible for storing the state of the board
    * turn: i8 - 1 for white, -1 for black
    * opponent: String - either "human" or "engine"
*/
#[derive(Clone)]
struct Session {
    id: String,
    board: Chessboard,
    turn: i8,
    user_color: i8,
    opponent_color: i8,
}

impl Session {
    fn new(id: String, board: Chessboard, turn: i8, user_color: i8, opponent_color: i8) -> Session {
        Session {
            id,
            board,
            turn,
            user_color,
            opponent_color,
        }
    }
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_board_state(&self) -> Chessboard {
        self.board.clone()
    }
    fn get_turn(&self) -> i8 {
        self.turn
    }
    fn get_opponent_color(&self) -> i8 {
        self.opponent_color
    }
    fn get_user_color(&self) -> i8 {
        self.user_color
    }
    fn make_move(&mut self, mv: Move) {
        self.board.make_move(mv);
        self.turn *= -1;
    }
}

static mut GLOBAL_SESSIONS: Option<HashMap<String, Session>> = None;

fn create_new_session() -> String {
    let session_id = format!("{}", Utc::now().timestamp());
    let mut board = Chessboard::default();
    board.init();
    let session = Session::new(session_id.clone(), board , 1, WHITE, BLACK);

    unsafe {
        GLOBAL_SESSIONS
            .as_mut()
            .unwrap()
            .insert(session_id.clone(), session);
    }
    session_id
}

fn check_session(req: &HttpRequest) -> &mut Session {
    let cookie = req.cookie("session_id").expect("No session cookie found");
    let session_id = cookie.value().to_owned();
    //get session from session_id
    let session = get_session(&session_id[..]).unwrap_or_else(
        || {
            println!("Session not found");
            let new_session_id = create_new_session();
            get_session(&new_session_id[..]).unwrap()
        });
    session
}

fn get_session(session_id: &str) -> Option<&'static mut Session> {
    unsafe {
        GLOBAL_SESSIONS
            .as_mut()
            .unwrap()
            .get_mut(session_id)
    }
}

fn get_all_sessions() -> Vec<&'static mut Session> {
    unsafe {
        GLOBAL_SESSIONS
            .as_mut()
            .unwrap()
            .values_mut()
            .collect()
    }
}

fn get_session_count() -> usize {
    unsafe {
        GLOBAL_SESSIONS
            .as_mut()
            .unwrap()
            .len()
    }
}
// This function initializes the global variable with an empty hashmap.
fn init_sessions() {
    unsafe {
        GLOBAL_SESSIONS = Some(HashMap::new());
    }
}