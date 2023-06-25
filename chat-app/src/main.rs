use async_std::io;
use futures::{future::Either, prelude::*, select};
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    gossipsub, identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport
};
use std::{
    collections::hash_map::HashMap,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration
};
use libp2p_quic as quic;

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::async_io::Behaviour,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());

    println!("Local Peer ID: {local_peer_id}");

    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true)).upgrade(upgrade::Version::V1Lazy).authenticate(noise::Config::new(&id_keys).expect("signing libp2p-noise static keypair")).multiplex(yamux::Config::default()).timeout(Duration::from_secs(20)).boxed();
    let quic_transport = quic::async_std::Transport::new(quic::Config::new(&id_keys));
    let transport = OrTransport::new(quic_transport, tcp_transport).map(|either_output, _| match either_output {
        Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
        Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer))
    }).boxed();

}

