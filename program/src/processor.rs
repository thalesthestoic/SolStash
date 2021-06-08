se solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::PrintProgramError,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::{clock::Clock, Sysvar},
};

use num_traits::FromPrimitive;
use spl_token::{instruction::transfer, state::Account};

use crate::{
    error::VestingError,
    instruction::{Schedule, VestingInstruction, SCHEDULE_SIZE},
    state::{pack_schedules_into_slice, unpack_schedules, VestingSchedule, VestingScheduleHeader},
};

pub struct Processor {}

impl Processor {
    pub fn process_init(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        seeds: [u8; 32],
        schedules: u32
    ) -> ProgramResult {
        
        Ok(())
    }
