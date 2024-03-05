use serde::Serialize;

#[derive(Serialize)]
pub struct HeartbeatResponse {
    op: i8,
    d: i32,
}


pub fn create_heartbeat_response(sequence: i32) -> HeartbeatResponse{
    HeartbeatResponse {
        op: 1,
        d: sequence,
    }
}