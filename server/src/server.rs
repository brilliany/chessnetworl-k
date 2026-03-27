use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tower_http::{services::ServeDir, services::ServeFile, trace::TraceLayer};

use chessnetwork_core::movegenerator::generate_moves;
use chessnetwork_core::*;

use crate::session::Session;
use crate::{get_config, get_config_value};

// Shared application state

type Sessions = Arc<Mutex<HashMap<String, Session>>>;

#[derive(Clone)]
struct AppState {
    sessions: Sessions,
}

#[tokio::main]
pub(crate) async fn main() {
    let config = get_config();
    let addr = get_config_value(&config, "http_server_address");
    let port = get_config_value(&config, "http_server_port");

    let state = AppState {
        sessions: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = api_routes(state)
        .merge(page_routes())
        .merge(serve_dirs())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port))
        .await
        .unwrap();

    println!("Listening on {}:{}", addr, port);
    axum::serve(listener, app).await.unwrap();
}

// Static-file serving
fn serve_dirs() -> Router {
    const STATIC_DIRS: &[(&str, &str)] = &[
        ("/css", "static/css"),
        ("/js", "static/js"),
        ("/icons", "static/icons"),
        ("/pieces", "static/pieces"),
    ];

    let fallback = ServeFile::new("static/invalid_request.html");
    let mut router = Router::new();
    for (route, path) in STATIC_DIRS {
        router = router.nest_service(route, ServeDir::new(path).not_found_service(fallback.clone()));
    }
    router.fallback_service(fallback)
}

// webpages
fn page_routes() -> Router {
    Router::new()
        .route("/", get(welcome))
        .route("/play-engine", get(play_engine))
        .route("/solo-game", get(solo_game))
}

async fn welcome() -> Html<String> {
    Html(read_static_file("static/welcome.html"))
}

async fn play_engine(jar: CookieJar) -> Response {
    let body = read_static_file("static/chessboard.html");
    let common_script = read_static_file("static/js/chess-common.js");
    let script = read_static_file("static/js/play-engine.js");
    let body = format!("{}<script>{}</script><script>{}</script>", body, common_script, script);

    if jar.get("session_id").is_some() {
        return Html(body).into_response();
    }

    // No session cookie yet – create one and set it.
    let session_id = format!("{}", Utc::now().timestamp());
    let cookie = format!("session_id={}; Path=/; HttpOnly=false", session_id);

    (
        [("set-cookie", cookie)],
        Html(body),
    )
        .into_response()
}

async fn solo_game(jar: CookieJar) -> Response {
    let body = read_static_file("static/chessboard.html");
    let common_script = read_static_file("static/js/chess-common.js");
    let script = read_static_file("static/js/solo-game.js");
    let body = format!("{}<script>{}</script><script>{}</script>", body, common_script, script);

    if jar.get("session_id").is_some() {
        return Html(body).into_response();
    }

    // No session cookie yet - create one and set it.
    let session_id = format!("{}", Utc::now().timestamp());
    let cookie = format!("session_id={}; Path=/; HttpOnly=false", session_id);

    (
        [("set-cookie", cookie)],
        Html(body),
    )
        .into_response()
}

fn read_static_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading {path}: {e}");
        "Error reading file".into()
    })
}

// API routes
fn api_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/get-session", get(get_session_object))
        .route("/api/populate-board", get(populate_board))
        .route("/api/possible-moves", get(possible_moves))
        .route("/api/make-engine-move", get(make_engine_move))
        .route("/api/move-piece", post(move_piece))
        .with_state(state)
}


// response types
#[derive(Deserialize)]
struct PossibleMovesQuery {
    x: u8,
    y: u8,
    color: i8,
}

#[derive(Deserialize)]
struct MovePieceQuery {
    from_x: u8,
    from_y: u8,
    to_x: u8,
    to_y: u8,
    special_move: Option<SpecialMoveHint>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SpecialMoveHint {
    Normal,
    EnPassant,
    Castling,
}

#[derive(Serialize)]
struct MoveJson {
    from_x: u8,
    from_y: u8,
    to_x: u8,
    to_y: u8,
    special_move: SpecialMoveHint,
}

#[derive(Serialize)]
struct PossibleMovesResponse {
    moves: Vec<MoveJson>,
}

#[derive(Serialize)]
struct SessionResponse {
    board: serde_json::Value,
    turn: i8,
    opponent: i8,
    user: i8,
    session_id: String,
}

// Handlers
async fn get_session_object(
    jar: CookieJar,
    State(state): State<AppState>,
) -> Json<SessionResponse> {
    let session = get_or_create_session(&jar, &state.sessions);

    let board_json: serde_json::Value =
        serde_json::from_str(&session.board.clone().convert_to_json())
            .expect("board json parse error");

    let user_color = session.get_user_color();
    let opponent = if user_color == WHITE { BLACK } else { WHITE };

    Json(SessionResponse {
        board: board_json,
        turn: session.get_turn(),
        opponent,
        user: user_color,
        session_id: session.get_id(),
    })
}

async fn populate_board(
    jar: CookieJar,
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let session = get_or_create_session(&jar, &state.sessions);
    let mut board = session.board.clone();

    let session_count = state.sessions.lock()
        .map(|s| s.len())
        .unwrap_or(0);

    println!(
        "Sessions: {} | Sending board for session {}",
        session_count,
        session.id
    );
    board.print_board();

    let json: serde_json::Value =
        serde_json::from_str(&board.convert_to_json()).expect("board json parse error");
    Json(json)
}

async fn possible_moves(
    jar: CookieJar,
    State(state): State<AppState>,
    Query(params): Query<PossibleMovesQuery>,
) -> Json<PossibleMovesResponse> {
    let x = params.x;
    let y = params.y;
    let color = params.color;

    let mut session = get_or_create_session(&jar, &state.sessions);
    let chessboard = &mut session.board;
    chessboard.print_board();

    let moves: Vec<MoveJson> = generate_moves(chessboard, color)
        .into_iter()
        .filter(|mv| mv.get_from_x() == x && mv.get_from_y() == y)
        .map(|mv| MoveJson {
            from_x: mv.get_from_x(),
            from_y: mv.get_from_y(),
            to_x: mv.get_to_x(),
            to_y: mv.get_to_y(),
            special_move: move_to_special_move_hint(&mv),
        })
        .collect();

    Json(PossibleMovesResponse { moves })
}

async fn move_piece(
    jar: CookieJar,
    State(state): State<AppState>,
    Query(params): Query<MovePieceQuery>,
) -> impl IntoResponse {
    let from_x = params.from_x;
    let from_y = params.from_y;
    let to_x = params.to_x;
    let to_y = params.to_y;
    let special_move = params.special_move;

    let session_id = get_session_id(&jar);

    //lock session during move
    {
        let mut sessions = match state.sessions.lock() {
            Ok(s) => s,
            Err(poisoned) => {
                eprintln!("Thread panicked while moving piece in session {}, recovering...", session_id);
                poisoned.into_inner()
            }
        };

        let count = sessions.len();
        let session = sessions
            .get_mut(&session_id)
            .expect("session not found");

        let turn = session.get_turn();
        let selected_move = generate_moves(&mut session.board, turn)
            .into_iter()
            .find(|mv| {
                mv.get_from_x() == from_x
                    && mv.get_from_y() == from_y
                    && mv.get_to_x() == to_x
                    && mv.get_to_y() == to_y
                    && special_move
                        .map(|hint| move_to_special_move_hint(mv) == hint)
                        .unwrap_or(true)
            });

        let Some(selected_move) = selected_move else {
            return axum::http::StatusCode::BAD_REQUEST;
        };

        session.make_move(selected_move);

        println!(
            "Sessions: {} | Session {} board:",
            count,
            session.id
        );
        session.board.print_board();
    };

    axum::http::StatusCode::OK
}

async fn make_engine_move(
    jar: CookieJar,
    State(state): State<AppState>,
) -> Json<MoveJson> {
    let session_id = get_session_id(&jar);

    // lock session to get board
    let mut board = {
        let sessions = match state.sessions.lock() {
            Ok(s) => s,
            Err(poisoned) => {
                eprintln!("Thread panicked while making engine move for session {}, recovering...", session_id);
                poisoned.into_inner()
            }
        };

        let session = sessions
            .get(&session_id)
            .expect("session not found");

        println!("Engine making move for session {}", session.id);
        session.board.clone()
    };

    // run engine without lock to not block requests
    board.print_board();
    let mut engine = Engine::new_single(60, BLACK);
    let engine_move = engine.get_best_move(&mut board).unwrap();

    let result = MoveJson {
        from_x: engine_move.get_from_x(),
        from_y: engine_move.get_from_y(),
        to_x: engine_move.get_to_x(),
        to_y: engine_move.get_to_y(),
        special_move: move_to_special_move_hint(&engine_move),
    };

    // lock session again to move
    {
        let mut sessions = match state.sessions.lock() {
            Ok(s) => s,
            Err(poisoned) => {
                eprintln!("Thread panicked while applying engine move for session {}, recovering...", session_id);
                poisoned.into_inner()
            }
        };

        let session = sessions
            .get_mut(&session_id)
            .expect("session not found");

        session.make_move(engine_move);
    }

    Json(result)
}

fn move_to_special_move_hint(mv: &Move) -> SpecialMoveHint {
    if mv.is_en_passant() {
        SpecialMoveHint::EnPassant
    } else if mv.is_castling() {
        SpecialMoveHint::Castling
    } else {
        SpecialMoveHint::Normal
    }
}

// Session helpers
/// Read the session id from the cookie jar, or return "0" if not found
fn get_session_id(jar: &CookieJar) -> String {
    jar.get("session_id")
        .map(|c| c.value().to_owned())
        .unwrap_or_else(|| "0".into())
}

/// Return clone of the session
/// Creates a new session if none exists for the cookie.
fn get_or_create_session(jar: &CookieJar, sessions: &Sessions) -> Session {
    let id = get_session_id(jar);
    let mut map = match sessions.lock() {
        Ok(m) => m,
        Err(poisoned) => {
            eprintln!("Thread panicked while accessing sessions for session {}, recovering...", id);
            poisoned.into_inner()
        }
    };

    if !map.contains_key(&id) {
        let mut board = Chessboard::default();
        board.init();
        let session = Session::new(id.clone(), board, 1, WHITE, BLACK);
        map.insert(id.clone(), session);
    }

    map.get(&id).unwrap().clone()
}
