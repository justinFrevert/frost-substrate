#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use ice_frost::{self, signature::ThresholdSignature, GroupKey, keygen::{VerifiableSecretSharingCommitment, Coefficients}};

#[runtime_interface]
pub trait IcePrimitive {
    fn verify(&mut self, signature_bytes: [u8; 64], group_key_bytes: [u8; 32], message: &[u8; 64]) -> Result<(), ()> {
		// TODO: remove unwraps
		let signature = ThresholdSignature::from_bytes(&signature_bytes).unwrap();
		let group_key = GroupKey::from_bytes(&group_key_bytes).unwrap();

		signature.verify(&group_key, message).map_err(|_| ())
	}

	fn generate(&mut self) -> Option<[u8; 32]> {
		let commitments = VerifiableSecretSharingCommitment {
			// The first one registered :)
			index: 0,
			points: Vec::new()
		};
		
		let public = commitments.public_key();
		public.map(|o| o.to_owned().compress().to_bytes())
	}
}