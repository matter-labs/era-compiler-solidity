import * as path from 'path';

const outputDir = 'artifacts';
const binOutputFile = 'C.zbin';
const eraVMAssemblyOutputFile = 'C.zasm';
const libraryDefault = 'MiniMath.sol:MiniMath=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC';
const contractSolFilename = 'contract.sol';
const contractYulFilename = 'contract.yul';
const contractEraVMAssemblyFilename = 'contract.zasm';
const contractLlvmFilename = 'contract.ll';
const contractJSONFilename = 'contract.json';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicYulContract = path.join(pathToContracts, 'yul', contractYulFilename);
const pathToBasicEraVMAssemblyContract = path.join(pathToContracts, 'eravm', contractEraVMAssemblyFilename);
const pathToBasicSolContract = path.join(pathToContracts, 'solidity', contractSolFilename);
const pathToBasicLlvmContract = path.join(pathToContracts, "llvm", contractLlvmFilename);
const pathToBasicJSONContract = path.join(pathToContracts, "json", contractJSONFilename);

export const paths = {
  outputDir: outputDir,
  binOutputFile: binOutputFile,
  eraVMAssemblyOutputFile: eraVMAssemblyOutputFile,
  libraryDefault: libraryDefault,
  contractSolFilename: contractSolFilename,
  contractEraVMAssemblyFilename: contractEraVMAssemblyFilename,
  contractYulFilename: contractYulFilename,
  contractLlvmFilename: contractLlvmFilename,
  pathToOutputDir: pathToOutputDir,
  pathToContracts: pathToContracts,
  pathToBasicEraVMAssemblyContract: pathToBasicEraVMAssemblyContract,
  pathToBasicSolContract: pathToBasicSolContract,
  pathToBasicYulContract: pathToBasicYulContract,
  pathToBasicLlvmContract: pathToBasicLlvmContract,
  pathToBasicJSONContract: pathToBasicJSONContract,
};
