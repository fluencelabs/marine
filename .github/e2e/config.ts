export type Node = { peerId: string; multiaddr: string };
export const ci: Node[] = [
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9991/ws/p2p/12D3KooWBM3SdXWqGaawQDGQ6JprtwswEg3FWGvGhmgmMez1vRbR',
    peerId: '12D3KooWBM3SdXWqGaawQDGQ6JprtwswEg3FWGvGhmgmMez1vRbR'
  },
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9992/ws/p2p/12D3KooWMTX8WyWBykinniE7sZnQjT8AXv6XX8rNYreB3GCSSE5f',
    peerId: '12D3KooWMTX8WyWBykinniE7sZnQjT8AXv6XX8rNYreB3GCSSE5f'
  },
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9993/ws/p2p/12D3KooWRT8V5awYdEZm6aAV9HWweCEbhWd7df4wehqHZXAB7yMZ',
    peerId: '12D3KooWRT8V5awYdEZm6aAV9HWweCEbhWd7df4wehqHZXAB7yMZ'
  },
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9994/ws/p2p/12D3KooWBzLSu9RL7wLP6oUowzCbkCj2AGBSXkHSJKuq4wwTfwof',
    peerId: '12D3KooWBzLSu9RL7wLP6oUowzCbkCj2AGBSXkHSJKuq4wwTfwof'
  },
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9995/ws/p2p/12D3KooWBf6hFgrnXwHkBnwPGMysP3b1NJe5HGtAWPYfwmQ2MBiU',
    peerId: '12D3KooWBf6hFgrnXwHkBnwPGMysP3b1NJe5HGtAWPYfwmQ2MBiU'
  },
  {
    multiaddr: '/ip4/127.0.0.1/tcp/9996/ws/p2p/12D3KooWPisGn7JhooWhggndz25WM7vQ2JmA121EV8jUDQ5xMovJ',
    peerId: '12D3KooWPisGn7JhooWhggndz25WM7vQ2JmA121EV8jUDQ5xMovJ'
  }
];

export const config = {
  relays: ci,
  externalAddressesRelay1: ['/ip4/127.0.0.1/tcp/7771', '/ip4/127.0.0.1/tcp/9991/ws'],
  externalAddressesRelay2: ['/ip4/127.0.0.1/tcp/7772', '/ip4/127.0.0.1/tcp/9992/ws'],
  tryCatchError:
    "Local service error, ret_code is 1, error message is '\"Service with id 'unex' not found (function getStr)\"'",
};

export const isEphemeral = false;
