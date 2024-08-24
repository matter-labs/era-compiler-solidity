import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --yul tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1743
  describe(`Run ${zksolcCommand} with --yul by default`, () => {
    const zksolcArgs = [`${paths.pathToBasicYulContract}`, `--yul`];
    const solcArgs = [`${paths.pathToBasicYulContract}`, `--strict-assembly`];
    const invalidArgs = ['--yul', 'anyarg'];
    const result = executeCommand(zksolcCommand, zksolcArgs);
    const invalidResult = executeCommand(zksolcCommand, invalidArgs);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--yul output is presented", () => {
      expect(result.output).toMatch(/(Compiler run successful)/i);
      expect(result.output).toMatch(/(No output requested)/i);
    });

    xit("solc exit code == zksolc exit code", () => {
        const solcResult = executeCommand(solcCommand, solcArgs);
        expect(solcResult.exitCode).toBe(result.exitCode);
    });

    it("Invalid command exit code = 1", () => {
      expect(invalidResult.exitCode).toBe(1);
    });

    it("Invalid solc exit code == Invalid zksolc exit code", () => { 
      const solcResult = executeCommand(solcCommand, invalidArgs);
      expect(solcResult.exitCode).toBe(invalidResult.exitCode);
    });
  });

  //id1820
  describe(`Run ${zksolcCommand} with double --yul options`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--yul`, `--yul`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(The argument '--yul' was provided more than once,)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1821
  describe(`Run ${zksolcCommand} with incompatible input format - solidity contract`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--yul`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Yul parsing)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1819:1
  describe(`Run ${zksolcCommand} with incompatible json modes --combined-json`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--yul`, `--combined-json`, `anyarg`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Only one mode is allowed at the same time:)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1819:2
  describe(`Run ${zksolcCommand} with incompatible json modes --standard-json`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--yul`, `--standard-json`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Only one mode is allowed at the same time:)/i);
    });
  });
});
