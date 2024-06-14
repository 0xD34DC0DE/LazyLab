import {
    AppBar as MuiAppBar,
    Drawer,
    IconButton,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText,
    Toolbar, Typography
} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import Logo from "./Logo.tsx";
import VpnKeyIcon from '@mui/icons-material/VpnKey';
import {useState} from "react";
import {Link} from "react-router-dom";

interface AppBarProps {

}

const AppBar = ({}: AppBarProps) => {
    const [open, setOpen] = useState<boolean>(false);

    return (
        <MuiAppBar position={"static"}>
            <Toolbar>
                <IconButton size={"large"} edge={"start"} onClick={() => setOpen(true)}>
                    <MenuIcon/>
                </IconButton>
                <Logo/>
                <Typography variant={"h6"} ml={1}>LazyLab</Typography>
            </Toolbar>
            <Drawer open={open} anchor={"left"} onClose={() => setOpen(false)}>
                <List>
                    <ListItem disablePadding>
                        <ListItemButton component={Link} to={"sessions"} onClick={() => setOpen(false)}>
                            <ListItemIcon><VpnKeyIcon/></ListItemIcon>
                            <ListItemText>Sessions</ListItemText>
                        </ListItemButton>
                    </ListItem>
                </List>
            </Drawer>
        </MuiAppBar>
    );
};

export default AppBar;
