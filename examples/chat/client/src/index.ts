import Fluence from 'fluence';
import {seedToPeerId} from "fluence/dist/seed";
import {FluenceClient} from "fluence/dist/fluenceClient";
import {Address} from "fluence/dist/address";
import {WASM} from "../hist";

const CHAT_BLUEPRINT = "";
const PEER_ID = "";
let nodePeerId = "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt"
let MULTIADDR = '/ip4/127.0.0.1/tcp/9001/ws/p2p/' + nodePeerId

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

// start();
publishBlueprint();

async function publishBlueprint() {
    let pid = await Fluence.generatePeerId();
    console.log("1111")
    let cl = await Fluence.connect(MULTIADDR, pid);
    console.log("2222")
    let bps = await cl.getAvailableBlueprints()
    console.log("333")
    console.log(bps)
    let services = await cl.getActiveInterfaces()
    console.log("333")
    console.log(services)
    let modules = await cl.getAvailableModules()
    console.log("4444")
    console.log(modules)
    // await cl.addBlueprint("chat", ["history", "user-list", "sqlite"])
    let serv = await cl.createService("c51d5527-894e-43c5-aae7-76d4b5ad2a33");
    console.log(serv)
}

