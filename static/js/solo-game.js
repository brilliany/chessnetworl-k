// session id from cookies
const sessionId = getCookie("session_id");
const boardOrientation = 1;
let currentTurn = 1;
let savedPossibleMoves = [];
let pieces = [];
let isSubmittingMove = false;

initialRequest(sessionId);

async function initialRequest(sessionId) {
    const data = await getSession(sessionId);
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
                removePossibleMoves();

                const flippedBoard = boardOrientation === 1 ? board.slice().reverse() : board;

                for (let i = 0; i < flippedBoard.length; i++) {
                    const row = flippedBoard[i];
                    const rowElement = document.getElementById("row" + i);
                    for (let j = 0; j < row.length; j++) {
                        const pieceName = row[7 - j];
                        const squareElement = rowElement.children[j];
                        squareElement.children[0].innerHTML = "";
                        if (pieceName !== "empty") {
                            addPieceToSquare(squareElement, pieceName, j, i);
                        }
                    }
                }
            });
        } else {
            alert("Could not populate board");
        }
    });
}

function addPieceToSquare(squareElement, pieceName, x, y) {
    const pieceElement = squareElement.children[0];
    if (pieceElement.children.length === 0) {
        const pieceImg = document.createElement("img");
        pieceElement.appendChild(pieceImg);
    }

    const pieceImg = pieceElement.children[0];
    pieceImg.setAttribute("src", "./pieces/" + pieceName + ".png");
    pieceImg.setAttribute("alt", pieceName);

    let listenerFunction = null;
    const pieceColor = pieceName.startsWith("white") ? 1 : -1;
    if (pieceColor === currentTurn) {
        listenerFunction = addListenerToPiece(pieceImg, pieceName, x, y);
    }

    pieces.push({
        location: [x, y],
        piece: pieceName,
        element: pieceImg,
        listenerFunction: listenerFunction,
    });
}

function addListenerToPiece(pieceImg, pieceName, x, y) {
    const listenerFunction = async function () {
        if (savedPossibleMoves.length > 0) {
            removePossibleMoves();
            return;
        }

        const queries = [
            ["x", 7 - x],
            ["y", 7 - y],
            ["color", currentTurn],
        ];

        await fetch("/api/possible-moves" + "?" + new URLSearchParams(queries), {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
        }).then((response) => {
            if (response.status === 200) {
                response.json().then((data) => {
                    const possibleMoves = data.moves;
                    for (let i = 0; i < possibleMoves.length; i++) {
                        const toX = 7 - possibleMoves[i].to_x;
                        const toY = 7 - possibleMoves[i].to_y;
                        const toSquare = document.getElementById("row" + toY).children[toX];
                        toSquare.classList.add("possible-move");

                        const specialMove = possibleMoves[i].special_move;
                        const possibleMoveListener = function () {
                            makeMove(x, y, toX, toY, specialMove);
                        };

                        toSquare.addEventListener("click", possibleMoveListener, { once: true });
                        savedPossibleMoves.push({
                            location: [toX, toY],
                            specialMove: specialMove,
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
            currentTurn *= -1;
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

function removePossibleMoves() {
    for (let i = savedPossibleMoves.length - 1; i >= 0; i--) {
        const move = savedPossibleMoves[i].location;
        const square = document.getElementById("row" + move[1]).children[move[0]];
        square.classList.remove("possible-move");
        square.removeEventListener("click", savedPossibleMoves[i].listenerFunction, { once: true });
        savedPossibleMoves.splice(i, 1);
    }
}

function getCookie(name) {
    let cookieValue = null;
    if (document.cookie && document.cookie !== "") {
        const cookies = document.cookie.split(";");
        for (let i = 0; i < cookies.length; i++) {
            const cookie = cookies[i].trim();
            if (cookie.substring(0, name.length + 1) === (name + "=")) {
                cookieValue = decodeURIComponent(cookie.substring(name.length + 1));
                break;
            }
        }
    }
    return cookieValue;
}

async function getSession(sessionID) {
    let data;
    await fetch("/api/get-session", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    }).then(async (response) => {
        if (response.status === 200) {
            await response.json().then((raw) => {
                data = raw;
            });
        } else {
            alert("Could not get session");
        }
    });
    return data;
}

