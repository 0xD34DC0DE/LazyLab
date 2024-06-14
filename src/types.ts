import {invoke} from "@tauri-apps/api";

class Session {
    private _id: number = -1;
    public readonly host: string;
    public readonly user: string;

    private constructor(host: string, user: string) {
        this.host = host;
        this.user = user;
    }

    public static async connect(host: string, user: string, password: string): Promise<Session> {
        const session = new Session(host, user);
        await session.connect(password);
        return session;
    }

    public static async sessions(): Promise<Session[]> {
        type SessionInfo = {addrs: [string, number], user: string, };
        const sessions_info = await invoke<SessionInfo[]>("get_sessions");
        return sessions_info.map((info) => {
            const [host, _] = info.addrs;
            return new Session(host, info.user);
        });
    }

    get id(): number { return this._id;}

    public async connect(password: string) {
        await invoke<number>("start_session", {
                req: {
                    addr: [this.host, 22],
                    user: this.user,
                    password
                }
            }
        ).then(id => this._id = id);
    }
}

export default Session;