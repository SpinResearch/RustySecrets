use protobuf;
use std::num;

error_chain!{
    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Protobuf(protobuf::ProtobufError);
        ParseInt(num::ParseIntError);
    }

    errors {
        EmptyShares {
        	description("No shares were provided.")
        } 
        IncompatibleSets(groups: Vec<Vec<u8>>) {
        	description("The shares are incompatible with each other.")
        }
        InvalidSignature(share_num: u8, desc: String) {
        	description("The signature of this share is not valid.")
        	// display("The signature of this share is not valid. ({})", desc)
        }
        MissingShares(required: u8, found: usize) {
 			description("The number of shares provided is insufficient to recover the secret.")
 			display("The number of shares provided is insufficient to recover the secret. ({} shares are required to recover the secret, found only {}.)", required, found)
        }
        MissingSignature(share_num: u8) {
        	description("Signature is missing while shares are required to be signed.")
        }
        SecretDeserializationIssue {
        	description("An issue was encountered deserializing the secret.
        		Updating to the latest version of RustySecrets might help fix this.")
        }
        ShareParsingError(share_num: u8, desc: String) {
        	description("This share is incorrectly formatted.")
        	display("This share is incorrectly formatted. ({})", desc)
        }
        DuplicateShareNum(share_num: u8) {
        	description("This share number has already been used by a previous share.")
        } 
        DuplicateShareData(share_num: u8) {
        	description("The data encoded in this share is the same as the one found in a previous share.")
        }
    }
}
