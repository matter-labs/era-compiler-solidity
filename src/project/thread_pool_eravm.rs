//!
//! The EraVM project thread pool.
//!

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use era_compiler_llvm_context::Dependency;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::process::input_eravm::Input as EraVMInput;
use crate::process::output_eravm::Output as EraVMOutput;
use crate::project::contract::factory_dependency::FactoryDependency;
use crate::project::contract::Contract;

///
/// The project thread pool.
///
/// Performs the project compilation in parallel, executing the tasks in the thread pool.
/// Each contract is compiled in a separate `zksolc` process, when all its dependencies
/// have been compiled beforehand.
///
#[derive(Clone)]
pub struct ThreadPool {
    /// The inner thread pool.
    pub inner: rusty_pool::ThreadPool,
    /// The thread-safe storage of input contracts.
    pub contracts: Arc<RwLock<BTreeMap<String, Contract>>>,
    /// The child process input template.
    pub input_template: EraVMInput,
    /// The thread-safe storage of evaluation results.
    pub results: Arc<RwLock<BTreeMap<String, anyhow::Result<EraVMContractBuild>>>>,
}

impl ThreadPool {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        threads: Option<usize>,
        contracts: BTreeMap<String, Contract>,
        input_template: EraVMInput,
    ) -> Self {
        let threads = threads.unwrap_or_else(num_cpus::get);
        let inner = rusty_pool::ThreadPool::new(threads, threads, Duration::from_secs(1));

        Self {
            inner,
            contracts: Arc::new(RwLock::new(contracts)),
            input_template,
            results: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    ///
    /// Starts the evaluation.
    ///
    /// Checks which contracts have satisfied dependencies and are ready to be compiled.
    ///
    pub fn start(&self) {
        let contracts_satisfied: Vec<String> = self
            .contracts
            .read()
            .expect("Sync")
            .iter()
            .filter_map(|(path, contract)| {
                if contract.are_factory_dependencies_satisfied(
                    self.results
                        .read()
                        .expect("Sync")
                        .keys()
                        .collect::<Vec<&String>>(),
                    &self.input_template.dependency_data,
                ) {
                    Some(path.to_owned())
                } else {
                    None
                }
            })
            .collect();

        'outer: for path in contracts_satisfied.into_iter() {
            let contract = match self.contracts.write().expect("Sync").remove(path.as_str()) {
                Some(contract) => contract,
                None => continue,
            };

            let mut dependencies = BTreeMap::new();
            for dependency in contract.get_factory_dependencies().into_iter() {
                let resolved_path = self
                    .input_template
                    .dependency_data
                    .resolve_path(dependency)
                    .expect("Always valid");
                let output = match self
                    .results
                    .read()
                    .expect("Sync")
                    .get(resolved_path.as_str())
                    .expect("Always exists")
                {
                    Ok(contract) => contract.to_owned(),
                    Err(_error) => continue 'outer,
                };
                dependencies.insert(resolved_path, output);
            }

            self.evaluate(path, contract, dependencies);
        }
    }

    ///
    /// Joins the thread pool and returns the extracted collection of evaluated results.
    ///
    pub fn finish(self) -> BTreeMap<String, anyhow::Result<EraVMContractBuild>> {
        self.inner.join();
        Arc::try_unwrap(self.results)
            .expect("Sync")
            .into_inner()
            .expect("Sync")
    }

    ///
    /// Compiles the contract and stores the evaluated result.
    ///
    /// Afterwards, the evaluation loop is restarted to check if any other contracts' dependencies are now satisfied.
    ///
    fn evaluate(
        &self,
        path: String,
        contract: Contract,
        dependencies: BTreeMap<String, EraVMContractBuild>,
    ) {
        let mut input = self.input_template.to_owned();
        input.contract = Some(contract);
        input.dependency_data.dependencies.extend(dependencies);

        let results = self.results.clone();
        let pool = self.to_owned();
        self.inner.evaluate(move || {
            let result: anyhow::Result<EraVMOutput> =
                crate::process::call(input, era_compiler_llvm_context::Target::EraVM);
            results
                .write()
                .expect("Sync")
                .insert(path, result.map(|output| output.build));
            pool.start();
        });
    }
}
