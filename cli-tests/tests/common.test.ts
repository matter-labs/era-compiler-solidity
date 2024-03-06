import {executeCommand, isDestinationExist, isFileEmpty} from "../src/helper";
import { paths } from '../src/entities';

jest.retryTimes(2);

describe("Common tests", () => {
    const zksolcCommand = 'zksolc';
    const solcCommand = 'solc';

    //id1762
    describe(`Run ${zksolcCommand} without any args`, () => {
        const args = [''];
        const result = executeCommand(zksolcCommand, args);

        it("Info with help is presented", () => {
            expect(result.output).toMatch(/(No input sources specified|Error(s) found.)/i);
        });

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("solc exit code == zksolc exit code", () => {
            const solcResult = executeCommand(solcCommand, args);
            expect(solcResult.exitCode).toBe(result.exitCode);
        });
    });

    //#1713
    describe(`Default run of ${zksolcCommand} from the help`, () => {
        const args = [
            `"${paths.pathToBasicSolContract}"`,
            `-O3`,
            `--bin`,
            `--output-dir`,
            `"${paths.pathToOutputDir}"`
        ]; // potential issue on zksolc with full path on Windows cmd
        const result = executeCommand(zksolcCommand, args);
        

        it("Compiler run successful", () => {
            expect(result.output).toMatch(/(Compiler run successful.)/i);
        });

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output dir is created", () => {
            expect(isDestinationExist(paths.pathToOutputDir)).toBe(true);
        });

        xit("Output file is created", () => { // a bug on windows
            expect(isDestinationExist(paths.pathToSolBinOutputFile)).toBe(true);
        });

        it("the output file is not empty", () => {
            expect(isFileEmpty(paths.pathToSolBinOutputFile)).toBe(false);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });
    });

    //#1818
    describe(`Run ${zksolcCommand} with multiple output options from the help`, () => {
        const args = [
            `"${paths.pathToBasicSolContract}"`,
            `-O3`,
            `--bin`,
            `--asm`,
            `--output-dir`,
            `"${paths.pathToOutputDir}"`
        ]; // potential issue on zksolc with full path on Windows cmd

        console.log(__dirname);
        console.log(zksolcCommand);
        console.log(args);
        console.log(paths.pathToSolBinOutputFile);

        const result = executeCommand(zksolcCommand, args);

        it("Compiler run successful", () => {
            expect(result.output).toMatch(/(Compiler run successful.)/i);
        });
        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });
        it("Output dir is created", () => {
            expect(isDestinationExist(paths.pathToOutputDir)).toBe(true);
        });
        xit("Output files are created", () => { // a bug on windows
            expect(isDestinationExist(paths.pathToSolBinOutputFile)).toBe(true);
            expect(isDestinationExist(paths.pathToSolAsmOutputFile)).toBe(true);
        });
        it("the output files are not empty", () => {
            expect(isFileEmpty(paths.pathToSolBinOutputFile)).toBe(false);
            expect(isFileEmpty(paths.pathToSolAsmOutputFile)).toBe(false);
        });
        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });
    });
});
