import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --asm tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1746
  describe(`Run ${zksolcCommand} with --asm by default`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--asm`];
    const invalidArgs = [`--asm`];
    const result = executeCommand(zksolcCommand, args);
    const invalidResult = executeCommand(zksolcCommand, invalidArgs);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--asm output is presented", () => {
        expect(result.output).toMatch(/(__entry:)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });

    it("run invalid: zksolc --asm", () => {
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

  //id1816
  describe(`Run ${zksolcCommand} with 2 same flags --asm --asm`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--asm`, `--asm`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
        expect(result.output).toMatch(/(The argument '--asm' was provided more than once)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1817
  describe(`Run ${zksolcCommand} with --asm with wrong input format`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--asm`];
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
