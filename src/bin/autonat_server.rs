use clap::Parser;
use futures::prelude::*;
use libp2p::{
    autonat,
    core::upgrade::Version,
    identify, identity,
    multiaddr::Protocol,
    noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Transport,
};
use std::{error::Error, net::Ipv4Addr};

#[derive(Debug, Parser)]
#[clap(name = "autonat")]
struct Opt {
    #[clap(long)]
    listen_port: Option<u16>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let opt = Opt::parse();
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {local_peer_id:?}");

    let transport = tcp::async_io::Transport::default()
        .upgrade(Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();
    let behaviour = Behaviour::new(local_key.public());

    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    swarm.listen_on(
        Multiaddr::empty()
            .with(Protocol::Ip4(Ipv4Addr::UNSPECIFIED))
            .with(Protocol::Tcp(opt.listen_port.unwrap_or(0))),
    )?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on : {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            e => println!("{e:?}"),
        }
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    autonat: autonat::Behaviour,
    identify: identify::Behaviour,
}

impl Behaviour {
    fn new(local_public_key: identity::PublicKey) -> Self {
        Self {
            identify: identify::Behaviour::new(identify::Config::new(
                "ipfs/0.1.0".into(),
                local_public_key.clone(),
            )),
            autonat: autonat::Behaviour::new(
                local_public_key.to_peer_id(),
                autonat::Config {
                    only_global_ips: false,
                    ..Default::default()
                },
            ),
        }
    }
}

#[derive(Debug)]
enum Event {
    Autonat(autonat::Event),
    Identify(identify::Event),
}

impl From<identify::Event> for Event {
    fn from(value: identify::Event) -> Self {
        Self::Identify(value)
    }
}

impl From<autonat::Event> for Event {
    fn from(value: autonat::Event) -> Self {
        Self::Autonat(value)
    }
}
