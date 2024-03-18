use std::env::{self, VarError};
use std::sync::{Once, OnceLock};

use color_eyre::config::{Frame, HookBuilder, Theme};
use color_eyre::owo_colors::style;

use crate::prelude::*;

fn setup_color_eyre() {
    let theme = Theme::dark()
        .crate_code(style().red())
        .dependency_code(style().white().dimmed())
        .code_hash(style().black().dimmed())
        .error(style().red())
        .help_info_note(style().bright_blue())
        .help_info_warning(style().yellow())
        .help_info_suggestion(style().bright_cyan())
        .help_info_error(style().red());

    fn remove_test_setup(frames: &mut Vec<&Frame>) {
        let tail = {
            let mut iter = frames.iter().enumerate();

            loop {
                let (i, frame) = match iter.next() {
                    Some(next) => next,
                    None => return,
                };

                if let Some(name) = frame.name.as_deref() {
                    if name.starts_with("test::__rust_begin_short_backtrace") {
                        break i;
                    }
                }
            }
        };

        frames.truncate(tail);
    }

    fn remove_filtered(frames: &mut Vec<&Frame>) {
        const FILTER: &[&str] = &[];

        frames.retain(|frame| match frame.name.as_deref() {
            Some(name) => !FILTER.iter().any(|f| name.starts_with(f)),
            None => true,
        });
    }

    HookBuilder::default()
        .theme(theme)
        .add_frame_filter(Box::new(remove_test_setup))
        .add_frame_filter(Box::new(remove_filtered))
        .install()
        .unwrap()
}

fn get_inactive_features() -> &'static [Pubkey] {
    static INACTIVE_FEATURES: OnceLock<Box<[Pubkey]>> = OnceLock::new();

    INACTIVE_FEATURES.get_or_init(|| match env::var("MAINNET_INACTIVE_FEATURES") {
        Ok(var) => var
            .split('\n')
            .map(|f| f.parse::<Pubkey>().unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        Err(VarError::NotPresent) => Box::new([]),
        Err(err) => panic!("failed to get MAINNET_INACTIVE_FEATURES: {err:?}"),
    })
}

pub fn program_test(program_name: &str, program_id: Pubkey) -> ProgramTest {
    static ONCE: Once = Once::new();

    ONCE.call_once(setup_color_eyre);

    let mut program_test = ProgramTest::new(program_name, program_id, None);

    let common_test = env!("CARGO_PKG_NAME");

    solana_logger::setup_with(&format!(
        "solana_rbpf::vm=debug,\
        solana_runtime::message_processor=debug,\
        solana_runtime::system_instruction_processor=trace,\
        solana_program_test=info,\
        tarpc=error,\
        {common_test}=trace,\
        {program_name}=trace,\
        test=trace,\
        warn",
    ));

    for feature in get_inactive_features() {
        program_test.deactivate_feature(*feature);
    }

    program_test
}
