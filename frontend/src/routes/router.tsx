import { createBrowserRouter } from "react-router-dom";
import App from "../App";
import AuthPath from "./Auth/AuthPath";
import HomePath from "./Home/HomePath";
import NotFound from "../components/NotFound/NotFound";

const router = createBrowserRouter([
    {path: "/", element: <App />},
    {path: "/home", element: <HomePath />},
    {path: "/auth", element: <AuthPath />},
    {path: "*", element: <NotFound />},
  ]);

export default router