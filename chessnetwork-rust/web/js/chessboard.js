

//session id from cookies
const sessionId = getCookie("session_id");
let color = 0;
let savedPossibleMoves = []
let pieces = [] // keeps track of the pieces on the board, the 'element' property might not correspond to the actual element on the board


await initialRequest(sessionId);
async function initialRequest(sessionId) {
    let data = await getSession(sessionId);
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
                const board = JSON.parse(data).board;

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
                            addPieceToSquare(squareElement, piece_name, j, i);
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
    pieceImg.setAttribute("src", "./pieces/" + piece_name + ".png");
    pieceImg.setAttribute("alt", piece_name);

    let listenerFunction = null;
    //add event listener to piece if it's the player's color
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
            ["x", 7-x],
            ["y", 7-y],
            ["color", color],
        ];
        fetch("/api/possible-moves" + "?" + new URLSearchParams(queries), {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
        }).then((response) => {
            if (response.status === 200) {
                response.json().then((data) => {
                    let possibleMoves = JSON.parse(data).moves;
                    for (let i = 0; i < possibleMoves.length; i++) {
                        const toX = 7- possibleMoves[i].to_x;
                        const toY = 7- possibleMoves[i].to_y;
                        const toSquare = document.getElementById("row" + toY).children[toX];
                        toSquare.classList.add("possible-move");
                        let possibleMoveListener = function () {
                            makeMove(x, y, toX, toY, piece_name);
                        };
                        toSquare.addEventListener("click", possibleMoveListener, { once: true });
                        savedPossibleMoves.push({
                            location: [toX, toY],
                            listenerFunction: possibleMoveListener,
                        });
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
        //we have to flip the board for the internal board
        ["from_x", 7-x],
        ["from_y", 7-y],
        ["to_x", 7-toX],
        ["to_y", 7-toY],
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
    let first = fromSquare.children[0];
    let pawn = first.children[0].getAttribute("alt").endsWith("pawn");
    console.log(first.children[0])
    if (first.children[0] === undefined) {
        alert("No piece to move");
        return;
    }
    let pieceElement = first.children[0];
    //if piece on toSquare, remove it
    if (toSquare.children[0].children.length > 0) {
        toSquare.children[0].children[0].remove();
    } else if (pawn && toX !== x) {
        document.getElementById("row" + (toY + color)).children[toX].children[0].children[0].remove()
    }
    //remove piece from fromSquare
    first.innerHTML = "";
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
            response.json().then((response) => {
                let json = JSON.parse(response);
                console.log(json)
                const fromX = 7 - json.from_x;
                const fromY = 7 - json.from_y;
                const toX = 7 - json.to_x;
                const toY = 7 - json.to_y;
                movePiece(fromX, fromY, toX, toY);
                unfreezeBoard();
                console.log("Engine moved from " + fromX + ", " + fromY + " to " + toX + ", " + toY);
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



function getCookie(name) {
    let cookieValue = null;
    if (document.cookie && document.cookie !== '') {
        const cookies = document.cookie.split(';'); //split cookies by ;
        for (let i = 0; i < cookies.length; i++) {
            const cookie = cookies[i].trim(); //trim spaces
            // Does this cookie string begin with the name we want?
            if (cookie.substring(0, name.length + 1) === (name + '=')) { //if cookie name is found
                cookieValue = decodeURIComponent(cookie.substring(name.length + 1)); //get cookie value
                break;
            }
        }
    }
    return cookieValue;
}

async function getSession(sessionID) {
    // Path: /api/get-session, returns a json with the color of the player
    // ex. { "color": "white" }
    let data;
    await fetch("/api/get-session", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    }).then(async (response) => {
        console.log(response)
        if (response.status === 200) {
            await response.json().then((raw) => {
                console.log(raw)
                data = JSON.parse(raw);
            });
        } else {
            alert("Could not get color");
        }
    });
    console.log(data)
    return data;
}