import Fluence from 'fluence';
import {seedToPeerId} from "fluence/dist/seed";
import {FluenceClient} from "fluence/dist/fluenceClient";
import {Address} from "fluence/dist/address";

const MULTIADDR = "";
const CHAT_BLUEPRINT = "";
const PEER_ID = "";

let client: FluenceClient;

class FluenceChat {

    client: FluenceClient
    serviceId: string
    name: string

    constructor(client: FluenceClient, serviceId: string, name: string) {
        this.client = client;
        this.name = name;
        this.serviceId = serviceId;
        client.subscribe((args: any, target: Address, replyTo: Address) => {
            console.log(args);
            return false;
        })
    }

    async changeName(name: string) {
        let user = this.client.selfPeerIdStr;
        this.name = name;
        let result = await client.callService(PEER_ID, this.serviceId, "user-list", [user, name, user], "change_name")
        console.log(result)
    }

    async deleteUser(user: string) {
        let result = await client.callService(PEER_ID, this.serviceId, "user-list", [user, user], "delete")
        console.log(result)
    }

    async addUser(user: string, name: string) {
        let result = await client.callService(PEER_ID, this.serviceId, "user-list", [user, name, user], "add")
        console.log(result)
    }

    async getHistory() {
        let result = await client.callService(PEER_ID, this.serviceId, "history", [], "get_all")
        console.log(result)
    }

    async sendMessage(msg: string) {
        let result = await client.callService(PEER_ID, this.serviceId, "history", [this.client.selfPeerIdStr, msg], "add")
        console.log(result)
    }
}

async function createChat(name: string, seed?: string) {
    let client = await connect(seed);
    let serviceId = await client.createService(CHAT_BLUEPRINT);
    let chat = new FluenceChat(client, serviceId, name);

}

async function joinChat(name: string, chatId: string, seed?: string) {
    let client = await connect(seed);
    let chat = new FluenceChat(client, chatId, name);
}

async function connect(seed?: string): Promise<FluenceClient> {

    let pid;
    if (seed) {
        pid = await seedToPeerId(seed);
    } else {
        pid = await Fluence.generatePeerId();
    }

    client = await Fluence.connect(MULTIADDR, pid);
    return client;
}

async function start() {
    console.log("hello world");
    let pid = await Fluence.generatePeerId();
    console.log(pid.toB58String())
}

start();

