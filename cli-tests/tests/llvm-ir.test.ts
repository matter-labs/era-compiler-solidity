import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --llvm-ir tests", () => {
  const zksolcCommand = 'zksolc';

  //id1744
  describe(`Run ${zksolcCommand} with --llvm-ir by default`, () => {
    const args = [`${paths.pathToBasicLlvmContract}`, `--llvm-ir`];
    const invalidArgs = [`--llvm-ir`, `anyarg`];
    const result = executeCommand(zksolcCommand, args);
    const invalidResult = executeCommand(zksolcCommand, invalidArgs);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Invalid command exit code = 1", () => {
      expect(invalidResult.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Compiler run successful. No output requested. Use --asm and --bin flags.)/i);
    });
  });

  //id1825
  describe(`Run ${zksolcCommand} with 2 same flags --llvm-ir --llvm-ir`, () => {
    const args = [`${paths.pathToBasicLlvmContract}`, `--llvm-ir`, `--llvm-ir`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(The argument '--llvm-ir' was provided more than once)/i);
    });
  });

  //id1826
  describe(`Run ${zksolcCommand} with --llvm-ir with wrong input format`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--llvm-ir`, `--bin`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(expected top-level entity)/i);
    });
  });

  //id1827:1
  describe(`Run ${zksolcCommand} with incompatible json modes --combined-json`, () => {
    const args = [`${paths.pathToBasicLlvmContract}`, `--llvm-ir`, `--combined-json`, `anyarg`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Only one mode is allowed at the same time)/i);
    });
  });

  //id1827:2
  describe(`Run ${zksolcCommand} with incompatible json modes --standard-json`, () => {
    const args = [`${paths.pathToBasicYulContract}`, `--llvm-ir`, `--standard-json`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(Only one mode is allowed at the same time)/i);
    });
  });

});
