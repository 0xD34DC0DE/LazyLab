import {createBrowserRouter, createRoutesFromElements, Route} from "react-router-dom";
import Root from "../Root.tsx";
import SessionsPage from "./SessionsPage.tsx";
import AddSessionModal from "./AddSessionModal.tsx";

const router = createBrowserRouter(
    createRoutesFromElements(
        <Route path="/" element={<Root/>}>
            <Route path={"sessions"} element={<SessionsPage/>}>
                <Route path={"add"} element={<AddSessionModal/>} action={AddSessionModal.action}/>
            </Route>
        </Route>
    )
)

export default router;