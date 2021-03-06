use crate::aries::messages::proof_presentation::presentation::Presentation;
use crate::aries::messages::proof_presentation::presentation_request::PresentationRequest;
use crate::aries::messages::status::Status;
use crate::aries::handlers::proof_presentation::prover::states::initial::InitialState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    pub connection_handle: u32,
    pub presentation_request: PresentationRequest,
    pub presentation: Presentation,
    pub status: Status,
}


impl From<InitialState> for FinishedState {
    fn from(state: InitialState) -> Self {
        trace!("transit state from InitialState to FinishedState");
        FinishedState {
            connection_handle: 0,
            presentation_request: state.presentation_request,
            presentation: Default::default(),
            status: Status::Declined,
        }
    }
}


