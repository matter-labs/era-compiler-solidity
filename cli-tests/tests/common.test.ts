import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToSolBinOutputFile, pathToSolAsmOutputFile} from "../src/helper";
import { paths } from '../src/entities';


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
        const tmpDirZkSolc = createTmpDirectory();
        const args = [
            `"${paths.pathToBasicSolContract}"`,
            `-O3`,
            `--bin`,
            `--output-dir`,
            `"${tmpDirZkSolc.name}"`
        ]; // potential issue on zksolc with full path on Windows cmd
        const result = executeCommand(zksolcCommand, args);
        

        it("Compiler run successful", () => {
            expect(result.output).toMatch(/(Compiler run successful.)/i);
        });

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output dir is created", () => {
            expect(isDestinationExist(tmpDirZkSolc.name)).toBe(true);
        });

        xit("Output file is created", () => { // a bug on windows
            expect(isDestinationExist(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(true);
        });

        it("the output file is not empty", () => {
            expect(isFileEmpty(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(false);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkSolc.removeCallback();
        });
    });

    //#1818
    describe(`Run ${zksolcCommand} with multiple output options from the help`, () => {
        const tmpDirZkSolc = createTmpDirectory();
        const args = [
            `"${paths.pathToBasicSolContract}"`,
            `-O3`,
            `--bin`,
            `--asm`,
            `--output-dir`,
            `"${tmpDirZkSolc.name}"`
        ]; // potential issue on zksolc with full path on Windows cmd
        const result = executeCommand(zksolcCommand, args);

        it("Compiler run successful", () => {
            expect(result.output).toMatch(/(Compiler run successful.)/i);
        });
        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });
        it("Output dir is created", () => {
            expect(isDestinationExist(tmpDirZkSolc.name)).toBe(true);
        });
        xit("Output files are created", () => { // a bug on windows
            expect(isDestinationExist(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(true);
            expect(isDestinationExist(pathToSolAsmOutputFile(tmpDirZkSolc.name))).toBe(true);
        });
        it("the output files are not empty", () => {
            expect(isFileEmpty(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(false);
            expect(isFileEmpty(pathToSolAsmOutputFile(tmpDirZkSolc.name))).toBe(false);
        });
        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkSolc.removeCallback();
        });
    });
});
