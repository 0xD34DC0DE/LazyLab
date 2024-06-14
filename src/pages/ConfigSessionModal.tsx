import {Box, Button, Dialog, FormGroup, FormLabel, Paper, Stack, TextField, Typography} from "@mui/material";
import {Form, redirect, useActionData, useNavigate, useParams} from "react-router-dom";
import {useState} from "react";
import {useSessionStore} from "../stores/sessions.ts";

interface ConfigSessionModalProps {

}

const ConfigSessionModal = ({}: ConfigSessionModalProps) => {
    const error = useActionData() as Error | undefined;
    const navigate = useNavigate();
    const params = useParams();

    return (
        <Dialog open={true} onClose={() => navigate("..")}>
            <Paper>
                <Box p={2}>
                    <Typography mb={2} variant={"h5"}>New session</Typography>
                    <Form method={"post"}>
                        <Stack spacing={2}>

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

ConfigSessionModal.action = async ({request}: any) => {
    const formData = await request.formData() as FormData;
    const host = formData.get("host")! as string;

    try {
        //await useSessionStore.getState().sessions.find(s => s.id, id)
    } catch (e) {
        return e;
    }

    return redirect("..");
}

export default ConfigSessionModal;
