use anchor_basic_amm::entry as entry_anchor_basic_amm;
use anchor_basic_amm::ID as PROGRAM_ID_ANCHOR_BASIC_AMM;
const PROGRAM_NAME_ANCHOR_BASIC_AMM: &str = "anchor_basic_amm";
use fuzz_instructions::anchor_basic_amm_fuzz_instructions::FuzzInstruction as FuzzInstruction_anchor_basic_amm;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_anchor_basic_amm;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_ANCHOR_BASIC_AMM,
                &PROGRAM_ID_ANCHOR_BASIC_AMM,
                processor!(convert_entry!(entry_anchor_basic_amm))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_ANCHOR_BASIC_AMM, &mut client);
        });
    }
}
