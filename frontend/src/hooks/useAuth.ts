import { useEffect, useState } from "react";

export default function useAuth<T>(endpoint: string, options: RequestInit = {}) {
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(true);

    const fetchData = async (retry = 0) => {
        try {
            const response = await fetch(`http://localhost:5050${endpoint}`, { ...options, credentials: "include"});

            if (response.status === 401 && retry < 2) {
                const refresh = await fetch("http://localhost:5050/auth/refresh", {
                    method: "POST",
                    credentials: "include",
                });

                if (refresh.ok) {
                    return fetchData(retry + 1);
                }

                else {
                    setError("Session has expired, please log in again.");
                    return;
                }
            }

            const result = await response.json();

            if (response.ok) {
                setData(result);
            }

            else {
                setError(result.message)
            }
        }

        catch {
            setError("An error occurred while fetching data.");
        }

        finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchData();

        const interval = setInterval(() => {
            fetch("http://localhost:5050/auth/refresh", {
                method: "POST",
                credentials: "include",
            });
        }, 5 * 60 * 60 * 1000); // refreshes every 5 hours

        return () => clearInterval(interval);
    }, []);

    return{ data, error, loading }
}