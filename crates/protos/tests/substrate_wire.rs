use prost::Message;
use protos::ateapi::{ActorState, MintActorJwtRequest, ObjectRef, ResumeActorResponse};

// Service names from Substrate's public v1 APIs at c0451cea0.
#[test]
fn control_service_name_matches_server() {
	assert_eq!(
		protos::ateapi::control_server::SERVICE_NAME,
		"substrate.ateapi.v1.Control"
	);
}

#[test]
fn credential_provider_service_name_matches_server() {
	assert_eq!(
		protos::credprovider::credential_provider_server::SERVICE_NAME,
		"substrate.credprovider.v1.CredentialProvider"
	);
}

// Encoded with Substrate's Go bindings at 34bde5827. Keep these independent of
// the gateway bindings so a shared mistake in the client and mock cannot pass.
#[test]
fn resume_response_decodes_worker_assignment() {
	let wire = [
		10, 25, 10, 7, 26, 3, 117, 105, 100, 32, 7, 42, 14, 8, 2, 18, 10, 42, 8, 49, 48, 46, 48, 46,
		48, 46, 49, 16, 1,
	];
	let response = ResumeActorResponse::decode(wire.as_slice()).unwrap();
	assert!(response.resumed);
	let actor = response.actor.unwrap();
	let metadata = actor.metadata.unwrap();
	assert_eq!(metadata.uid, "uid");
	assert_eq!(metadata.version, 7);
	let status = actor.status.expect("running actor status");
	assert_eq!(status.state(), ActorState::Running);
	assert_eq!(
		status.worker_assignment.unwrap().worker_pod_ips,
		["10.0.0.1"]
	);
}

#[test]
fn mint_actor_jwt_request_matches_server_wire_format() {
	let wire = [
		10, 13, 10, 4, 100, 101, 109, 111, 18, 5, 97, 99, 116, 111, 114, 18, 19, 104, 116, 116, 112,
		115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 24, 172, 2,
	];
	let request = MintActorJwtRequest {
		actor: Some(ObjectRef {
			atespace: "demo".into(),
			name: "actor".into(),
		}),
		audiences: vec!["https://example.com".into()],
		expiration_seconds: 300,
	};
	assert_eq!(request.encode_to_vec(), wire);
}

#[test]
fn terminal_actor_states_match_server_values() {
	assert_eq!(ActorState::try_from(5).unwrap(), ActorState::Crashed);
	assert_eq!(ActorState::try_from(6).unwrap(), ActorState::Deleting);
	assert_eq!(ActorState::try_from(7).unwrap(), ActorState::Reverting);
}
