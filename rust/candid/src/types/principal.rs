use super::{CandidType, Serializer, Type, TypeId};

pub use ic_types::Principal;

impl CandidType for Principal {
    fn id() -> TypeId {
        TypeId::of::<Principal>()
    }
    fn _ty() -> Type {
        Type::Principal
    }
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_principal(self.as_slice())
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use crate::{CandidType, Encode, Decode, Nat};
    use ic_types::Principal;
    
    #[test]
    fn principal() {
        let principal = Principal::from_text("w7x7r-cok77-xa").unwrap();
        let bytes = Encode!(&principal).unwrap();
        let principal2 = Decode!(&bytes, Principal).unwrap();
        assert_eq!(principal, principal2);
    }

    #[test]
    fn complex_decode_test() {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
        #[candid_path("crate")]
        pub enum CanisterStatusType {
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "stopping")]
            Stopping,
            #[serde(rename = "stopped")]
            Stopped,
        }

        #[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
        #[candid_path("crate")]
        pub struct DefiniteCanisterSettingsArgs {
            controller: Principal,
            controllers: Vec<Principal>,
            compute_allocation: Nat,
            memory_allocation: Nat,
            freezing_threshold: Nat,
        }

        #[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
        #[candid_path("crate")]
        pub struct CanisterStatusResultV2 {
            status: CanisterStatusType,
            module_hash: Option<Vec<u8>>,
            controller: Principal,
            settings: DefiniteCanisterSettingsArgs,
            memory_size: Nat,
            cycles: Nat,
            // this is for compat with Spec 0.12/0.13
            balance: Vec<(Vec<u8>, Nat)>,
            freezing_threshold: Nat,
        }

        let payload = vec![68, 73, 68, 76, 8, 108, 8, 156, 177, 250, 37, 104, 178, 206, 239, 47, 1, 192, 207, 242, 113, 125, 156, 186, 182, 156, 2, 2, 255, 219, 129, 247, 3, 125, 141, 170, 205, 148, 8, 125, 227, 249, 245, 217, 8, 5, 129, 207, 174, 244, 10, 7, 107, 3, 141, 164, 135, 155, 4, 127, 244, 150, 228, 145, 11, 127, 255, 219, 165, 219, 14, 127, 109, 3, 108, 2, 0, 4, 1, 125, 109, 123, 108, 5, 156, 177, 250, 37, 104, 192, 207, 242, 113, 125, 215, 224, 155, 144, 2, 6, 222, 235, 181, 169, 14, 125, 168, 130, 172, 198, 15, 125, 109, 104, 110, 4, 1, 0, 1, 10, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 123, 1, 1, 0, 128, 160, 229, 185, 194, 145, 1, 0, 128, 160, 229, 185, 194, 145, 1, 1, 10, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 123, 1, 1, 10, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0];
        let controller = Principal::from_slice(&{
            let one = 1u64.to_be_bytes();
            let suffix = [1, 1];
            let mut whole: [u8; 10] = [0; 10];
            let (left, right) = whole.split_at_mut(one.len());
            left.copy_from_slice(&one);
            right.copy_from_slice(&suffix);
            whole
        });
        let cycles: Nat = 5_000_000_000_000_u128.into();
        let freezing_threshold: Nat = 123.into();
        assert_eq!(
            Decode!(&payload, CanisterStatusResultV2).unwrap(),
            CanisterStatusResultV2{
                cycles: cycles.clone(),
                freezing_threshold: freezing_threshold.clone(),

                status: CanisterStatusType::Running,
                module_hash: None,
                controller,
                settings: DefiniteCanisterSettingsArgs {
                    controller,
                    controllers: vec![controller],
    
                    compute_allocation: 0.into(),
                    memory_allocation: 0.into(),
                    freezing_threshold,
                },
    
                memory_size: 0.into(),
    
                balance: vec![(vec![0], cycles)],
            }
        );
    }
}