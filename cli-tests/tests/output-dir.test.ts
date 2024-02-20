import * as path from "path";
import {executeCommand, isDestinationExist, removeDirectory, changeDirectoryPermissions, createDirecotry} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --output-dir tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1749
  describe(`Run ${zksolcCommand} with --output-dir by default`, () => {

    const args = [`${paths.pathToBasicSolContract}`, `--bin --output-dir './tmp/'`];
    const invalidArgs = [`--output-dir`];
    const invalidArgsNoInpSource = [`--output-dir test`]
    const result = executeCommand(zksolcCommand, args);
    const invalidResult = executeCommand(zksolcCommand, invalidArgs);
    const invalidResultNoInpSource = executeCommand(zksolcCommand, invalidArgsNoInpSource);

    it("tmp dir is created", () => {
      expect(isDestinationExist(paths.testTmpDir)).toBe(true);
    });

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful\. Artifact\(s\) can be found in directory "\.\/tmp\/"\./i);
    });

    it("solc exit code == zksolc exit code", () => {
      const removedTmp = removeDirectory(paths.testTmpDir);
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });

    it("run invalid: zksolc --output-dir", () => {
      expect(invalidResult.output).toMatch(/(error: The argument '--output-dir <output-directory>' requires a value but none was supplied)/i);
    });

    it("Invalid command exit code = 1", () => {
      expect(invalidResult.exitCode).toBe(1);
    });

    it("Invalid solc exit code == Invalid zksolc exit code for zksolc --output-dir", () => {
      const solcResult = executeCommand(solcCommand, invalidArgs);
      expect(solcResult.exitCode).toBe(invalidResult.exitCode);
    });

    it("run invalid: zksolc --output-dir test", () => {
      expect(invalidResultNoInpSource.exitCode).toBe(1);
    });

    it("run invalid: zksolc --output-dir output result", () => {
      expect(invalidResultNoInpSource.output).toMatch(/No input sources specified\.\s*Error\(s\) found\. Compilation aborted/i);
    });

    it("Invalid solc exit code == Invalid zksolc exit code for zksolc --output-dir", () => {
      const solcResult = executeCommand(solcCommand, invalidArgsNoInpSource);
      expect(solcResult.exitCode).toBe(invalidResultNoInpSource.exitCode);
    });


  });

  //id1813
  describe(`Run ${zksolcCommand} with --output-dir - specific symbols`, () => {

    const args = [`${paths.pathToBasicSolContract}`, `--bin --output-dir './#@#$^%&*( wqerqwURU/'`];
    const result = executeCommand(zksolcCommand, args);

    it("tmp dir is created", () => {
      expect(isDestinationExist(paths.testTmpDir)).toBe(true);
    });

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/Compiler run successful\. Artifact\(s\) can be found in directory ".*"\./i);
    });

    it("solc exit code == zksolc exit code", () => {
      const removedTmp = removeDirectory(paths.DirSpecSymb);
      console.log(paths.DirSpecSymb);
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1812
  describe(`Run ${zksolcCommand} with --output-dir - output-dir - wrong permissions`, () => {
    //folder parameters to be created and deleted after the test
    const testDireName = '/test_to_delete';
    //create test folder
    const createNewDir = createDirecotry(paths.TestRootDir + testDireName)
    //change permissions to read-only
    const changePermissions = changeDirectoryPermissions(paths.TestRootDir + testDireName, 'r');

    const args =  [`${paths.pathToBasicSolContract}`, ` --bin --output-dir '${testDireName}/test'`];

    const result = executeCommand(zksolcCommand, args);

    it("tmp dir is NOT created", () => {
      expect(isDestinationExist(paths.TestRootDir + testDireName + '/test')).toBe(false);
    });

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--output-dir output is presented", () => {
      expect(result.output).toMatch(/(Read-only file system)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      console.log(solcCommand + args);
      expect(solcResult.exitCode).toBe(2);
    });

    afterAll(() => {
      const changePermissions = changeDirectoryPermissions(paths.TestRootDir + testDireName, 'r+w');
      const removedTmp = removeDirectory(paths.TestRootDir + testDireName);
    });
  });



});
