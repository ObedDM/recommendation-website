export default function HomePath() {

    async function getHome() {
        try {
            const response = await fetch("http://localhost:5050/gethome", {
                method: "GET",
                credentials: "include"
            });

            const result = await response.json();

            if (response.ok) {
                console.log(result)

                return true

            } else {
                console.warn(result.message)

                return false
            }
        } catch (err) {
            console.error("Error getting /Home info", err)
        }
    }

    return (
        <div className="bg-white">
            <h1 className="text-blue-500"> Welcome Home </h1>

            <button onClick={getHome}>
                ee
            </button>

        </div>
    )
}