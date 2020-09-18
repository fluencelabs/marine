import {FluenceClient} from "fluence/dist/fluenceClient";
import {Address, getSignature} from "fluence/dist/address";

const NAME_CHANGED = "NAME_CHANGED"
const RELAY_CHANGED = "RELAY_CHANGED"
export const USER_ADDED = "USER_ADDED"
const USER_DELETED = "USER_DELETED"
const MESSAGE = "MESSAGE"
export const MODULE_CHAT = "CHAT"
const HISTORY = "history"
export const USER_LIST = "user-list"

export interface Member {
    clientId: string,
    relay: string,
    sig: string,
    name: string
}

export class FluenceChat {

    client: FluenceClient
    serviceId: string
    name: string
    relay: string
    chatPeerId: string
    members: Member[]

    constructor(client: FluenceClient, serviceId: string, peerId: string, name: string, relay: string, members: Member[]) {
        this.client = client;
        this.name = name;
        this.serviceId = serviceId;
        this.members = members.filter(m => m.clientId !== this.client.selfPeerIdStr);
        this.relay = relay;
        this.chatPeerId = peerId;
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
                        member = this.members.filter(m => m.clientId === args.clientId)[0];
                        if (member) {
                            member.name = args.name;
                            this.addMember(member);
                            console.log("Name changed: " + args.clientId)
                        } else {
                            console.log("Cannot change name. There is no member: " + JSON.stringify(member))
                        }
                        break;
                    case RELAY_CHANGED:
                        member = this.members.filter(m => m.clientId === args.clientId)[0];
                        this.addMember(member);
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
                        console.log("message received to " + this.name)
                        let m = this.members.find(m => m.clientId === args.clientId)
                        if (m) {
                            console.log(`${m.name}: ${args.message}`)
                        }
                        break;
                    default:
                        console.log("Unexpected fname: " + fname)
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
        await this.client.callService(this.chatPeerId, this.serviceId, USER_LIST, [clientId, name, clientId], "change_name")
        await this.sendToAll({clientId: clientId, name: name}, NAME_CHANGED)
    }

    async publishRelay() {
        let clientId = this.client.selfPeerIdStr;
        let relay = this.client.connection.nodePeerId.toB58String();
        let sig = getSignature(this.client.connection.replyTo)
        await this.client.callService(this.chatPeerId, this.serviceId, USER_LIST, [clientId, relay, sig, clientId], "change_relay")
        await this.sendToAll({clientId: clientId, relay: relay, sig: sig}, RELAY_CHANGED)
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
        await this.client.callService(this.chatPeerId, this.serviceId, USER_LIST, [user, user], "delete")
        this.deleteMember(user)
    }

    async getHistory(): Promise<any> {
        return await this.client.callService(this.chatPeerId, this.serviceId, HISTORY, [], "get_all")
    }

    async sendToAll(args: any, fname: string) {
        for (const member of this.members) {
            console.log(`send command '${fname}' to: ` + JSON.stringify(member))
            await this.client.fireClient(member.relay, member.clientId, member.sig, MODULE_CHAT, args, fname)
        }
    }

    async sendMessage(msg: string) {
        await this.client.callService(this.chatPeerId, this.serviceId, HISTORY, [this.client.selfPeerIdStr, msg], "add")
        await this.sendToAll({
            clientId: this.client.selfPeerIdStr,
            message: msg
        }, MESSAGE);
    }
}