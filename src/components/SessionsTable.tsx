import {
    IconButton,
    Paper,
    Stack,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    Tooltip,
    Typography
} from "@mui/material";
import {Link as RouterLink, Outlet} from "react-router-dom";
import CircleIcon from "@mui/icons-material/Circle";
import {useSessionStore} from "../stores/sessions.ts";
import AddIcon from '@mui/icons-material/Add';
import EditIcon from '@mui/icons-material/Edit';

interface SessionsTableProps {

}

const SessionsTable = ({}: SessionsTableProps) => {
    const sessions = useSessionStore((state) => state.sessions);

    return (
        <>
            <Stack>
                <Typography variant={"h5"}>Sessions</Typography>
                <TableContainer component={Paper}>
                    <Table aria-label="sessions table">
                        <TableHead>
                            <TableRow>
                                <TableCell align={"center"} sx={{maxWidth: "2rem"}}>Status</TableCell>
                                <TableCell>User</TableCell>
                                <TableCell>Host</TableCell>
                                <TableCell/>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {sessions.map((session) => (
                                <TableRow
                                    key={session.id}
                                    sx={{'&:last-child td, &:last-child th': {border: 0}}}
                                >
                                    <TableCell align={"center"} sx={{maxWidth: "3rem"}}>
                                        <CircleIcon color={"success"} fontSize={"small"}/>
                                    </TableCell>
                                    <TableCell>{session.user}</TableCell>
                                    <TableCell>{session.host}</TableCell>
                                    <TableCell align={"center"} sx={{maxWidth: "1rem"}}>
                                        <Tooltip title={"Configure session"}>
                                            <IconButton to={`./config/:${session.id}`}
                                                        component={RouterLink}
                                                        color={"primary"}
                                            >
                                                <EditIcon color={"primary"} fontSize={"small"}/>
                                            </IconButton>
                                        </Tooltip>
                                    </TableCell>
                                </TableRow>
                            ))}
                            <TableRow>
                                <TableCell/>
                                <TableCell/>
                                <TableCell/>
                                <TableCell align={"right"} sx={{maxWidth: "2rem"}} size={"small"}>
                                    <Tooltip title={"Add a new session"}>
                                        <IconButton to={"add"}
                                                    component={RouterLink}
                                                    color={"primary"}
                                                    size={"small"}
                                        >
                                            <AddIcon/>
                                        </IconButton>
                                    </Tooltip>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </TableContainer>
            </Stack>
            <Outlet/>
        </>
    );
};

export default SessionsTable;
