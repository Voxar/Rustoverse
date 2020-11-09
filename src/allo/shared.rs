use super::wire::WireObject;

pub struct Identity {

}

pub struct Avatar {

}

pub struct Interaction<Body: WireObject> {
    pub kind: String,
    pub sender_entity_id: String,
    pub receiver_entity_id: String,
    pub request_id: String,
    pub body: Body,
}

pub struct Intent {
    
}

pub struct Announce {
    identity: Identity,
    avatar: Avatar
}
