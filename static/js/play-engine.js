//session id from cookies
const sessionId = getCookie("session_id");
let color = 0;
let savedPossibleMoves = []
let pieces = [] // keeps track of the pieces on the board, the 'element' property might not correspond to the actual element on the board
let isSubmittingMove = false;


initialRequest(sessionId);
async function initialRequest(sessionId) {
    let data = await getSession(sessionId, "Could not get color");
    color = data.opponent ^ 1;
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
    fetch("/api/populate-board", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    }).then((response) => {
        if (response.status === 200) {
            response.json().then((data) => {
                console.log(data);
                const board = data.board;

                // Make 0th rank the top row for white by flipping indices
                const flippedBoard = color === 1 ? board.slice().reverse() : board;

                for (let i = 0; i < flippedBoard.length; i++) {
                    const row = flippedBoard[i];
                    const rowElement = document.getElementById("row" + i);
                    for (let j = 0; j < row.length; j++) {
                        const piece_name = row[7-j];
                        const squareElement = rowElement.children[j];
                        console.log("squareElement has " + squareElement.children.length + " children");
                        if (piece_name !== "empty") {
                            addPieceToSquare(squareElement, piece_name, j, i, color, savedPossibleMoves, pieces, makeMove);
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
function makeMove(x, y, toX, toY, specialMove) {
    if (isSubmittingMove) {
        return;
    }
    isSubmittingMove = true;

    let queries = [
        //we have to flip the board for the internal board
        ["from_x", 7-x],
        ["from_y", 7-y],
        ["to_x", 7-toX],
        ["to_y", 7-toY],
    ]
    if (specialMove) {
        queries.push(["special_move", specialMove]);
    }
    fetch("/api/move-piece" + "?" + new URLSearchParams(queries), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",

    }).then((response) => {
        if (response.status === 200) {
            movePiece(x, y, toX, toY, specialMove);
            makeEngineMove();
        } else {
            isSubmittingMove = false;
            alert("Could not move piece");
        }
    }).catch(() => {
        isSubmittingMove = false;
        alert("Could not move piece");
    });
}
function movePiece(x, y, toX, toY, specialMove = "normal") {
 //moves a specific piece from one square to another in the html
    const fromSquare = document.getElementById("row" + y).children[x];
    const toSquare = document.getElementById("row" + toY).children[toX];
    let first = fromSquare.children[0];
    let pawn = first.children[0].getAttribute("alt").endsWith("pawn");
    console.log(first.children[0])
    if (first.children[0] === undefined) {
        alert("No piece to move");
        return;
    }
    let pieceElement = first.children[0];
    //if piece on toSquare, remove it
    if (specialMove === "en_passant") {
        document.getElementById("row" + (toY + color)).children[toX].children[0].children[0].remove()
    } else if (toSquare.children[0].children.length > 0) {
        toSquare.children[0].children[0].remove();
    } else if (pawn && toX !== x) {
        document.getElementById("row" + (toY + color)).children[toX].children[0].children[0].remove()
    }
    //remove piece from fromSquare
    first.innerHTML = "";
    //add piece to toSquare
    toSquare.children[0].appendChild(pieceElement);

    if (specialMove === "castling") {
        const rookFromX = toX > x ? 7 : 0;
        const rookToX = toX > x ? toX - 1 : toX + 1;
        const rookFromSquare = document.getElementById("row" + y).children[rookFromX];
        const rookToSquare = document.getElementById("row" + y).children[rookToX];
        if (rookFromSquare.children[0].children.length > 0) {
            const rookElement = rookFromSquare.children[0].children[0];
            rookFromSquare.children[0].innerHTML = "";
            if (rookToSquare.children[0].children.length > 0) {
                rookToSquare.children[0].children[0].remove();
            }
            rookToSquare.children[0].appendChild(rookElement);
        }
    }

    //add event listener to piece if it's the player's color
    const pieceColor = pieceElement.getAttribute("alt").startsWith("white") ? 1 : 0;
    if (pieceColor === color) {
        addListenerToPiece(pieceElement, pieceElement.getAttribute("alt"), toX, toY, color, savedPossibleMoves, makeMove);
    }
    removePossibleMoves(savedPossibleMoves);
}
const frozenSquares = [];

function freezeBoard() {
    //this method should freeze the board, so that the player can't move any pieces
    removePossibleMoves(savedPossibleMoves);
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
    fetch("/api/make-engine-move", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",

    }).then((response) => {
        if (response.status === 200) {
            response.json().then((json) => {
                console.log(json)
                const fromX = 7 - json.from_x;
                const fromY = 7 - json.from_y;
                const toX = 7 - json.to_x;
                const toY = 7 - json.to_y;
                movePiece(fromX, fromY, toX, toY, json.special_move || "normal");
                unfreezeBoard();
                isSubmittingMove = false;
                console.log("Engine moved from " + fromX + ", " + fromY + " to " + toX + ", " + toY);
            });
        } else {
            isSubmittingMove = false;
            alert("Could not make engine move");
        }
    }).catch(() => {
        isSubmittingMove = false;
        alert("Could not make engine move");
    });
}
