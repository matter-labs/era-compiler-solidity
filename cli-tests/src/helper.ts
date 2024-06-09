import * as fs from 'fs';
import { spawnSync } from "child_process";
import * as os from 'os';
import * as tmp from 'tmp';
import { paths } from './entities';
import * as path from 'path';


tmp.setGracefulCleanup();

export function executeCommand(command: string, args: string[]) {
  const result = spawnSync(command, args, { encoding: 'utf-8', shell: true, stdio: 'pipe' });
  return {
      exitCode: result.status,
      output: result.stdout.trim() || result.stderr.trim()
  };
}

export const isDestinationExist = (destination: string): boolean  => {
    let exitCode: number;
    if (os.platform() === 'win32') {
        exitCode = executeCommand('dir', [destination]).exitCode;
    } else {
        exitCode = executeCommand('ls', [destination]).exitCode;
    }
    return fs.existsSync(destination) && exitCode == 0;
};

export const isFileEmpty = (file: string): boolean  => {
    if (isDestinationExist(file)) {
        return (fs.readFileSync(file).length === 0);
    }
    console.log(`ERROR: file/path ${file} is not found.\n`)
};

export const createTmpDirectory = (name = 'tmp-XXXXXX'): tmp.DirResult => {
  if (!fs.existsSync(paths.pathToOutputDir)) {
    fs.mkdirSync(paths.pathToOutputDir, { recursive: true });
  } 
  return tmp.dirSync({ template: name, tmpdir: paths.pathToOutputDir, unsafeCleanup: true });
};

export const changeDirectoryPermissions = (directoryPath: string, permission: string): void => {
  let args: string[];
  let command: string;

  if ( os.platform() === 'win32' ) {
    command = 'icacls'
    switch (permission) {
      case 'r':
        args = [directoryPath, '/deny "Everyone:(OI)(CI)(W)" /grant:r "Everyone:(OI)(CI)(R)']; // Read-only
        break;
      case 'a':
        args = [directoryPath, '/grant "Everyone:(OI)(CI)(F)']; // All
        break;
    }
  } else {
    command = 'chmod -R'
    switch (permission) {
      case 'r':
        args = ['-wx', directoryPath]; // Read-only
        break;
      case 'a':
        args = ['+wx', directoryPath]; // All
        break;
    }
  }

  try {
    executeCommand(command, args);
  } catch (error) {
    console.error(`Error changing permissions for directory '${directoryPath}':`, error);
  }
};

export const pathToSolBinOutputFile = (destination: string): string  => {
  return path.join(destination, paths.contractSolFilename, paths.binOutputFile);
};

export const pathToSolEraVMAssemblyOutputFile = (destination: string): string  => {
  return path.join(destination, paths.contractSolFilename, paths.eraVMAssemblyOutputFile);
};

export const isOutputTheSame = (file: string, cliOutput: string): boolean => {
    const fileOutput = fs.readFileSync(file, 'utf-8');
    console.log("fileOutput: \n" + fileOutput);
    console.log("cliOutput: \n" + cliOutput);
    if (cliOutput.includes(fileOutput) || fileOutput.includes(cliOutput)) {
        return true
    } else {
        return false
    }
}
