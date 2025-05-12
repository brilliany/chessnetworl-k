import * as API from "/js/web_utils.js"

//session id from cookies
const sessionId = API.getCookie("session_id");
let color = 0;
const engine_thinking_div = document.getElementById("engine-thinking");
let savedPossibleMoves = []
let pieces = [] // keeps track of the pieces on the board, the 'element' property might not correspond to the actual element on the board

generateIcon();

await initialRequest(sessionId);
async function initialRequest(sessionId) {
    let data = await API.getSession(sessionId);
    console.log(data)
    color = -data.opponent;
    populateBoard(sessionId);
}

/**
 * Populates the board with the pieces
 * sets the child elements of each square to the corresponding piece
 * start of a populated row looks like this:
 * <div class="row" id="0">
 *     <div class="square white">
 *        <div class="piece" id="white_rook_0">
 *            <img src="img/white_rook.png" alt="white rook">
 *        </div>
 *     </div>
 *     <div class="square black">
 *         <div class="piece" id="white_knight_0">
 *             <img src="img/white_knight.png" alt="white knight">
 *         </div>
 *     </div>
 *     <-- ... -->
 */
function populateBoard() {
    // Path: /api/populate-board, returns a json with the board with the pieces
// ex. { "white_pawns": <white pawns bitboard>
//       "white_rooks": <white rooks bitboard>

    fetch("/api/populate-board", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    }).then((response) => {
        if (response.status === 200) {
        /*
            * The JSON string is formatted as follows:
    * {
    *   board: [
    *       [black_rook, black_knight, black_bishop, black_queen, black_king, black_bishop, black_knight, black_rook],
    *       [black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn],
    *       [empty, empty, empty, empty, empty, empty, empty, empty],
        */
            response.json().then((data) => {

                const board = JSON.parse(data).board;

                for (let i = 0; i < board.length; i++) {
                    const row = board[i];
                    const rowElement = document.getElementById("row" + i);

                    for (let j = 0; j < row.length; j++) {
                        const piece_name = row[j];
                        const squareElement = rowElement.children[j];
                        console.log("squareElement has " + squareElement.children.length + " children");
                        if (piece_name !== "empty") {
                            addPieceToSquare(squareElement, piece_name,j, i);
                        } else {
                            squareElement.children[0].innerHTML = "";
                        }
                    }
                }
            });

        } else {
            alert("Could not populate board");
        }
    });
}


function addPieceToSquare(squareElement, piece_name,x,y) {
    const pieceElement = squareElement.children[0];
    //if the piece element doesn't have a child, add one
    if (pieceElement.children.length === 0) {
        const pieceImg = document.createElement("img");
        pieceElement.appendChild(pieceImg);
    }
    const pieceImg = pieceElement.children[0];
    pieceImg.setAttribute("src", "/pieces/" + piece_name + ".png");
    pieceImg.setAttribute("alt", piece_name);

    let listenerFunction = null;
    //add event listener to piece if it's the player's color
    console.log(color)
    if (color === 1 && piece_name.startsWith("white") || color === -1 && piece_name.startsWith("black")) {
        listenerFunction = addListenerToPiece(pieceImg, piece_name, x, y);
    }
    pieces.push({
        location: [x, y],
        piece: piece_name,
        element: pieceImg,
        listenerFunction: listenerFunction,
    })
}
function addListenerToPiece(pieceImg, piece_name, x, y) {
    console.log("adding listener to: " + piece_name)
    let listenerFunction = function () {
        if (savedPossibleMoves.length > 0) {
            //remove all possible move squares
            removePossibleMoves();
            return;
        }
        console.log("Clicked on piece " + piece_name + " at " + x + ", " + y);
        // Path: /api/get-possible-moves, returns a json with the possible moves for the piece
        // ex. { "possible_moves": [x1, y1, x2, y2]
        let queries = [
            ["x", x],
            ["y", y],
            ["piece", piece_name],
        ]
        fetch("/api/possible-moves" + "?" + new URLSearchParams(queries), {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",

        }).then((response) => {
            if (response.status === 200) {
                console.log(response);
                response.json().then((possibleMoves) => {
                    //possibleMoves is an array of moves
                    /**
                     [
                        {
                         from_square: <index from 0-63>,
                        to_square: <index from 0-63>,
                        },
                        ...
                     ]
                     */
                    let receivedMoves = Object.values(possibleMoves);
                    console.log("Possible moves: " + receivedMoves.length);
                    for (let i = 0; i < possibleMoves.length; i++) {
                        const move = possibleMoves[i];
                        const fromX = move.from_square % 8;
                        const fromY = Math.floor(move.from_square / 8);
                        const toX = move.to_square % 8;
                        const toY = Math.floor(move.to_square / 8);
                        const toSquare = document.getElementById("row" + toY).children[toX];
                        toSquare.classList.add("possible-move");
                        let possibleMoveListener = function () {
                            makeMove(x, y, toX, toY, piece_name);
                        };
                        toSquare.addEventListener("click", possibleMoveListener, {once: true});
                        savedPossibleMoves.push(
                            {
                                location: [toX, toY],
                                listenerFunction: possibleMoveListener,
                            }
                        )
                    }
                });
            } else {
                alert("Could not get possible moves");
            }
        });
    };
    pieceImg.addEventListener("click", listenerFunction);
    return listenerFunction;
}

function makeMove(x, y, toX, toY, pieceName) {
    // Path: /api/move-piece, moves a piece
    let queries = [
        ["from_x", x],
        ["from_y", y],
        ["to_x", toX],
        ["to_y", toY],
        ["piece", pieceName],
    ]
    fetch("/move-piece" + "?" + new URLSearchParams(queries), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",

    }).then((response) => {
        if (response.status === 200) {
            movePiece(x, y, toX, toY);
            makeEngineMove();
        } else {
            alert("Could not move piece");
        }
    });
}
function movePiece(x, y, toX, toY) {
 //moves a specific piece from one square to another in the html
    const fromSquare = document.getElementById("row" + y).children[x];
    const toSquare = document.getElementById("row" + toY).children[toX];
    if (fromSquare.children[0].children === 0) {
        alert("No piece to move");
        return;
    }
    let pieceElement = fromSquare.children[0].children[0];
    //if piece on toSquare, remove it
    if (toSquare.children[0].children.length > 0) {
        toSquare.children[0].children[0].remove();
    }
    //remove piece from fromSquare
    fromSquare.children[0].innerHTML = "";
    //add piece to toSquare
    toSquare.children[0].appendChild(pieceElement);
    //add event listener to piece if it's the player's color
    const pieceColor = pieceElement.getAttribute("alt").startsWith("white") ? 1 : -1;
    if (pieceColor === color) {
        addListenerToPiece(pieceElement, pieceElement.getAttribute("alt"), toX, toY);
    }
    removePossibleMoves();
}
const frozenSquares = [];

function freezeBoard() {
    //this method should freeze the board, so that the player can't move any pieces
    removePossibleMoves();
    frozenSquares.length = 0;
    //loop through all squares and remove event listeners from own pieces
    for (let i = 0; i < pieces.length; i++) {
        const piece = pieces[i];
        if (piece.listenerFunction) {
            piece.element.removeEventListener("click", piece.listenerFunction);
            frozenSquares.push(piece.location);
        }
    }
}

function unfreezeBoard() {
    //this method should unfreeze the board, so that the player can move pieces again
    //loop through all squares and add event listeners to own pieces
    for (let i = 0; i < pieces.length; i++) {
        const piece = pieces[i];
        if (piece.listenerFunction) {
            piece.element.addEventListener("click", piece.listenerFunction);
        }
    }
}

function makeEngineMove() {
    freezeBoard();
    engine_thinking_div.style.display = "block";
    // Path: /api/make-engine-move, returns a json with the possible moves for the piece
    // ex. { "possible_moves": [x1, y1, x2, y2]
    fetch("/api/make-engine-move", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",

    }).then((response) => {
        if (response.status === 200) {
            console.log(response);
            response.json().then((move) => {
                /**
                    {
                     from_square: <index from 0-63>,
                    to_square: <index from 0-63>,
                    }
                 */
                const fromX = move.from_square % 8;
                const fromY = Math.floor(move.from_square / 8);
                const toX = move.to_square % 8;
                const toY = Math.floor(move.to_square / 8);
                movePiece(fromX, fromY, toX, toY);
                unfreezeBoard();
                console.log("Engine moved from " + fromX + ", " + fromY + " to " + toX + ", " + toY);
                engine_thinking_div.style.display = "none";
            });
        } else {
            alert("Could not make engine move");
        }
    });
}
function removePossibleMoves() {
    console.log("Removing possible moves: " + savedPossibleMoves.length);
    for(let i = savedPossibleMoves.length - 1; i >= 0; i--){
        const move = savedPossibleMoves[i].location;
        const square = document.getElementById("row" + move[1]).children[move[0]];
        square.classList.remove("possible-move");
        square.removeEventListener("click", savedPossibleMoves[i].listenerFunction, {once: true});
        //remove from savedPossibleMoves
        savedPossibleMoves.splice(i, 1);
        console.log("Removed possible move index " + i);
    }
}
function generateIcon() {
    const container = document.getElementById("user-name");
    let text = container.children[0];
    if (text) {
        text.textContent = "User" + sessionId;
    } else {
        const text = document.createElement("h1");
        text.textContent = "User" + sessionId;
        container.appendChild(text);
    }
}