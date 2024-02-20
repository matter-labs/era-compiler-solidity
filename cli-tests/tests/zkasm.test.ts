import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --zkasm tests", () => {
  const zksolcCommand = 'zksolc';

  //id1745
  describe(`Run ${zksolcCommand} with --zkasm by default`, () => {
    const args = [`${paths.pathToBasicZkasmContract}`, `--zkasm`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--zkasm output is presented", () => {
        expect(result.output).toMatch(/(Compiler run successful)/i);
        expect(result.output).toMatch(/(No output requested)/i);
    });
  });

  //id1822
  describe(`Run ${zksolcCommand} with double zkasm options`, () => {
    const args = [`${paths.pathToBasicZkasmContract}`, `--zkasm`, `--zkasm`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--zkasm error is presented", () => {
        expect(result.output).toMatch(/(The argument '--zkasm' was provided more than once)/i);
    });
  });

  //id1823
  xdescribe(`Run ${zksolcCommand} with incompatible input format`, () => { // !issue because it compiles with incompatible input format
    const args = [`${paths.pathToBasicSolContract}`, `--zkasm`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--zkasm error is presented", () => {
        expect(result.output).toMatch(/(Error: Expected keyword "object")/i);
    });
  });

  //id1824
  describe(`Run ${zksolcCommand} with incompatible json modes`, () => { 
    const args = [`${paths.pathToBasicZkasmContract}`, `--zkasm`, `--combined-json`, `anyarg`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--zkasm error is presented", () => {
        expect(result.output).toMatch(/(Only one modes is allowed at the same time: Yul, LLVM IR, EraVM assembly, combined JSON, standard JSON.)/i);
    });
  });
});
