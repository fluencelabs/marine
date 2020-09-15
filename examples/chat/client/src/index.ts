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

    constructor(client: FluenceClient, serviceId: string) {
        this.client = client;
        this.serviceId = serviceId;
        client.subscribe((args: any, target: Address, replyTo: Address) => {
            console.log(args);
            return false;
        })
    }

    async getHistory() {
        let result = await client.callService(PEER_ID, this.serviceId, "history", {}, "get_history")
    }

    async sendMessage() {
        let result = await client.callService(PEER_ID, this.serviceId, "history", {}, "send_meesage")
    }
}

async function createChat(name: string, seed?: string) {
    let client = await connect(name, seed);
    let serviceId = await client.createService(CHAT_BLUEPRINT);
    let chat = new FluenceChat(client, serviceId);

}

async function joinChat(name: string, chatId: string, seed?: string) {
    let client = await connect(name, seed);
    let chat = new FluenceChat(client, chatId);
}

async function connect(name: string, seed?: string): Promise<FluenceClient> {

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

