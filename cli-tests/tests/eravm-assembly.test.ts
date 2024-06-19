import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --eravm-assembly tests", () => {
  const zksolcCommand = 'zksolc';

  //id1745
  describe(`Run ${zksolcCommand} with --eravm-assembly by default`, () => {
    const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm-assembly`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--eravm-assembly output is presented", () => {
        expect(result.output).toMatch(/(Compiler run successful)/i);
        expect(result.output).toMatch(/(No output requested)/i);
    });
  });

  //id1822
  describe(`Run ${zksolcCommand} with double eravm options`, () => {
    const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm-assembly`, `--eravm-assembly`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
        expect(result.output).toMatch(/(The argument '--eravm-assembly' was provided more than once)/i);
    });
  });

  //id1823
  xdescribe(`Run ${zksolcCommand} with incompatible input format`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--eravm-assembly`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
        expect(result.output).toMatch(/(Error: Expected keyword "object")/i);
    });
  });

  //id1824
  describe(`Run ${zksolcCommand} with incompatible json modes`, () => { 
    const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm-assembly`, `--combined-json`, `anyarg`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
        expect(result.output).toMatch(/(Only one mode is allowed)/i);
    });
  });
});
