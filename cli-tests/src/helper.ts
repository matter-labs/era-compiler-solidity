import * as fs from 'fs';
import { spawnSync } from "child_process";


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

export const createDirecotry = (file: string): boolean => {
  try {
    fs.mkdirSync(file);
    console.log(`Folder '${file}' created successfully.`);
    return true;
  } catch (error) {
    console.error(`Error creating folder '${file}':`, error);
    return false;
  }
};

export const removeDirectory = (file: string): boolean => {
  if (fs.existsSync(file)) {
    // Check if it's a directory
    const stats = fs.statSync(file);
    if (stats.isDirectory()) {
      try {
        // Remove directory recursively
        fs.rmdirSync(file, { recursive: true });
        console.log(`Directory '${file}' removed successfully.`);
        return true; // Return true if removal was successful
      } catch (error) {
        console.error(`Error removing directory '${file}':`, error);
        return false; // Return false if there was an error during removal
      }
    } else {
      console.error(`'${file}' is not a directory.`);
      return false; // Return false if the path exists but is not a directory
    }
  } else {
    console.error(`Directory '${file}' does not exist.`);
    return false; // Return false if the directory does not exist
  }
};

export const changeDirectoryPermissions = (directoryPath: string, permission: string): boolean => {
  let mode: number;

  switch (permission) {
    case 'r':
      mode = 0o444; // Read-only
      break;
    case 'w':
      mode = 0o222; // Write-only
      break;
    case 'r+w':
      mode = 0o666; // Read-write
      break;
    default:
      console.error('Invalid permission. Please choose "r", "w", or "r+w".');
      return true;
  }

  try {
    fs.chmodSync(directoryPath, mode);
    console.log(`Permissions changed for directory '${directoryPath}' to '${permission}'.`);
  } catch (error) {
    console.error(`Error changing permissions for directory '${directoryPath}':`, error);
  }
};

