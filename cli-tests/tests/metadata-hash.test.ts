import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --metadata-hash tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1751:I
  describe(`Run ${zksolcCommand} with --metadata-hash default`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--metadata-hash=none`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("--metadata-hash info is presented", () => {
      expect(result.output).toMatch(/(Compiler run successful)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1751:II
  describe(`Run ${zksolcCommand} with --metadata-hash no arg`, () => {
    const args = [`${paths.pathToBasicSolContract}`, `--metadata-hash`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(requires a value but none was supplied)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

  //id1751:III
  describe(`Run ${zksolcCommand} with --metadata-hash no input file`, () => {
    const args = [`--metadata-hash=none`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 1", () => {
      expect(result.exitCode).toBe(1);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/(No input sources specified)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });
  });

});
