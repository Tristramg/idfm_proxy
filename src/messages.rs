use std::sync::Arc;

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<DataUpdate>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct DataUpdate {
    pub pt_data: Arc<crate::PTData>,
}
