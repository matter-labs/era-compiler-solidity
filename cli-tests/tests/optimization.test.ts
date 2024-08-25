import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --optimization tests", () => {
  const zksolcCommand = 'zksolc';
  const optimization_args: string[] = [`0`, `1`, `2`, `3`, `s`, `z`];

  //id1752:I
  for (let i = 0; i < optimization_args.length; i++) {
    describe(`Run ${zksolcCommand} with -O${optimization_args[i]}`, () => {
      const args = [`${paths.pathToBasicSolContract}`, `-O${optimization_args[i]}`];
      const result = executeCommand(zksolcCommand, args);

      it("Valid command exit code = 0", () => {
        expect(result.exitCode).toBe(0);
      });

      it("--metadata-hash info is presented", () => {
        expect(result.output).toMatch(/(Compiler run successful)/i);
      });

    });
  }

  //id1752:II
  describe(`Run ${zksolcCommand} with -O${optimization_args[0]} and no input file`, () => {
    const args = [`-O${optimization_args[0]}`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--metadata-hash info is presented", () => {
      expect(result.output).toMatch(/(No input sources specified)/i);
    });

  });

  //id1752:III
  describe(`Run ${zksolcCommand} with wrong optimization option`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `-O99`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("--metadata-hash info is presented", () => {
      expect(result.output).toMatch(/(Unexpected optimization option|Invalid value for)/i);
    });

  });

});
