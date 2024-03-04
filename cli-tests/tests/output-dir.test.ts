import {executeCommand, isDestinationExist, removeDirectory, changeDirectoryPermissions, createDirectory} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --output-dir tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1749:I
  describe(`Run ${zksolcCommand} with --output-dir by default`, () => {

    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${paths.pathToOutputDir}`];
    const result = executeCommand(zksolcCommand, args);

    it("A dir is created", () => {
      expect(isDestinationExist(paths.pathToOutputDir)).toBe(true);
    });

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful\. Artifact\(s\) can be found in directory/i);
    });

    it("solc exit code == zksolc exit code", () => {
      expect(removeDirectory(paths.pathToOutputDir)).toBe(true);
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
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
    const args = [`--bin`, `--output-dir`, `${paths.pathToOutputDir}`]
    const result = executeCommand(zksolcCommand, args);

    it("exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Compiler warning/error is presented", () => {
      expect(result.output).toMatch(/No input sources specified\.\s*Error\(s\) found\. Compilation aborted/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1813
  describe(`Run ${zksolcCommand} with --output-dir - specific symbols`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${paths.pathToBadOutputDir}`];
    const result = executeCommand(zksolcCommand, args);

    it("Exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Custom dir is created", () => {
      expect(isDestinationExist(paths.pathToBadOutputDir)).toBe(true);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful/i);
    });

    it("solc exit code == zksolc exit code", () => {
      expect(removeDirectory(paths.pathToBadOutputDir)).toBe(true);
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
      expect(removeDirectory(paths.pathToBadOutputDir)).toBe(true);
    });
  });

  //id1812
  describe(`Run ${zksolcCommand} with --output-dir - output-dir - wrong permissions`, () => {
    createDirectory(paths.pathToCustomOutputDir)
    changeDirectoryPermissions(paths.pathToCustomOutputDir, 'r');
    const args =  [`${paths.pathToBasicSolContract}`, `--bin`, `--output-dir`, `${paths.pathToCustomOutputDir}`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/(Permission denied|Access is denied)/i);
    });

    // Exit code should be the same
    xit("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);

    });
  });

  afterAll(() => {
    changeDirectoryPermissions(paths.pathToCustomOutputDir, 'a');
    removeDirectory(paths.pathToCustomOutputDir);
    removeDirectory(paths.pathToOutputDir);
    removeDirectory(paths.pathToBadOutputDir);
  });

});
