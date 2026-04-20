// session id from cookies
const sessionId = getCookie("session_id");
const boardOrientation = 1;
let currentTurn = 1;
let savedPossibleMoves = [];
let pieces = [];
let isSubmittingMove = false;

initialRequest(sessionId);

async function initialRequest(sessionId) {
    const data = await getSession(sessionId, "Could not get session");
    if (data && typeof data.turn === "number") {
        currentTurn = data.turn;
    }
    populateBoard();
}

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
                const board = data.board;
                pieces = [];
                removePossibleMoves(savedPossibleMoves);

                const flippedBoard = boardOrientation === 1 ? board.slice().reverse() : board;

                for (let i = 0; i < flippedBoard.length; i++) {
                    const row = flippedBoard[i];
                    const rowElement = document.getElementById("row" + i);
                    for (let j = 0; j < row.length; j++) {
                        const pieceName = row[7 - j];
                        const squareElement = rowElement.children[j];
                        squareElement.children[0].innerHTML = "";
                        if (pieceName !== "empty") {
                            addPieceToSquare(squareElement, pieceName, j, i, currentTurn, savedPossibleMoves, pieces, makeMove);
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

    const queries = [
        ["from_x", 7 - x],
        ["from_y", 7 - y],
        ["to_x", 7 - toX],
        ["to_y", 7 - toY],
    ];

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
             currentTurn ^= 1;
             populateBoard();
            isSubmittingMove = false;
        } else {
            isSubmittingMove = false;
            alert("Could not move piece");
        }
    }).catch(() => {
        isSubmittingMove = false;
        alert("Could not move piece");
    });
}


