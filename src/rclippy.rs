//! Clippy wrapper with very pedantic configuration

/// Flavor of clippy harshness
mod clippy_flavor;

use std::{
    io,
    process::{Command, Stdio},
};

use clap::Parser;

use clippy_flavor::ClippyFlavor;

/// List of all lints for the default development flavor
const DEVELOPMENT_LINTS: &[&str] = &[
    "missing_docs",
    // Cargo lints
    "clippy::wildcard_dependencies",
    // Pedantic lints
    "clippy::borrow_as_ptr",
    "clippy::case_sensitive_file_extension_comparisons",
    "clippy::cast_lossless",
    "clippy::cast_possible_truncation",
    "clippy::cast_possible_wrap",
    "clippy::cast_precision_loss",
    "clippy::cast_ptr_alignment",
    "clippy::cast_sign_loss",
    "clippy::checked_conversions",
    "clippy::cloned_instead_of_copied",
    "clippy::copy_iterator",
    "clippy::default_trait_access",
    "clippy::doc_link_with_quotes",
    "clippy::doc_markdown",
    "clippy::empty_enum",
    "clippy::enum_glob_use",
    "clippy::expl_impl_clone_on_copy",
    "clippy::explicit_deref_methods",
    "clippy::filter_map_next",
    "clippy::flat_map_option",
    "clippy::float_cmp",
    "clippy::fn_params_excessive_bools",
    "clippy::from_iter_instead_of_collect",
    "clippy::if_not_else",
    "clippy::implicit_clone",
    "clippy::implicit_hasher",
    "clippy::inconsistent_struct_constructor",
    "clippy::index_refutable_slice",
    "clippy::inefficient_to_string",
    "clippy::inline_always",
    "clippy::invalid_upcast_comparisons",
    "clippy::items_after_statements",
    "clippy::iter_not_returning_iterator",
    "clippy::large_digit_groups",
    "clippy::large_stack_arrays",
    "clippy::large_types_passed_by_value",
    "clippy::linkedlist",
    "clippy::macro_use_imports",
    "clippy::manual_assert",
    "clippy::manual_instant_elapsed",
    "clippy::manual_let_else",
    "clippy::manual_ok_or",
    "clippy::manual_string_new",
    "clippy::many_single_char_names",
    "clippy::map_unwrap_or",
    "clippy::match_bool",
    "clippy::match_on_vec_items",
    "clippy::match_same_arms",
    "clippy::match_wild_err_arm",
    "clippy::match_wildcard_for_single_variants",
    "clippy::maybe_infinite_iter",
    "clippy::mismatching_type_param_order",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
    "clippy::module_name_repetitions",
    "clippy::must_use_candidate",
    "clippy::mut_mut",
    "clippy::naive_bytecount",
    "clippy::needless_bitwise_bool",
    "clippy::needless_continue",
    "clippy::needless_for_each",
    "clippy::needless_pass_by_value",
    "clippy::no_effect_underscore_binding",
    "clippy::option_option",
    "clippy::ptr_as_ptr",
    "clippy::range_minus_one",
    "clippy::range_plus_one",
    "clippy::redundant_closure_for_method_calls",
    "clippy::redundant_else",
    "clippy::ref_binding_to_reference",
    "clippy::ref_option_ref",
    "clippy::return_self_not_must_use",
    "clippy::same_functions_in_if_condition",
    "clippy::semicolon_if_nothing_returned",
    "clippy::similar_names",
    "clippy::single_match_else",
    "clippy::stable_sort_primitive",
    "clippy::string_add_assign",
    "clippy::struct_excessive_bools",
    "clippy::too_many_lines",
    "clippy::transmute_ptr_to_ptr",
    "clippy::trivially_copy_pass_by_ref",
    "clippy::unchecked_duration_subtraction",
    "clippy::unicode_not_nfc",
    "clippy::unnecessary_join",
    "clippy::unnecessary_wraps",
    "clippy::unnested_or_patterns",
    "clippy::unreadable_literal",
    "clippy::unsafe_derive_deserialize",
    "clippy::unused_async",
    "clippy::unused_self",
    "clippy::used_underscore_binding",
    "clippy::verbose_bit_mask",
    "clippy::wildcard_imports",
    "clippy::zero_sized_map_values",
    // Nursery lints
    "clippy::as_ptr_cast_mut",
    "clippy::branches_sharing_code",
    "clippy::debug_assert_with_mut_call",
    "clippy::derive_partial_eq_without_eq",
    "clippy::empty_line_after_outer_attr",
    "clippy::fallible_impl_from",
    "clippy::iter_on_empty_collections",
    "clippy::iter_on_single_items",
    "clippy::manual_clamp",
    "clippy::mutex_integer",
    "clippy::needless_collect",
    "clippy::nonstandard_macro_braces",
    "clippy::or_fun_call",
    "clippy::path_buf_push_overwrite",
    "clippy::redundant_pub_crate",
    "clippy::significant_drop_in_scrutinee",
    // "clippy::significant_drop_tightening",
    "clippy::string_lit_as_bytes",
    "clippy::suboptimal_flops",
    "clippy::suspicious_operation_groupings",
    "clippy::trailing_empty_array",
    "clippy::trait_duplication_in_bounds",
    "clippy::transmute_undefined_repr",
    "clippy::trivial_regex",
    "clippy::type_repetition_in_bounds",
    "clippy::unused_peekable",
    "clippy::unused_rounding",
    "clippy::use_self",
    "clippy::useless_let_if_seq",
    // Restriction lints
    "clippy::allow_attributes_without_reason",
    "clippy::arithmetic_side_effects",
    "clippy::as_conversions",
    "clippy::as_underscore",
    "clippy::assertions_on_result_states",
    "clippy::clone_on_ref_ptr",
    "clippy::create_dir",
    "clippy::decimal_literal_representation",
    "clippy::default_numeric_fallback",
    "clippy::default_union_representation",
    "clippy::disallowed_script_idents",
    "clippy::empty_drop",
    "clippy::empty_structs_with_brackets",
    "clippy::exit", // This hinders error reporting and might generally be counterproductive
    "clippy::expect_used",
    "clippy::filetype_is_file",
    "clippy::float_cmp_const",
    "clippy::fn_to_numeric_cast_any",
    "clippy::format_push_string",
    "clippy::indexing_slicing",
    "clippy::lossy_float_literal",
    "clippy::mem_forget",
    "clippy::missing_docs_in_private_items",
    "clippy::mixed_read_write_in_expression",
    "clippy::mod_module_files",
    "clippy::mutex_atomic",
    "clippy::pattern_type_mismatch",
    "clippy::rc_buffer",
    "clippy::rc_mutex",
    "clippy::same_name_method",
    // "clippy::semicolon_inside_block",
    "clippy::separated_literal_suffix",
    "clippy::str_to_string",
    "clippy::string_add",
    "clippy::string_slice",
    "clippy::string_to_string",
    "clippy::suspicious_xor_used_as_pow",
    "clippy::try_err",
    "clippy::undocumented_unsafe_blocks",
    "clippy::unnecessary_safety_comment",
    "clippy::unnecessary_safety_doc",
    "clippy::unnecessary_self_imports",
    "clippy::unwrap_used",
];

/// Extra pedantic lints which are ok by themselves but should NEVER be on a main branch
const PEDANTIC_LINTS: [&str; 2] = ["clippy::todo", "clippy::dbg_macro"];

/// Lints to always reject
const DENY_ALWAYS: [&str; 1] = ["non_ascii_idents"];

/// Run a standard set of cargo watch commands and use one of multiple standardised clippy commands
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct CliArgs {
    /// Which clippy flavor to use
    #[clap(value_parser, default_value_t = ClippyFlavor::default())]
    clippy_flavor: ClippyFlavor,

    /// Wether to only warn for lints
    #[clap(short, long, value_parser, default_value_t = false)]
    warn: bool,

    /// Wether to run with release
    #[clap(short = 'O', long, value_parser, default_value_t = false)]
    optimize: bool,
}

fn main() -> io::Result<()> {
    let cli_args = CliArgs::parse();

    let mut cmd = Command::new("cargo");
    // cmd.args(&["+nightly", "clippy"]);
    cmd.arg("clippy");

    if cli_args.optimize {
        cmd.arg("--release");
    }

    cmd.arg("--");

    let action_flag = if cli_args.warn { "-W" } else { "-D" };

    let lints: &[&[&str]] = match cli_args.clippy_flavor {
        ClippyFlavor::Pedantic => &[DEVELOPMENT_LINTS, &PEDANTIC_LINTS],
        ClippyFlavor::Development => &[DEVELOPMENT_LINTS],
        ClippyFlavor::Prototype => &[],
    };

    for lint in lints.iter().flat_map(|li| li.iter()) {
        cmd.args([action_flag, lint]);
    }

    cmd.args([
        "-A",
        "clippy::must_use_candidate",
        "-A",
        "clippy::needless_pass_by_value",
        "-A",
        "clippy::module_name_repetitions",
        "-A",
        "clippy::enum_variant_names",
        action_flag,
        "warnings",
    ]);

    for lint in DENY_ALWAYS {
        cmd.args(["-D", lint]);
    }

    let output = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    } else {
        Ok(())
    }
}
