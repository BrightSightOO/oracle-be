use crate::accounts::{
    AssertionV1, ConfigV1, CurrencyV1, OracleV1, RequestV1, StakeV1, VoteV1, VotingV1,
};
use crate::types::{AccountType, Bounds, RequestKind, RequestState};

macro_rules! borsh1_delegate_impl {
    ($($account:ty)*) => {
        $(
            impl ::borsh1::BorshDeserialize for $account {
                fn deserialize(buf: &mut &[u8]) -> ::std::io::Result<Self> {
                    ::borsh::BorshDeserialize::deserialize(buf)
                }

                fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
                    ::borsh::BorshDeserialize::deserialize_reader(reader)
                }

                fn try_from_slice(v: &[u8]) -> ::std::io::Result<Self> {
                    ::borsh::BorshDeserialize::try_from_slice(v)
                }

                fn try_from_reader<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
                    ::borsh::BorshDeserialize::try_from_reader(reader)
                }
            }

            impl ::borsh1::BorshSerialize for $account {
                fn serialize<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                    ::borsh::BorshSerialize::serialize(&self, writer)
                }
            }
        )*
    };
}

borsh1_delegate_impl! { OracleV1 ConfigV1 StakeV1 RequestV1 AssertionV1 CurrencyV1 VotingV1 VoteV1 }
borsh1_delegate_impl! { AccountType Bounds RequestKind RequestState }
