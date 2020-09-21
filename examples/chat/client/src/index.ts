import Fluence from 'fluence';
import {seedToPeerId} from "fluence/dist/seed";
import {FluenceClient} from "fluence/dist/fluenceClient";
import {getSignature} from "fluence/dist/address";
import {FluenceChat, HISTORY, Member, MODULE_CHAT, USER_ADDED, USER_LIST} from "./fluenceChat";

// change these constants in different environment
const CHAT_BLUEPRINT = "cc5587c8-2d14-4e47-96f8-c3159b0c7206";
const CHAT_PEER_ID = "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt"

// parameters from `fluence-playground` local network
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

async function createChat(name: string, relay: string, relayAddress: string, seed?: string): Promise<FluenceChat> {
    let client = await connect(relayAddress, seed);
    let sig = getSignature(client.connection.replyTo)
    if (sig) {
        let serviceId = await client.createService(CHAT_BLUEPRINT, CHAT_PEER_ID);
        delay(1000)
        // TODO: implement a real signature check in the future
        await client.callService(CHAT_PEER_ID, serviceId, USER_LIST, [client.selfPeerIdStr, relay, sig, name], "join")
        console.log("serviceId: " + serviceId)
        return new FluenceChat(client, serviceId, CHAT_PEER_ID, name, relay,[]);
    } else {
        console.error("Signature should be presented.")
        throw new Error("Signature should be presented.")
    }

}

async function connectToChat(chatId: string, relay: string, relayAddress: string, seed: string): Promise<FluenceChat> {
    let client = await connect(relayAddress, seed);
    let sig = getSignature(client.connection.replyTo)

    if (sig) {
        let members = await getMembers(client, chatId);
        let you = members.find(m => m.clientId === client.selfPeerIdStr)
        if (you) {
            let chat = new FluenceChat(client, chatId, CHAT_PEER_ID, name, relay, members);
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
        await client.callService(CHAT_PEER_ID, chatId, USER_LIST, [client.selfPeerIdStr, relay, sig, name], "join")
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
        return new FluenceChat(client, chatId, CHAT_PEER_ID, name, relay, members);
    } else {
        console.error("Signature should be presented.")
        throw new Error("Signature should be presented.")
    }
}

async function getMembers(client: FluenceClient, chatId: string): Promise<Member[]> {
    let membersStr = (await client.callService(CHAT_PEER_ID, chatId, USER_LIST, {}, "get_users")).result as string
    let members: Member[] = []
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
    return members;
}

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function scenario() {
    console.log("start")
    let creator = await createChat("Alice", relays[1].peerId, relays[1].multiaddr)
    console.log("chat created")
    await delay(1000)
    let user = await joinChat("Bob", creator.serviceId, relays[2].peerId, relays[2].multiaddr)
    console.log("user joined")

    await delay(1000)

    console.log("creator send message")
    await creator.sendMessage("hello")
    await delay(1000)
    console.log("user send message")
    await user.sendMessage("hi")

    await user.reconnect(relays[0].multiaddr);

    console.log("creator send second message")
    await creator.sendMessage("how ya doin")
    await delay(1000)
    console.log("user send second message")
    await user.sendMessage("ama fine")

    await user.changeName("John")

    console.log("creator send second message")
    await creator.sendMessage("what is your name?")
    await delay(1000)
    console.log("user send second message")
    await user.sendMessage("Not Bob")

    let h1 = await creator.getHistory();
    console.log("history creator: " + JSON.stringify(h1))
    let h2 = await user.getHistory();
    console.log("history user: " + JSON.stringify(h2))
    let members = await getMembers(user.client, user.serviceId)
    console.log("members: " + JSON.stringify(members))
}

declare global {
    interface Window {
        joinChat: any;
        createChat: any;
        scenario: any;
        connectToChat: any;
    }
}

window.joinChat = joinChat;
window.createChat = createChat;
window.scenario = scenario;
window.connectToChat = connectToChat;
// Fluence.setLogLevel('trace')

async function connect(relayAddress: string, seed?: string): Promise<FluenceClient> {
    let pid;
    if (seed) {
        pid = await seedToPeerId(seed);
    } else {
        pid = await Fluence.generatePeerId();
    }

    console.log("PID = " + pid.toB58String())

    return await Fluence.connect(relayAddress, pid);
}

// publishes a blueprint for chat application and shows its id
async function publishBlueprint() {
    let pid = await Fluence.generatePeerId();
    let cl = await Fluence.connect(relays[0].multiaddr, pid);

    let blueprintId = await cl.addBlueprint("chat", ["sqlite", HISTORY, USER_LIST])
    console.log(`BLUEPRINT ID: ${blueprintId}`)
}

