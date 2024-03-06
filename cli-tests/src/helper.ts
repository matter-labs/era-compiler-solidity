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

export function isDestinationExist(destination: string): boolean {
    let result: boolean = fs.existsSync(destination);
    console.log("Destination exists: " + result);
    return result;
}

export function isFileEmpty(file: string): boolean {
    let result: boolean = true;
    if (fs.existsSync(file)) {
      console.log("File exists");
      console.log("readfile: ");
      let readfile: Buffer = fs.readFileSync(file);
      console.log("IM HERE IM HERE IM HERE 1");
      console.log(readfile.length == 0);
      console.log("IM HERE IM HERE IM HERE 2");
      result = readfile.length == 0;
      console.log("IM HERE IM HERE IM HERE 3");
    }
    return result;
}

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
