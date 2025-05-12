export function getCookie(name) {
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

export async function getSession(sessionID) {
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
        if (response.status === 200) {
            await response.json().then((raw) => {
                data = JSON.parse(raw);
            });
        } else {
            alert("Could not get color");
        }
    });
    return data;
}