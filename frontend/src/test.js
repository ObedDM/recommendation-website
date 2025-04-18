async function fetchTest() {
    const response = await fetch("http://127.0.0.1:5050/auth/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username: "cruella1",
            password: "a123"
        })
    });
    const result = await response.json();

    return result
}

fetchTest().then(e => console.log(e));