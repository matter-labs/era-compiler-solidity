import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


//id1743
describe("Run with --yul by default", () => {
  const command = `zksolc ${paths.pathToBasicYulContract} --yul`;
  const result = executeCommand(command);
  const commandInvalid = 'zksolc --yul';
  const resultInvalid = executeCommand(commandInvalid);

  it("Valid command exit code = 0", () => {
    expect(result.exitCode).toBe(0);
  });

  it("--yul output is presented", () => {
    expect(result.output).toMatch(/(Compiler run successful)/i);
    expect(result.output).toMatch(/(No output requested)/i);
  });


  xit("solc exit code == zksolc exit code", () => { // issue with solc compilation
      const command = `solc ${paths.pathToBasicYulContract} --yul`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });

  it("run invalid: zksolc --yul", () => {
    expect(resultInvalid.output).toMatch(/(The input file is missing)/i);
  });
  it("Invalid command exit code = 1", () => {
    expect(resultInvalid.exitCode).toBe(1);
  });

  it("Invalid solc exit code == Invalid zksolc exit code", () => { 
    const command = 'solc --yul';
    const solcResult = executeCommand(command);
    expect(solcResult.exitCode).toBe(resultInvalid.exitCode);
  });
});

//id1820
describe("Run with double yul options", () => {
  const command = `zksolc ${paths.pathToBasicYulContract} --yul --yul`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--yul error is presented", () => {
    expect(result.output).toMatch(/(The argument '--yul' was provided more than once,)/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicYulContract} --yul --yul`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});

//id1821
describe("Run with incompatible input format - solidity contract", () => {
  const command = `zksolc ${paths.pathToBasicSolContract} --yul`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--yul error is presented", () => {
    expect(result.output).toMatch(/(Error: Expected keyword "object")/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicSolContract} --yul`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});

//id1819:1
describe("Run with incompatible json modes --combined-json", () => {
  const command = `zksolc ${paths.pathToBasicYulContract} --yul --combined-json anyarg`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--yul error is presented", () => {
    expect(result.output).toMatch(/(Only one modes is allowed at the same time:)/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicYulContract} --yul --combined-json anyarg`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});

//id1819:2
describe("Run with incompatible json modes --standard-json", () => {
  const command = `zksolc ${paths.pathToBasicYulContract} --yul --standard-json`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--yul error is presented", () => {
    expect(result.output).toMatch(/(Only one modes is allowed at the same time:)/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicYulContract} --yul --standard-json`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});


