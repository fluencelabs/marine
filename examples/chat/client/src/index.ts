import Fluence from 'fluence';
import {seedToPeerId} from "fluence/dist/seed";
import {FluenceClient} from "fluence/dist/fluenceClient";
import {Address, getSignature} from "fluence/dist/address";

const CHAT_BLUEPRINT = "cc5587c8-2d14-4e47-96f8-c3159b0c7206";
const CHAT_PEER_ID = "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt"

let relays = [
    {
        multiaddr: "/ip4/127.0.0.1/tcp/9001/ws/p2p/12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt",
        peerId: "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt"
    },
    {
        multiaddr: "/ip4/127.0.0.1/tcp/9002/ws/p2p/12D3KooWGGv3ZkcbxNtM7jPzrtgxprd2Ws4zm9z1JkNSUwUgyaUN",
        peerId: "12D3KooWGGv3ZkcbxNtM7jPzrtgxprd2Ws4zm9z1JkNSUwUgyaUN"
    },
    {
        multiaddr: "/ip4/127.0.0.1/tcp/9003/ws/p2p/12D3KooWSGS1XxVx2fiYM5U66HKtF81ypbzA3v71jLBUVLZSNSNi",
        peerId: "12D3KooWSGS1XxVx2fiYM5U66HKtF81ypbzA3v71jLBUVLZSNSNi"
    }
]

const NAME_CHANGED = "NAME_CHANGED"
const RELAY_CHANGED = "RELAY_CHANGED"
const USER_ADDED = "USER_ADDED"
const USER_DELETED = "USER_DELETED"
const MESSAGE = "MESSAGE"
const MODULE_CHAT = "CHAT"
const HISTORY = "history"
const USER_LIST = "user-list"

let client: FluenceClient;

interface Member {
    clientId: string,
    relay: string,
    sig: string,
    name: string
}

class FluenceChat {

    client: FluenceClient
    serviceId: string
    name: string
    relay: string
    members: Member[]

    constructor(client: FluenceClient, serviceId: string, name: string, relay: string, members: Member[]) {
        this.client = client;
        this.name = name;
        this.serviceId = serviceId;
        this.members = members.filter(m => m.clientId !== this.client.selfPeerIdStr);
        this.relay = relay;
        client.subscribe((args: any, target: Address, replyTo: Address, moduleId, fname) => {
            console.log(`MODULE: ${moduleId}, fname: ${fname}`)
            let member: Member;
            if (moduleId === MODULE_CHAT) {
                switch (fname) {
                    case USER_ADDED:
                        member = {
                            clientId: args.member.clientId,
                            relay: args.member.relay,
                            sig: args.member.sig,
                            name: args.member.name
                        }
                        console.log(`Member added to ${this.name}: ` + JSON.stringify(member))
                        this.addMember(member);
                        break;
                    case NAME_CHANGED:
                        member = this.members.filter(m => m.clientId === args.clientId)[0]
                        if (member) {
                            member.name = args.name;
                            this.members.push(member);
                            console.log("Name changed: " + args.clientId)
                        } else {
                            console.log("Cannot change name. There is no member: " + JSON.stringify(member))
                        }
                        break;
                    case RELAY_CHANGED:
                        member = this.members.filter(m => m.clientId === args.clientId)[0]
                        if (member) {
                            member.relay = args.relay;
                            member.sig = args.sig;
                            this.members.push(member);
                            console.log("Relay changed: " + args.clientId)
                        } else {
                            console.log("Cannot change relay. There is no member: " + JSON.stringify(member))
                        }
                        break;
                    case USER_DELETED:
                        console.log("Member deleted: " + args.clientId)
                        this.deleteMember(args.clientId);
                        break;
                    case MESSAGE:
                        console.log("message received")
                        console.log(args)
                        console.log(this.members)
                        let m = this.members.find(m => m.clientId === args.clientId)
                        if (m) {
                            console.log(`${m.name}: ${args.message}`)
                        }
                        break;
                    default:
                        console.log("WHAT?!  " + fname)
                        break;
                }
            } else {
                console.log("Unhandled moduleId: " + moduleId + ", args: " + JSON.stringify(args));
            }

            return false;
        })
    }

    async changeName(name: string) {
        let clientId = this.client.selfPeerIdStr;
        this.name = name;
        let result = await client.callService(CHAT_PEER_ID, this.serviceId, USER_LIST, [clientId, name, clientId], "change_name")
        await this.sendToAll({clientId: clientId, name: name}, NAME_CHANGED)
        console.log(result)
    }

    async publishRelay() {
        let clientId = this.client.selfPeerIdStr;
        let relay = client.connection.nodePeerId.toB58String();
        let sig = getSignature(client.connection.replyTo)
        let result = await client.callService(CHAT_PEER_ID, this.serviceId, USER_LIST, [clientId, relay, sig, clientId], "change_relay")
        await this.sendToAll({clientId: clientId, relay: relay, sig: sig}, RELAY_CHANGED)
        console.log(result)
    }

    async reconnect(multiaddr: string) {
        await this.client.connect(multiaddr);
        await this.publishRelay();
    }

    deleteMember(clientId: string) {
        this.members = this.members.filter(m => m.clientId !== clientId)
    }

    addMember(member: Member) {
        if (member.clientId !== this.client.selfPeerIdStr) {
            this.members = this.members.filter(m => m.clientId !== member.clientId)
            this.members.push(member)
        }
    }

    async deleteUser(user: string) {
        let result = await client.callService(CHAT_PEER_ID, this.serviceId, USER_LIST, [user, user], "delete")
        this.deleteMember(user)
        console.log(result)
    }

    async getHistory(): Promise<any> {
        return await client.callService(CHAT_PEER_ID, this.serviceId, HISTORY, [], "get_all")
    }

    async sendToAll(args: any, fname: string) {
        for (const member of this.members) {
            console.log(`send command '${fname}' to: ` + JSON.stringify(member))
            await client.fireClient(member.relay, member.clientId, member.sig, MODULE_CHAT, args, fname)
        }
    }

    async sendMessage(msg: string) {
        let result = await client.callService(CHAT_PEER_ID, this.serviceId, HISTORY, [this.client.selfPeerIdStr, msg], "add")
        await this.sendToAll({
            clientId: this.client.selfPeerIdStr,
            message: msg
        }, MESSAGE);
        console.log("result send message: " + JSON.stringify(result))
    }
}

async function createChat(name: string, relay: string, relayAddress: string, seed?: string) {
    let client = await connect(relayAddress, seed);
    let sig = getSignature(client.connection.replyTo)
    if (sig) {
        let serviceId = await client.createService(CHAT_BLUEPRINT, CHAT_PEER_ID);
        delay(1000)
        let result = await client.callService(CHAT_PEER_ID, serviceId, USER_LIST, [client.selfPeerIdStr, relay, sig, name], "join")
        console.log("serviceId: " + serviceId)
        return new FluenceChat(client, serviceId, name, relay,[]);
    } else {
        console.error("Signature should be presented.")
    }

}

async function connectToChat(chatId: string, relay: string, relayAddress: string, seed: string): Promise<FluenceChat> {
    let client = await connect(relayAddress, seed);
    let sig = getSignature(client.connection.replyTo)

    if (sig) {
        let members = await getMembers(client, chatId);
        let you = members.find(m => m.clientId === client.selfPeerIdStr)
        if (you) {
            let chat = new FluenceChat(client, chatId, name, relay, members);
            await chat.publishRelay();
            return chat;
        } else {
            console.error("You are not in chat. Use 'join'")
            throw new Error("You are not in chat. Use 'join'")
        }
    } else {
        console.error("Signature should be presented.")
        throw new Error("Signature should be presented.")
    }
}

async function joinChat(name: string, chatId: string, relay: string, relayAddress: string, seed?: string): Promise<FluenceChat> {
    let client = await connect(relayAddress, seed);

    let sig = getSignature(client.connection.replyTo)

    if (sig) {
        let members = await getMembers(client, chatId);
        let result = await client.callService(CHAT_PEER_ID, chatId, USER_LIST, [client.selfPeerIdStr, relay, sig, name], "join")
        console.log(result)
        for (const member of members) {
            await client.fireClient(member.relay, member.clientId, member.sig, MODULE_CHAT, {
                member: {
                    clientId: client.selfPeerIdStr,
                    sig: sig,
                    relay: relay,
                    name: name
                }
            }, USER_ADDED)
        }
        return new FluenceChat(client, chatId, name, relay, members);
    } else {
        console.error("Signature should be presented.")
        throw new Error("Signature should be presented.")
    }
}

async function getMembers(client: FluenceClient, chatId: string): Promise<Member[]> {
    let membersStr = (await client.callService(CHAT_PEER_ID, chatId, USER_LIST, {}, "get_users")).result as string
    let members: Member[] = []
    console.log("MEMBER RRAW")
    console.log(membersStr)
    let membersRaw = membersStr.split("|")
    membersRaw.forEach((v) => {
        let memberRaw = v.split(",")
        let member: Member = {
            clientId: memberRaw[1],
            relay: memberRaw[2],
            sig: memberRaw[3],
            name: memberRaw[4]
        }
        members.push(member);
    })
    console.log(members)
    return members;
}

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function scenario() {
    console.log("start")
    let creator = await createChat("papa", relays[1].peerId, relays[1].multiaddr)
    console.log("chat created")
    await delay(1000)
    let user = await joinChat("mama", creator.serviceId, relays[2].peerId, relays[2].multiaddr)
    console.log("user joined")

    await delay(1000)

    console.log("creator send message")
    await creator.sendMessage("hello")
    await delay(1000)
    console.log("user send message")
    await user.sendMessage("hi")

    /**/

    // let h1 = await creator.getHistory();
    // console.log("h1: " + JSON.stringify(h1))
    // let h2 = await user.getHistory();
    // console.log("h2: " + JSON.stringify(h2))
    // await getMembers(user.client, user.serviceId)
}

declare global {
    interface Window {
        joinChat: any;
        createChat: any;
        scenario: any;
    }
}

window.joinChat = joinChat;
window.createChat = createChat;
window.scenario = scenario;

async function connect(relayAddress: string, seed?: string): Promise<FluenceClient> {

    let pid;
    if (seed) {
        pid = await seedToPeerId(seed);
    } else {
        pid = await Fluence.generatePeerId();
    }
    // Fluence.setLogLevel('trace')
    console.log("PID = " + pid.toB58String())
    client = await Fluence.connect(relayAddress, pid);

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
    let cl = await Fluence.connect(relays[0].multiaddr, pid);
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
    // await cl.addBlueprint("chat", ["sqlite", "history", USER_LIST])
    // let serv = await cl.createService("cc5587c8-2d14-4e47-96f8-c3159b0c7206");
    // console.log(serv)
}

