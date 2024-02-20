import * as path from 'path';

const outputDir = 'artifacts';
const binExtension = ':C.zbin';
const asmExtension = ':C.zasm';
const llvmExtension = ':C.bc';
const contractSolFilename = 'contract.sol';
const contractYulFilename = 'contract.yul';
const contractZkasmFilename = 'contract.zkasm';
const contractLlvmFilename = 'contract.ll';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicYulContract = path.join(pathToContracts, 'yul', contractYulFilename);
const pathToBasicZkasmContract = path.join(pathToContracts, 'zkasm', contractZkasmFilename);
const pathToBasicSolContract = path.join(pathToContracts, 'solidity', contractSolFilename);
const pathToBasicLlvmContract = path.join(pathToContracts, "llvm", contractLlvmFilename);
const pathToSolBinOutputFile = path.join(pathToOutputDir, contractSolFilename + binExtension);
const pathToSolAsmOutputFile = path.join(pathToOutputDir, contractSolFilename + asmExtension);
const pathToLlvmContractsFile = path.join(pathToOutputDir, contractLlvmFilename + llvmExtension);

export const paths = {
  outputDir: outputDir,
  binExtension: binExtension,
  asmExtension: asmExtension,
  contractSolFilename: contractSolFilename,
  contractZkasmFilename: contractZkasmFilename,
  contractYulFilename: contractYulFilename,
  pathToOutputDir: pathToOutputDir,
  pathToContracts: pathToContracts,
  pathToBasicZkasmContract: pathToBasicZkasmContract,
  pathToBasicSolContract: pathToBasicSolContract,
  pathToBasicYulContract: pathToBasicYulContract,
  pathToBasicLlvmContract: pathToBasicLlvmContract,
  pathToSolBinOutputFile: pathToSolBinOutputFile,
  pathToSolAsmOutputFile: pathToSolAsmOutputFile,
  pathToLlvmOutputFile: pathToLlvmContractsFile
};
