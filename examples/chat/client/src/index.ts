import Fluence from 'fluence';
import {seedToPeerId} from "fluence/dist/seed";
import {FluenceClient} from "fluence/dist/fluenceClient";
import {Address, getSignature} from "fluence/dist/address";

const CHAT_BLUEPRINT = "cc5587c8-2d14-4e47-96f8-c3159b0c7206";
const PEER_ID = "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt";
let nodePeerId = "12D3KooWQ8x4SMBmSSUrMzY2m13uzC7UoSyvHaDhTKx7hH8aXxpt"
let MULTIADDR = '/ip4/127.0.0.1/tcp/9001/ws/p2p/' + nodePeerId

const USER_ADDED = "USER_ADDED"
const USER_DELETED = "USER_DELETED"
const MESSAGE = "MESSAGE"
const CHAT = "CHAT"
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
    members: Member[]

    constructor(client: FluenceClient, serviceId: string, name: string, members: Member[]) {
        this.client = client;
        this.name = name;
        this.serviceId = serviceId;
        this.members = members;
        client.subscribe((args: any, target: Address, replyTo: Address, moduleId, fname) => {
            console.log(`MODULE: ${moduleId}, fname: ${fname}`)
            if (moduleId === CHAT) {
              switch(fname) {
                  case USER_ADDED:
                      let member: Member = {
                          clientId: args.member.clientId,
                          relay: args.member.relay,
                          sig: args.member.sig,
                          name: args.member.name
                      }
                      console.log(`Member added to ${this.name}: ` + JSON.stringify(member))
                      this.addMember(member);
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
        let user = this.client.selfPeerIdStr;
        this.name = name;
        let result = await client.callService(PEER_ID, this.serviceId, USER_LIST, [user, name, user], "change_name")
        console.log(result)
    }

    deleteMember(clientId: string) {
        this.members = this.members.filter(m => m.clientId === clientId)
    }

    addMember(member: Member) {
        if (member.clientId !== this.client.selfPeerIdStr) {
            this.members.push(member)
        }
    }

    async deleteUser(user: string) {
        let result = await client.callService(PEER_ID, this.serviceId, USER_LIST, [user, user], "delete")
        this.deleteMember(user)
        console.log(result)
    }

    async getHistory(): Promise<any> {
        return await client.callService(PEER_ID, this.serviceId, HISTORY, [], "get_all")
    }

    async sendMessage(msg: string) {
        let result = await client.callService(PEER_ID, this.serviceId, HISTORY, [this.client.selfPeerIdStr, msg], "add")
        for (const member of this.members) {
            console.log("send to: " + JSON.stringify(member))
            await client.fireClient(member.relay, member.clientId, member.sig, CHAT, {clientId: this.client.selfPeerIdStr, message: msg}, MESSAGE)
        }
        console.log("result send message: " + JSON.stringify(result))
    }
}

async function createChat(name: string, seed?: string) {
    let client = await connect(seed);
    let sig = getSignature(client.connection.replyTo)
    if (sig) {
        let serviceId = await client.createService(CHAT_BLUEPRINT);
        let result = await client.callService(PEER_ID, serviceId, USER_LIST, [client.selfPeerIdStr, nodePeerId, sig, name], "join")
        console.log("serviceId: " + serviceId)
        return new FluenceChat(client, serviceId, name, []);
    } else {
        console.error("Signature should be presented.")
    }

}

async function joinChat(name: string, chatId: string, seed?: string, relayAddress?: string) {
    let client = await connect(seed, relayAddress);

    let sig = getSignature(client.connection.replyTo)

    if (sig) {
        let members = await getMembers(client, chatId);
        let result = await client.callService(PEER_ID, chatId, USER_LIST, [client.selfPeerIdStr, nodePeerId, sig, name], "join")
        console.log(result)
        for (const member of members) {
            await client.fireClient(member.relay, member.clientId, member.sig, CHAT, {member: {clientId: client.selfPeerIdStr, sig: sig, relay: nodePeerId, name: name}}, USER_ADDED)
        }
        return new FluenceChat(client, chatId, name, members);
    } else {
        console.error("Signature should be presented.")
    }



}

async function getMembers(client: FluenceClient, chatId: string): Promise<Member[]> {
    let membersStr = (await client.callService(nodePeerId, chatId, USER_LIST, {}, "get_users")).result as string
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
    return new Promise( resolve => setTimeout(resolve, ms) );
}

async function scenario() {
    console.log("start")
    let creator = await createChat("papa")
    console.log("chat created")
    await delay(1000)
    let user = await joinChat("mama", creator.serviceId)
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

async function connect(seed?: string, relayAddress?: string): Promise<FluenceClient> {

    let pid;
    if (seed) {
        pid = await seedToPeerId(seed);
    } else {
        pid = await Fluence.generatePeerId();
    }
    // Fluence.setLogLevel('trace')
    console.log("PID = " + pid.toB58String())
    client = await Fluence.connect(relayAddress ?? MULTIADDR, pid);

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
    // await cl.addBlueprint("chat", ["sqlite", "history", USER_LIST])
    // let serv = await cl.createService("cc5587c8-2d14-4e47-96f8-c3159b0c7206");
    // console.log(serv)
}

