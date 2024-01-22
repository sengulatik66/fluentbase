use crate::{LowLevelAPI, LowLevelSDK};
use alloc::{vec, vec::Vec};
pub use alloy_primitives::{Address, Bytes, B256, U256};
use fluentbase_codec::{define_codec_struct, BufferDecoder, Encoder};

define_codec_struct! {
    pub struct ContractInput {
        // env info
        env_chain_id: u64,
        // contract info
        contract_address: Address,
        contract_caller: Address,
        contract_bytecode: Bytes,
        contract_code_size: u32,
        contract_code_hash: B256,
        contract_input: Bytes,
        contract_value: U256,
        // block info
        block_hash: B256,
        block_coinbase: Address,
        block_timestamp: u64,
        block_number: u64,
        block_difficulty: u64,
        block_gas_limit: u64,
        block_base_fee: U256,
        // tx info
        tx_gas_price: U256,
        tx_gas_priority_fee: Option<U256>,
        tx_caller: Address,
        // tx_blob_hashes: Vec<B256>,
        // tx_blob_gas_price: u64,
    }
}
// define_codec_struct! {
//     pub struct ContractLog {
//         address: Address,
//         topics: Vec<B256>,
//         data: Bytes,
//     }
// }
define_codec_struct! {
    pub struct ContractOutput {
        return_data: Bytes,
        // logs: Vec<ContractLog>,
    }
}

macro_rules! impl_reader_helper {
    ($input_type:ty, $return_typ:ty) => {{
        let mut buffer: [u8; <$input_type>::FIELD_SIZE] = [0; <$input_type>::FIELD_SIZE];
        LowLevelSDK::sys_read(&mut buffer, <$input_type>::FIELD_OFFSET as u32);
        let mut result: $return_typ = Default::default();
        let (offset, length) = <$input_type>::decode_field_header_at(&buffer, 0, &mut result);
        if length > 0 {
            let mut buffer2 = vec![0; offset + length];
            buffer2[0..<$input_type>::FIELD_SIZE].copy_from_slice(&buffer);
            LowLevelSDK::sys_read(
                &mut buffer2.as_mut_slice()[offset..(offset + length)],
                offset as u32,
            );
            <$input_type>::decode_field_body_at(&buffer2, 0, &mut result);
        }
        result
    }};
}
macro_rules! impl_reader_func {
    (fn $fn_name:ident() -> $return_typ:ty, $input_type:ty) => {
        paste::paste! {
            pub fn $fn_name() -> $return_typ {
                impl_reader_helper!($input_type, $return_typ)
            }
            // pub fn [<get_ $fn_name>](&mut self) -> &$return_typ {
                // if self.cached_state.$fn_name.is_none() {
                //     self.cached_state.$fn_name = Some(impl_reader_helper!($input_type, $return_typ));
                // }
                // self.cached_state.$fn_name.as_ref().unwrap()
            // }
        }
    };
}

// #[derive(Default)]
// struct CachedState {
//     env_chain_id: Option<u64>,
//     contract_address: Option<Address>,
//     contract_caller: Option<Address>,
//     contract_bytecode: Option<Bytes>,
//     contract_code_size: Option<u32>,
//     contract_code_hash: Option<B256>,
//     contract_input: Option<Bytes>,
//     contract_value: Option<U256>,
//     block_hash: Option<B256>,
//     block_coinbase: Option<Address>,
//     block_timestamp: Option<u64>,
//     block_number: Option<u64>,
//     block_difficulty: Option<u64>,
//     block_gas_limit: Option<u64>,
//     block_base_fee: Option<U256>,
//     tx_gas_price: Option<U256>,
//     tx_gas_priority_fee: Option<Option<U256>>,
//     tx_caller: Option<Address>,
//     tx_blob_hashes: Option<Vec<B256>>,
//     tx_blob_gas_price: Option<u64>,
// }

#[derive(Default)]
pub struct ExecutionContext {
    output: Option<ContractOutput>,
    // cached_state: CachedState,
}

macro_rules! output_mut_or_default {
    ($self:ident) => {{
        if $self.output.is_none() {
            $self.output = Some(Default::default());
        }
        $self.output.as_mut().unwrap()
    }};
}

impl ExecutionContext {
    // env info
    impl_reader_func!(fn env_chain_id() -> u64, ContractInput::EnvChainId);
    // contract info
    impl_reader_func!(fn contract_address() -> Address, ContractInput::ContractAddress);
    impl_reader_func!(fn contract_caller() -> Address, ContractInput::ContractCaller);
    impl_reader_func!(fn contract_bytecode() -> Bytes, ContractInput::ContractBytecode);
    impl_reader_func!(fn contract_code_size() -> u32, ContractInput::ContractCodeSize);
    impl_reader_func!(fn contract_code_hash() -> B256, ContractInput::ContractCodeHash);
    impl_reader_func!(fn contract_input() -> Bytes, ContractInput::ContractInput);
    impl_reader_func!(fn contract_value() -> U256, ContractInput::ContractValue);
    // block info
    impl_reader_func!(fn block_hash() -> B256, ContractInput::BlockHash);
    impl_reader_func!(fn block_coinbase() -> Address, ContractInput::BlockCoinbase);
    impl_reader_func!(fn block_timestamp() -> u64, ContractInput::BlockTimestamp);
    impl_reader_func!(fn block_number() -> u64, ContractInput::BlockNumber);
    impl_reader_func!(fn block_difficulty() -> u64, ContractInput::BlockDifficulty);
    impl_reader_func!(fn block_gas_limit() -> u64, ContractInput::BlockGasLimit);
    impl_reader_func!(fn block_base_fee() -> U256, ContractInput::BlockBaseFee);
    // tx info
    impl_reader_func!(fn tx_gas_price() -> U256, ContractInput::TxGasPrice);
    impl_reader_func!(fn tx_gas_priority_fee() -> Option<U256>, ContractInput::TxGasPriorityFee);
    impl_reader_func!(fn tx_caller() -> Address, ContractInput::TxCaller);
    // impl_reader_func!(fn tx_blob_hashes() -> Vec<B256>, ContractInput::TxBlobHashes);
    // impl_reader_func!(fn tx_blob_gas_price() -> u64, ContractInput::TxBlobGasPrice);

    // pub fn emit_log(&mut self, topics: Vec<B256>, data: Bytes) {
    //     let address = Self::contract_address();
    //     let output = output_mut_or_default!(self);
    // output.logs.push(ContractLog {
    //     address,
    //     topics,
    //     data,
    // })
    // }

    pub fn emit_return(&mut self, return_data: &[u8]) {
        let output = output_mut_or_default!(self);
        output.return_data = Bytes::copy_from_slice(return_data);
    }

    pub fn static_return_and_exit<const N: usize>(
        &self,
        return_data: &'static [u8; N],
        exit_code: i32,
    ) where
        [u8; N + ContractOutput::HEADER_SIZE]:,
    {
        let contract_output = ContractOutput {
            return_data: Bytes::from_static(return_data),
            // logs: vec![],
        };
        let (buffer, length) =
            contract_output.encode_to_fixed::<{ N + ContractOutput::HEADER_SIZE }>(0);
        LowLevelSDK::sys_write(&buffer[..length]);
        LowLevelSDK::sys_halt(exit_code);
    }

    pub fn fast_return_and_exit<R: Into<Bytes>>(&self, return_data: R, exit_code: i32) {
        let contract_output = ContractOutput {
            return_data: return_data.into(),
            // logs: vec![],
        };
        LowLevelSDK::sys_write(contract_output.encode_to_vec(0).as_slice());
        // LowLevelSDK::sys_write(return_data);
        LowLevelSDK::sys_halt(exit_code);
    }

    pub fn exit(&self, exit_code: i32) {
        if let Some(output) = self.output.as_ref() {
            LowLevelSDK::sys_write(output.encode_to_vec(0).as_slice());
        }
        LowLevelSDK::sys_halt(exit_code);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        evm::{ContractInput, ExecutionContext, U256},
        LowLevelSDK,
    };
    use alloy_primitives::{Bytes, B256};
    use fluentbase_codec::Encoder;

    #[test]
    fn test_encode_decode() {
        // encode input and put into global var
        let contract_input = ContractInput {
            contract_input: Bytes::from_static(&[0, 1, 2, 3]),
            contract_bytecode: Bytes::from_static(&[4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
            block_hash: B256::from(U256::from(7)),
            ..Default::default()
        };
        let encoded_input = contract_input.encode_to_vec(0);
        LowLevelSDK::with_test_input(encoded_input);
        // read input fields
        let input = ExecutionContext::contract_input();
        assert_eq!(input, contract_input.contract_input);
        let bytecode = ExecutionContext::contract_bytecode();
        assert_eq!(bytecode, contract_input.contract_bytecode);
        let block_hash = ExecutionContext::block_hash();
        assert_eq!(block_hash, contract_input.block_hash);
    }
}
