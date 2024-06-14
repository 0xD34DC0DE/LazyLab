import {Box, Button, Dialog, FormGroup, FormLabel, Paper, Stack, TextField, Typography} from "@mui/material";
import {Form, redirect, useActionData, useNavigate} from "react-router-dom";
import {useState} from "react";
import {useSessionStore} from "../stores/sessions.ts";

interface AddSessionModalProps {

}

const AddSessionModal = ({}: AddSessionModalProps) => {
    const navigate = useNavigate();
    const error = useActionData() as Error | undefined;
    const [host, setHost] = useState<string>("");
    const [user, setUser] = useState<string>("");
    const [password, setPassword] = useState<string>("");

    return (
        <Dialog open={true} onClose={() => navigate("..")}>
            <Paper>
                <Box p={2}>
                    <Typography mb={2} variant={"h5"}>New session</Typography>
                    <Form method={"post"}>
                        <Stack spacing={2}>
                            <FormGroup>
                                <FormLabel>Host</FormLabel>
                                <TextField value={host} name={"host"} onChange={(e) => setHost(e.target.value)}/>
                            </FormGroup>
                            <FormGroup>
                                <FormLabel>User</FormLabel>
                                <TextField value={user} name={"user"} onChange={(e) => setUser(e.target.value)}/>
                            </FormGroup>
                            <FormGroup>
                                <FormLabel>Password</FormLabel>
                                <TextField type={"password"} name={"password"} value={password}
                                           onChange={(e) => setPassword(e.target.value)}/>
                            </FormGroup>
                            <Button type={"submit"}>Connect</Button>
                            {error !== undefined &&
                                <Typography color={"error"}>{error.message}</Typography>
                            }
                        </Stack>
                    </Form>
                </Box>
            </Paper>
        </Dialog>
    );
};

AddSessionModal.action = async ({request}: any) => {
    const formData = await request.formData() as FormData;
    const host = formData.get("host")! as string;
    const user = formData.get("user")! as string;
    const password = formData.get("password")! as string;

    try {
        await useSessionStore.getState().connect(host, user, password);
    } catch (e) {
        return e;
    }

    return redirect("..");
}

export default AddSessionModal;
