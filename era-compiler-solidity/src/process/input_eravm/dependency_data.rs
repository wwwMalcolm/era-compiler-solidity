//!
//! The EraVM dependency data.
//!

use std::collections::BTreeMap;

use crate::build_eravm::contract::Contract as EraVMContractBuild;

///
/// The EraVM dependency data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyData {
    /// The `solc` compiler version.
    pub solc_version: Option<era_solc::Version>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// The dependencies required by specific contract.
    pub dependencies: BTreeMap<String, EraVMContractBuild>,
}

impl DependencyData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_version: Option<era_solc::Version>,
        identifier_paths: BTreeMap<String, String>,
    ) -> Self {
        Self {
            solc_version,
            identifier_paths,
            dependencies: BTreeMap::new(),
        }
    }
}

impl era_compiler_llvm_context::Dependency for DependencyData {
    fn get(&self, identifier: &str) -> anyhow::Result<String> {
        let path = self.resolve_path(identifier)?;
        let contract = self
            .dependencies
            .get(path.as_str())
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("dependency `{path}` not found in the project"))?;
        match contract.build.bytecode_hash {
            Some(bytecode_hash) => Ok(hex::encode(bytecode_hash)),
            None => anyhow::bail!(
                "dependency `{path}` has no bytecode hash, as it may require library linkage"
            ),
        }
    }

    fn resolve_path(&self, identifier: &str) -> anyhow::Result<String> {
        self.identifier_paths
            .get(identifier.strip_suffix("_deployed").unwrap_or(identifier))
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "dependency with identifier `{}` not found in the project",
                    identifier
                )
            })
    }
}
