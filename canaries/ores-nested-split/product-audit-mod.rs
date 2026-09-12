mod alignment;
mod cargo_git_pins;
mod cli_secret_flag_hygiene;
mod config;
mod contract;
mod contract_config_peers;
mod contract_consumer;
#[cfg(test)]
mod contract_consumer_tests;
mod contract_evidence;
#[cfg(test)]
mod contract_evidence_tests;
mod contract_generated_evidence;
mod contract_tree;
mod digest;
mod docker_base_pins;
mod environment;
mod family;
mod flags2env_source_pins;
mod flags2env_submodule_source_hygiene;
mod functional_style;
mod infra_gitops;
mod nested_peer_contracts;
mod nested_split_peer_contracts;
mod oauth_provider_contract;
mod org;
mod package;
mod peer_authority_identity;
mod repository;
mod runtime_toml_env_boundary;
mod runtime_toml_generic;
mod runtime_toml_indiebuild;
mod runtime_toml_otel;
mod runtime_toml_rate_limit;
mod runtime_toml_registry;
#[cfg(test)]
mod runtime_toml_registry_tests;
mod runtime_toml_sidecar;
mod source_semantics;
mod split_peer_contracts;
mod tjsv_bidirectional_drift;
mod tjsv_full_check;
mod tjsv_generated_schema_provenance;
mod tjsv_invocation_scope;
mod tjsv_workflow_revision;
mod workflow_action_pins;
mod workflow_permissions;
mod workflow_tjsv_inputs;
mod workflow_tjsv_step_pairing;

use std::path::PathBuf;

pub use config::audit_service_configs;
pub use contract::audit_contract;
pub use contract_tree::audit_contract_tree;
pub use environment::{EnvironmentAuditOptions, audit_environment};
pub use family::{ContractPair, contract_pairs, evidence_report_path};
pub use org::audit_github_org;
pub use package::audit_package;
pub use source_semantics::{AstAuditMode, SourceAuditOptions, audit_source};

use crate::model::CommandReport;

/// Audit one local repository tree, recursively inspect independently authored
/// TypeSpec/JSON Schema peers across direct, nested split, flat split and
/// config-declared split layouts, reconcile legacy co-located findings only
/// after a nested split peer pair is structurally valid, enforce physical
/// independence plus separation between editable authorities and generated TJSV
/// comparison evidence, validate OAuth/OIDC provider source, workflow and
/// `*-test` evidence boundaries, require symmetric fail-closed TJSV drift
/// controls, forbid authored-Schema-A/generated-Schema-B cloning, require a full
/// compiler/emitter/comparison/differential `tjsv check` for every complete peer
/// pair, require one scoped invocation per workflow admission, require explicit
/// current TypeSpec, authored Schema A, report, Contract IR and generated-output
/// bindings plus a fresh same-workflow current-input verifier, require internally
/// consistent immutable TJSV workflow revisions, reject mutable GitHub Actions
/// dependencies, require explicit workflow-token posture and immutable
/// Docker/Cargo Git identities, fail closed on unknown ORES runtime TOML names,
/// enforce credential/public-argv separation plus canonical flags2env provenance,
/// cross-check runtime-owned secret env bindings against public argv exposure,
/// apply bounded peer-authority runtime configuration checks, and harden provider
/// IaC when the infra profile is explicitly selected.
///
/// On top of those lanes this build also applies the generic runtime-config
/// contract checks, the unified Supabase/Neon GitOps policy for `profile=infra`
/// (provider-local state and plaintext dotenv refusal included), and surfaces
/// bounded functional/immutability refactor candidates without blanket-banning
/// resource or hot-path mutation.
///
/// The `source` and `source-available` profiles additionally run the
/// parser-backed source audit ([`audit_source`]) over the baseline structural
/// checks and merge its findings under the `source.*` metadata prefix; they
/// never replace the baseline gates.
#[must_use]
pub fn audit_repository(options: &RepositoryAuditOptions) -> CommandReport {
    let source_mode = source_profile_mode(&options.profile);
    let structural_options;
    let structural = if source_mode.is_some() || options.profile.trim() == "family" {
        structural_options = RepositoryAuditOptions {
            path: options.path.clone(),
            profile: "baseline".to_owned(),
            additional_required_paths: options.additional_required_paths.clone(),
        };
        &structural_options
    } else {
        options
    };

    let report = nested_peer_contracts::augment_nested_peer_contract_audit(
        structural,
        repository::audit_repository(structural),
    );
    let report =
        nested_split_peer_contracts::augment_nested_split_peer_contract_audit(structural, report);
    let report = split_peer_contracts::augment_split_peer_contract_audit(structural, report);
    let report = contract_config_peers::augment_contract_config_peer_audit(structural, report);
    let report = peer_authority_identity::augment_peer_authority_identity_audit(structural, report);
    let report =
        contract_generated_evidence::augment_contract_generated_evidence_audit(structural, report);
    let report = oauth_provider_contract::augment_oauth_provider_contract_audit(structural, report);
    let report =
        tjsv_bidirectional_drift::augment_tjsv_bidirectional_drift_audit(structural, report);
    let report = tjsv_generated_schema_provenance::augment_tjsv_generated_schema_provenance_audit(
        structural, report,
    );
    let report = tjsv_full_check::augment_tjsv_full_check_audit(structural, report);
    let report = tjsv_invocation_scope::augment_tjsv_invocation_scope_audit(structural, report);
    let report = workflow_tjsv_inputs::augment_workflow_tjsv_input_audit(structural, report);
    let report =
        workflow_tjsv_step_pairing::augment_workflow_tjsv_step_pairing_audit(structural, report);
    let report = tjsv_workflow_revision::augment_tjsv_workflow_revision_audit(structural, report);
    let report = infra_gitops::augment_infra_gitops_audit(structural, report);
    let report = workflow_action_pins::augment_workflow_action_pin_audit(structural, report);
    let report = workflow_permissions::augment_workflow_permissions_audit(structural, report);
    let report = docker_base_pins::augment_docker_base_pin_audit(structural, report);
    let report = cargo_git_pins::augment_cargo_git_pin_audit(structural, report);
    let report = flags2env_source_pins::augment_flags2env_source_pin_audit(structural, report);
    let report = family::augment_family_audit(structural, report);
    let report = runtime_toml_registry::augment_runtime_toml_registry_audit(structural, report);
    let report = runtime_toml_generic::augment_generic_runtime_toml_audit(structural, report);
    let report = runtime_toml_rate_limit::augment_rate_limit_runtime_toml_audit(structural, report);
    let report =
        runtime_toml_env_boundary::augment_runtime_toml_env_boundary_audit(structural, report);
    let report = runtime_toml_otel::augment_otel_runtime_toml_audit(structural, report);
    let report = runtime_toml_indiebuild::augment_indiebuild_runtime_toml_audit(structural, report);
    let mut report = runtime_toml_sidecar::augment_sidecar_runtime_toml_audit(structural, report);
    cli_secret_flag_hygiene::audit_cli_secret_flag_hygiene(&structural.path, &mut report);
    flags2env_submodule_source_hygiene::audit_flags2env_submodule_source_hygiene(
        &structural.path,
        &mut report,
    );
    let report = functional_style::augment_functional_style_audit(structural, report.finalize());

    let Some(ast_mode) = source_mode else {
        return report;
    };
    let mut report = report.with_metadata("profile", serde_json::json!(&options.profile));
    let source = audit_source(&SourceAuditOptions {
        path: options.path.clone(),
        ast_mode,
    });
    let has_source_issues = source.issue_count() != 0;
    let source_metadata = source
        .metadata
        .into_iter()
        .map(|(key, value)| (format!("source.{key}"), value));
    report = report
        .with_findings(source.findings)
        .with_metadata_entries(source_metadata);
    if has_source_issues {
        report
            .findings
            .retain(|finding| finding.code != "repository-structure-ready");
    }
    report.finalize()
}

/// Map the public `audit repo --profile` spelling to a parser-evidence policy.
fn source_profile_mode(profile: &str) -> Option<AstAuditMode> {
    match profile.trim() {
        "source" => Some(AstAuditMode::Required),
        "source-available" => Some(AstAuditMode::Available),
        _ => None,
    }
}

/// Options for a GitHub organization audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubOrgAuditOptions {
    /// GitHub account or organization login.
    pub owner: String,
    /// Maximum repositories requested from `gh`.
    pub repo_limit: usize,
    /// Exact additional repository names that must exist.
    pub expected_repositories: Vec<String>,
    /// Optional standard family prefix.
    pub family_prefix: Option<String>,
    /// Standard family suffixes required with the prefix.
    pub family_members: Vec<String>,
    /// Require a `docs` or `*-docs` repository.
    pub require_docs_repository: bool,
    /// Inspect top-level entries for active repositories.
    pub check_layout: bool,
    /// Required top-level entries.
    pub required_root_entries: Vec<String>,
    /// Maximum repositories inspected for layout.
    pub max_layout_repositories: usize,
    /// Whether archived repositories receive layout checks.
    pub include_archived: bool,
}

/// Options for a local repository audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryAuditOptions {
    /// Local repository root.
    pub path: PathBuf,
    /// Structural profile.
    pub profile: String,
    /// Additional required paths.
    pub additional_required_paths: Vec<String>,
}

/// Options for whole-repository TypeSpec/JSON Schema parity validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractTreeAuditOptions {
    /// Repository root containing `contracts/`.
    pub path: PathBuf,
    /// Root directory for generated/parity evidence.
    pub report_root: PathBuf,
    /// Validator executable.
    pub validator: String,
}

/// Options for Cargo/zed-pkg metadata parity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageAuditOptions {
    /// Local package root.
    pub path: PathBuf,
    /// Whether Cargo.lock must exist.
    pub require_cargo_lock: bool,
}

/// Options for TypeSpec/JSON Schema parity validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractAuditOptions {
    /// TypeSpec source entry.
    pub typespec: PathBuf,
    /// Independently authored JSON Schema source.
    pub schema: PathBuf,
    /// Machine-readable validator report path.
    pub report: PathBuf,
    /// Validator executable.
    pub validator: String,
}
