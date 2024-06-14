import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import {RouterProvider} from "react-router-dom";
import router from "./pages/router.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RouterProvider router={router}/>
    </React.StrictMode>,
);
