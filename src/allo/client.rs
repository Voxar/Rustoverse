extern crate enet as ENET;
use ENET::Enet;
use std::net::Ipv4Addr;
use serde_json::json;

use super::shared::{Intent, Interaction, Identity, Avatar};
use super::wire::WireObject;


#[derive(Debug)]
pub struct ConnectionError;


pub struct Client {
  allospace: enet::Address,
  identity: Identity,
  avatar: Avatar,
  
  state: ClientState,
  
  connection: enet::Host<()>
}

#[derive(Debug, Clone, Copy)]
enum ClientState {
  Initial,
  Interrupted(InterruptionState),
  Nominal,
}

#[derive(Debug, Clone, Copy)]
enum InterruptionState {
  Connecting,
  TryLater,
}



impl Client {
  pub fn new(host: String, identity: Identity, avatar: Avatar) -> Client {
    let network = Enet::new().unwrap();
    let host = network.create_host::<()>(
      None, // local interface
      1, // simultanious connections
      enet::ChannelLimit::Limited(2),
      enet::BandwidthLimit::Unlimited,
      enet::BandwidthLimit::Unlimited
    ).unwrap();
    
    Client {
      allospace: enet::Address::new(Ipv4Addr::LOCALHOST, 21337),
      identity: identity,
      avatar: avatar,
      state: ClientState::Initial,
      connection: host,
    }
  }
  
  pub fn poll(&mut self) {
    
    match self.state {
      ClientState::Initial => {
        println!("Initial");
        self.connection.connect(&self.allospace, 2, 0).expect("failed starting connect");
        
        // move to interrupted
        self.state = ClientState::Interrupted(InterruptionState::Connecting); 
      }
      
      ClientState::Interrupted(state) => {
        println!("Interrupted {:?}", state);
        self.handle_state_interrupted(state);
      }
      
      ClientState::Nominal => {
        // poll network and parse messages
        let ref event = self.connection.service(1000).expect("service failed");
        match event {
          Some(enet::Event::Receive { sender, channel_id, packet } ) => {
            println!("Received {:?} from {:?}", packet, sender);
            let data = packet.data();
            println!("{:?}", std::str::from_utf8(data).expect("not utf8 string"));
          },
          
          Some(enet::Event::Disconnect(peer, reason)) => {
            println!("{:?} got disconnected: {:?}", peer, reason);
            self.state = ClientState::Interrupted(InterruptionState::Connecting);
          },
          
          None => return,
          
          Some(x) => {
            println!("Other: {:?}", x);
          },
        };
      }
    }    
    
  }
  
  fn handle_state_interrupted(&mut self, state: InterruptionState) {
    match state {
      InterruptionState::Connecting => {
        // connect to remote
        let event = self.connection.service(1000)
        .expect("service failed");
        
        match event {
          None => return,
          Some(enet::Event::Connect(ref peer)) => {
            println!("We are connected");
            
            let body = json!([
              "announce",
              "version",
              1,
              "identity",
              {
                "display_name": "Rusty Voxar",
              },
              "spawn_avatar",
              {
                "id": "randomstring",
              },
            ]);
              
              let int = Interaction {
                kind: "request".to_string(),
                sender_entity_id: "".to_string(),
                receiver_entity_id: "place".to_string(),
                request_id: "ANN0".to_string(),
                body: body.to_string(),
              };
              
              let string = format!("{}{}", int.to_wire(), "\n");
              println!("JSON Balboa: {}", string);
              let data = string.as_bytes();
              let packet = enet::Packet::new(data, enet::PacketMode::ReliableSequenced)
              .expect("packet construct error");
              peer.clone().send_packet(packet, 1).expect("Failed to send packet");
              
              self.state = ClientState::Nominal;
            }
            
            Some(enet::Event::Disconnect(ref peer, reason)) => {
              println!("Peer {:?} disconnected: {}", peer, reason);
              self.state = ClientState::Interrupted(InterruptionState::TryLater);
            }
            
            Some(enet::Event::Receive { .. }) => {
              println!("Unexpected receive while waiting for a connection");
            }
          };
        }
        
        InterruptionState::TryLater => {
          //TODO: Schedule reconnection
        }
      };
    }
    
  }
  