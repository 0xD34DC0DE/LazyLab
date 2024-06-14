import {create} from 'zustand'
import Session from "../types.ts";

interface SessionStore {
    sessions: Session[];
    connect: (host: string, user: string, password: string) => Promise<void>;
    reconnect: (session: Session, password: string) => Promise<void>;
    remove: (session: Session) => void;
}

export const useSessionStore = create<SessionStore>()(
    (set) => ({
        sessions: [],
        connect: async (host, user, password) => {
            const session = await Session.connect(host, user, password);
            set((state) => ({sessions: [...state.sessions, session]}));
        },
        reconnect: async (session, password) => {
            await session.connect(password);
        },
        remove: (session) => {
            set((state) => ({sessions: state.sessions.filter((s) => s !== session)}));
        }
    })
);

Session.sessions().then((sessions) => {
    useSessionStore.setState({sessions});
});
