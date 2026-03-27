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

async function getSession(sessionID, errorMessage = "Could not get session") {
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
            alert(errorMessage);
        }
    });
    return data;
}

function removePossibleMoves(savedPossibleMoves) {
    for (let i = savedPossibleMoves.length - 1; i >= 0; i--) {
        const move = savedPossibleMoves[i].location;
        const square = document.getElementById("row" + move[1]).children[move[0]];
        square.classList.remove("possible-move");
        square.removeEventListener("click", savedPossibleMoves[i].listenerFunction, { once: true });
        savedPossibleMoves.splice(i, 1);
    }
}

function addListenerToPiece(pieceImg, pieceName, x, y, currentColor, savedPossibleMoves, makeMove) {
    const listenerFunction = async function () {
        if (savedPossibleMoves.length > 0) {
            removePossibleMoves(savedPossibleMoves);
            return;
        }

        const queries = [
            ["x", 7 - x],
            ["y", 7 - y],
            ["color", currentColor],
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
                        const specialMove = possibleMoves[i].special_move;
                        const possibleMoveListener = function () {
                            makeMove(x, y, toX, toY, specialMove);
                        };

                        toSquare.classList.add("possible-move");
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

function addPieceToSquare(squareElement, pieceName, x, y, currentColor, savedPossibleMoves, pieces, makeMove) {
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
    if (pieceColor === currentColor) {
        listenerFunction = addListenerToPiece(pieceImg, pieceName, x, y, currentColor, savedPossibleMoves, makeMove);
    }

    pieces.push({
        location: [x, y],
        piece: pieceName,
        element: pieceImg,
        listenerFunction: listenerFunction,
    });
}

