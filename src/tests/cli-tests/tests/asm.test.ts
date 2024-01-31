import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


//id1746
describe("Run with --asm by default", () => {
  const command = `zksolc ${paths.pathToBasicSolContract} --asm`;
  const result = executeCommand(command);
  const commandInvalid = 'zksolc --asm';
  const resultInvalid = executeCommand(commandInvalid);

  it("Valid command exit code = 0", () => {
    expect(result.exitCode).toBe(0);
  });

  it("--asm output is presented", () => {
      expect(result.output).toMatch(/(__entry:)/i);
  });


  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicSolContract} --asm`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });

  it("run invalid: zksolc --asm", () => {
    expect(resultInvalid.output).toMatch(/(No input sources specified|Compilation aborted)/i);
  });
  it("Invalid command exit code = 1", () => {
    expect(resultInvalid.exitCode).toBe(1);
  });

  it("Invalid solc exit code == Invalid zksolc exit code", () => {
    const command = 'solc --asm';
    const solcResult = executeCommand(command);
    expect(solcResult.exitCode).toBe(resultInvalid.exitCode);
  });
});

//id1816
describe("Run with 2 same flags --asm --asm", () => {
  const command = `zksolc ${paths.pathToBasicSolContract} --asm --asm`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--asm error is presented", () => {
      expect(result.output).toMatch(/(The argument '--asm' was provided more than once)/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicSolContract} --asm --asm`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});

//id1817
describe("Run with --asm with wrong input format", () => {
  const command = `zksolc ${paths.pathToBasicYulContract} --asm`;
  const result = executeCommand(command);

  it("Valid command exit code = 1", () => {
    expect(result.exitCode).toBe(1);
  });

  it("--asm error is presented", () => {
      expect(result.output).toMatch(/(Expected identifier but got 'StringLiteral')/i);
  });

  it("solc exit code == zksolc exit code", () => {
      const command = `solc ${paths.pathToBasicYulContract} --asm`;
      const solcResult = executeCommand(command);
      expect(solcResult.exitCode).toBe(result.exitCode);
  });
});
