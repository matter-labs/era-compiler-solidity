import {executeCommand, isDestinationExist, changeDirectoryPermissions, createTmpDirectory} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

describe("Set of --output-dir tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';
  
  
  //id1749:I
  describe(`Run ${zksolcCommand} with --output-dir by default`, () => {
    const tmpDirZkSolc = createTmpDirectory();
    const tmpDirSolc = createTmpDirectory();
    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirZkSolc.name}`];
    const result = executeCommand(zksolcCommand, args);

    it("A dir is created", () => {
      expect(isDestinationExist(tmpDirZkSolc.name)).toBe(true);
      tmpDirZkSolc.removeCallback();
    });

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful\. Artifact\(s\) can be found in directory/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirSolc.name}`];
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
      tmpDirSolc.removeCallback();
    });
    
    
  });

  //id1749:II
  describe(`Run ${zksolcCommand} with --output-dir invalid arg - no path`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`];
    const invalidResult = executeCommand(zksolcCommand, args);

    it("run invalid: zksolc --output-dir", () => {
      expect(invalidResult.output).toMatch(/(error: The argument '--output-dir <output-directory>' requires a value but none was supplied)/i);
    });

    it("Invalid command exit code = 1", () => {
      expect(invalidResult.exitCode).toBe(1);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(invalidResult.exitCode);
    });
  });

  //id1749:III
  describe(`Run ${zksolcCommand} with --output-dir invalid args - no source`, () => {
    const tmpDirZkSolc = createTmpDirectory();
    const tmpDirSolc = createTmpDirectory();
    const args = [`--bin`, `--output-dir`, `${tmpDirZkSolc.name}`];
    const result = executeCommand(zksolcCommand, args);

    it("exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Compiler warning/error is presented", () => {
      expect(result.output).toMatch(/No input sources specified\.\s*Error\(s\) found\. Compilation aborted/i);
      tmpDirZkSolc.removeCallback();
    });

    it("solc exit code == zksolc exit code", () => {
      const args = [`--bin`, `--output-dir`, `${tmpDirSolc.name}`]
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
      tmpDirSolc.removeCallback();
    });
  });

  //id1813
  describe(`Run ${zksolcCommand} with --output-dir - specific symbols`, () => {
    const tmpDirZkSolc = createTmpDirectory(`File!and#$%-XXXXXX`);
    const tmpDirSolc = createTmpDirectory(`File!and#$%-XXXXXX`);
    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirZkSolc.name}`];
    const result = executeCommand(zksolcCommand, args);

    it("Exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Custom dir is created", () => {
      expect(isDestinationExist(tmpDirZkSolc.name)).toBe(true);
      tmpDirZkSolc.removeCallback();
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirSolc.name}`];
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
      tmpDirSolc.removeCallback();
    });
  });

  //id1812 - different behaviour on CI on Linux
  if (os.platform() !== 'linux') {
    describe(`Run ${zksolcCommand} with --output-dir - output-dir - wrong permissions`, () => {
      const tmpDirZkSolc = createTmpDirectory();
      // const tmpDirSolc = createTmpDirectory();
      changeDirectoryPermissions(tmpDirZkSolc.name, 'r');
      const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirZkSolc.name}`];
      const result = executeCommand(zksolcCommand, args);

      it("Valid command exit code = 1", () => {
        expect(result.exitCode).toBe(1);
      });

      it("--output-dir output is presented", () => {
        expect(result.output).toMatch(/(Permission denied|Access is denied)/i);
        tmpDirZkSolc.removeCallback();
      });

      // Exit code should be the same
      // xit("solc exit code == zksolc exit code", () => {
      //   const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${tmpDirSolc.name}`];
      //   const solcResult = executeCommand(solcCommand, args);
      //   expect(solcResult.exitCode).toBe(result.exitCode);
      //   tmpDirSolc.removeCallback();
      // });
    });
  }
});
