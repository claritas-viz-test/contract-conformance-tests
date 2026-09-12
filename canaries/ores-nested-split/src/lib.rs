pub mod model;

pub mod audit {
    use std::path::PathBuf;

    use crate::model::CommandReport;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RepositoryAuditOptions {
        pub path: PathBuf,
        pub profile: String,
        pub additional_required_paths: Vec<String>,
    }

    mod nested_split_peer_contracts;

    #[must_use]
    pub fn audit_nested_split_with_report(
        options: &RepositoryAuditOptions,
        report: CommandReport,
    ) -> CommandReport {
        nested_split_peer_contracts::augment_nested_split_peer_contract_audit(options, report)
    }

    #[must_use]
    pub fn audit_nested_split(options: &RepositoryAuditOptions) -> CommandReport {
        audit_nested_split_with_report(options, CommandReport::new("audit repo"))
    }
}
