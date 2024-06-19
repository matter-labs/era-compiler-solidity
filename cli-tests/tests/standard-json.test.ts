import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --standard-json tests", () => {
  const zksolcCommand = 'zksolc';
  const solcCommand = 'solc';

  //id1741:I
  xdescribe(`Run ${zksolcCommand} with --standard-json contract.json}`, () => {
    const args = [`--standard-json`, `${paths.pathToBasicJSONContract}`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/({"sources":{"A":{"i)/i);
    });

    it("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });

  });

  //id1828
  xdescribe(`Run ${zksolcCommand} with --standard-json incompatible input}`, () => {
    const args = [`--standard-json`, `${paths.pathToBasicYulContract}`];
    const result = executeCommand(zksolcCommand, args);

    it("Valid command exit code = 0", () => {
      expect(result.exitCode).toBe(0);
    });

    it("Error message is presented", () => {
      expect(result.output).toMatch(/Input files must be passed via standard JSON input/i);
    });

    xit("solc exit code == zksolc exit code", () => {
      const solcResult = executeCommand(solcCommand, args);
      expect(solcResult.exitCode).toBe(result.exitCode);
    });

  });

});
