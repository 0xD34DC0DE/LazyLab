import {Grid} from "@mui/material";
import SessionsTable from "../components/SessionsTable.tsx";

interface SessionsPageProps {

}

const SessionsPage = ({}: SessionsPageProps) => {
    return (
        <Grid container item justifySelf={"center"} display={"flex"} alignSelf={"center"}
              alignItems={"center"} justifyContent={"center"}
        >
            <Grid container item xs={4} gap={1}>
                <Grid item xs={8}>
                    <SessionsTable/>
                </Grid>
            </Grid>
        </Grid>
    );
};


export default SessionsPage;
