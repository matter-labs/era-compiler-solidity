import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --bin tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1747
  describe(`Run ${zksolcCommand} with --bin by default`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--bin`];
    const invalidArgs = [`--bin`];
    const result = executeCommand(zksolcCommand, args);
    const invalidResult = executeCommand(zksolcCommand, invalidArgs);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--bin output is presented", () => {
      expect(result.output).toMatch(/(bytecode: 0x)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });

    it("run invalid: zksolc --bin", () => {
      expect(invalidResult.output).toMatch(/(No input sources specified|Compilation aborted)/i);
    });

    it("Invalid command exit code = 1", () => {
      expect(invalidResult.exitCode).toBe(1);
    });

    it("Invalid solc exit code == Invalid zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, invalidArgs);
      expect(solcResult.exitCode).toBe(invalidResult.exitCode);
    });
  });

  //id1814
  describe(`Run ${zksolcCommand} with 2 same flags --bin --bin`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--bin`, `--bin`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(The argument '--bin' was provided more than once)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1815
  describe(`Run ${zksolcCommand} with --bin with wrong input format`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--bin`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Expected identifier but got 'StringLiteral')/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

});
