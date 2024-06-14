import {CssBaseline, ThemeProvider, Grid} from "@mui/material";
import {Outlet} from "react-router-dom";
import theme from "./theme.ts";
import AppBar from "./components/AppBar.tsx";

interface RootProps {

}

const Root = ({}: RootProps) => {
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline>
                <Grid container display={"flex"} flexDirection={"column"} height={"100vh"}>
                    <AppBar/>
                    <Grid container display={"flex"} flexGrow={1}>
                        <Outlet/>
                    </Grid>
                </Grid>
            </CssBaseline>
        </ThemeProvider>
    );
};

export default Root;
