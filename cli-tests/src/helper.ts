import * as fs from 'fs';
import { spawnSync } from "child_process";
import * as os from 'os';


export function executeCommand(command: string, args: string[]) {
  const result = spawnSync(command, args, { encoding: 'utf-8', shell: true, stdio: 'pipe' });
  return {
      exitCode: result.status,
      output: result.stdout.trim() || result.stderr.trim()
  };
}

export const isDestinationExist = (destination: string): boolean  => {
    return fs.existsSync(destination);
};

export const isFileEmpty = (file: string): boolean  => {
    if (isDestinationExist(file)) {
        return (fs.readFileSync(file).length === 0);
    }
};

export const createDirectory = (file: string): boolean => {
  try {
    fs.mkdirSync(file);
    return true;
  } catch (error) {
    return false;
  }
};

export const removeDirectory = (file: string): boolean => {
  if (fs.existsSync(file)) {
    const stats = fs.statSync(file);
    if (stats.isDirectory()) {
      try {
        fs.rmSync(file, { recursive: true });
        return true;
      } catch (error) {
      }
    } else {
      return false;
    }
  } else {
    return false;
  }
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
    command = 'chmod'
    switch (permission) {
      case 'r':
        args = ['-R 444', directoryPath]; // Read-only
        break;
      case 'a':
        args = ['-R 777', directoryPath]; // All
        break;
    }
  }

  try {
    executeCommand(command, args);
  } catch (error) {
    console.error(`Error changing permissions for directory '${directoryPath}':`, error);
  }
};
