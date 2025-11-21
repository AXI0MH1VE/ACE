use libp2p::PeerId;

#[derive(Clone, Debug)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub address: String,
}

#[derive(Clone, Default)]
pub struct P2PNetwork {
    pub peers: Vec<PeerInfo>,
}

impl P2PNetwork {
    pub fn discover_peers(&self) -> Vec<PeerInfo> {
        self.peers.clone()
    }

    pub fn add_peer(&mut self, peer: PeerInfo) {
        self.peers.push(peer);
    }
}
