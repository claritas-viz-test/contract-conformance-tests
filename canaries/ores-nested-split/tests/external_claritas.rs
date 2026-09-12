use std::fs;
use std::path::PathBuf;

use ores_cli_nested_split_canary::audit::{
    RepositoryAuditOptions, audit_nested_split, audit_nested_split_with_report,
};
use ores_cli_nested_split_canary::model::{CommandReport, Finding};
use tempfile::tempdir;

fn options(path: PathBuf) -> RepositoryAuditOptions {
    RepositoryAuditOptions {
        path,
        profile: "baseline".to_owned(),
        additional_required_paths: Vec::new(),
    }
}

#[test]
fn real_claritas_split_peer_reconciles_only_legacy_false_positives() {
    let repository = PathBuf::from(
        std::env::var_os("ORES_CLI_TEST_NESTED_SPLIT_REPO")
            .expect("ORES_CLI_TEST_NESTED_SPLIT_REPO must point at the exact Claritas checkout"),
    );
    let mut legacy = CommandReport::new("audit repo");
    legacy.push(
        Finding::error(
            "nested-authored-json-schema-missing",
            "legacy co-located scanner false positive",
        )
        .with_target("contracts/peer-authority-canary/typespec"),
    );
    legacy.push(
        Finding::error(
            "nested-typespec-source-missing",
            "legacy co-located scanner false positive",
        )
        .with_target("contracts/peer-authority-canary/json-schema"),
    );

    let report = audit_nested_split_with_report(&options(repository), legacy);
    assert_eq!(report.issue_count(), 0, "{:#?}", report.findings);
    assert_eq!(
        report
            .metadata
            .get("nestedSplitPeerContractCandidateCount")
            .and_then(serde_json::Value::as_u64),
        Some(1),
        "Claritas currently has exactly one nested-split contract home"
    );
    assert_eq!(
        report
            .metadata
            .get("nestedSplitPeerContractValidPairCount")
            .and_then(serde_json::Value::as_u64),
        Some(1),
        "the nested TypeSpec and authored JSON Schema must both be independently valid"
    );
    assert!(!report.findings.iter().any(|finding| {
        matches!(
            finding.code.as_str(),
            "nested-authored-json-schema-missing" | "nested-typespec-source-missing"
        )
    }));
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "nested-split-peer-contracts-inspected")
    );
}

#[test]
fn generated_schema_b_never_satisfies_authored_schema_a() {
    let root = tempdir().expect("temporary repository");
    let contract = root.path().join("contracts/render/typespec");
    fs::create_dir_all(&contract).expect("TypeSpec lane");
    fs::write(contract.join("main.tsp"), "model RenderPacket {}\n").expect("TypeSpec authority");
    let schema = root.path().join("contracts/render/json-schema");
    fs::create_dir_all(&schema).expect("JSON Schema lane");
    fs::write(
        schema.join("typespec.generated.schema.json"),
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema"}"#,
    )
    .expect("generated comparison evidence");

    let report = audit_nested_split(&options(root.path().to_path_buf()));
    assert!(report.findings.iter().any(|finding| {
        finding.code == "nested-split-authored-json-schema-missing"
    }));
    assert_eq!(
        report
            .metadata
            .get("nestedSplitPeerContractValidPairCount")
            .and_then(serde_json::Value::as_u64),
        Some(0)
    );
}

#[test]
fn ambiguity_and_wrong_draft_remain_fail_closed() {
    let ambiguous = tempdir().expect("temporary repository");
    let contract = ambiguous.path().join("contracts/render");
    fs::create_dir_all(contract.join("typespec")).expect("TypeSpec lane");
    fs::create_dir_all(contract.join("json-schema")).expect("JSON Schema lane");
    fs::write(contract.join("typespec/main.tsp"), "model RenderPacket {}\n")
        .expect("TypeSpec authority");
    for name in ["render.schema.json", "alternate.schema.json"] {
        fs::write(
            contract.join("json-schema").join(name),
            r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object"}"#,
        )
        .expect("authored JSON Schema candidate");
    }
    let report = audit_nested_split(&options(ambiguous.path().to_path_buf()));
    assert!(report.findings.iter().any(|finding| {
        finding.code == "nested-split-authored-json-schema-ambiguous"
    }));

    let wrong_draft = tempdir().expect("temporary repository");
    let contract = wrong_draft.path().join("contracts/render");
    fs::create_dir_all(contract.join("typespec")).expect("TypeSpec lane");
    fs::create_dir_all(contract.join("json-schema")).expect("JSON Schema lane");
    fs::write(contract.join("typespec/main.tsp"), "model RenderPacket {}\n")
        .expect("TypeSpec authority");
    fs::write(
        contract.join("json-schema/render.schema.json"),
        r#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object"}"#,
    )
    .expect("wrong-draft schema");
    let mut legacy = CommandReport::new("audit repo");
    legacy.push(
        Finding::error("nested-authored-json-schema-missing", "must remain fail closed")
            .with_target("contracts/render/typespec"),
    );
    let report = audit_nested_split_with_report(&options(wrong_draft.path().to_path_buf()), legacy);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "nested-split-json-schema-draft")
    );
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "nested-authored-json-schema-missing")
    );
}
