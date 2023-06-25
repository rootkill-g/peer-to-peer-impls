use crate::endpoint::{Config, QuinnConfig, ToEndpoint};
use crate::hole_punching::hole_puncher;
use crate::provider::Provider;
use crate::{endpoint, Connecting, Connection, Error};

use futures::channel::{mpsc, oneshot};
use futures::future::{BoxFuture, Either};
use futures::ready;
use futures::stream::StreamExt;
use futures::{prelude::*, stream::SelectAll};

use if_watch::IfEvent;

use libp2p_core::{
    multiaddr::{Multiaddr, Protocol},
    transport::{ListnerId, TransportError, TransportEvent},
    Transport,
};
use libp2p_identity::PeerId,
use std::collections::hash_map::{DefaultHasher, Entry};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll, Waker},
};


