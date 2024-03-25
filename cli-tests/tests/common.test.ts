import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToSolBinOutputFile, pathToSolAsmOutputFile} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';


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
        ]; 
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

        it("Output file is created", () => {
            // if ( os.platform() === 'win32' ) {
            //     console.log(executeCommand('dir', [tmpDirZkSolc.name, '/B']).output)
            // }
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
    describe.only(`Run ${zksolcCommand} with multiple output options from the help`, () => {
        const tmpDirZkSolc = createTmpDirectory();
        const args = [
            `"${paths.pathToBasicSolContract}"`,
            `-O3`,
            `--bin`,
            `--asm`,
            `--output-dir`,
            `"${tmpDirZkSolc.name}"`
        ];
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
        it("Output files are created", () => {
            // Remove if () {} after the bugfix
            if ( os.platform() === 'win32' ) {
                console.log("Expected file: " + pathToSolAsmOutputFile(tmpDirZkSolc.name))
                console.log("Actual file: " + executeCommand('dir', [tmpDirZkSolc.name, '/B']).output)
            }
            expect(isDestinationExist(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(true);
            expect(isDestinationExist(pathToSolAsmOutputFile(tmpDirZkSolc.name))).toBe(true);
        });
        it("The output files are not empty", () => {
            // Remove if () {} after the bugfix
            if ( os.platform() === 'win32' ) {
                const args_cmd = [
                    `"${paths.pathToBasicSolContract}"`,
                    `-O3`,
                    `--bin`,
                    `--asm`
                ];
                console.log(`The output file: ${ pathToSolBinOutputFile(tmpDirZkSolc.name) } contains: \n`
                    + executeCommand('type', [pathToSolBinOutputFile(tmpDirZkSolc.name)]).output);
                console.log(`The output file should contain: \n`
                    + executeCommand(zksolcCommand, args_cmd).output);
            }
            expect(isFileEmpty(pathToSolBinOutputFile(tmpDirZkSolc.name))).toBe(false);
            expect(isFileEmpty(pathToSolAsmOutputFile(tmpDirZkSolc.name))).toBe(false);
        });
        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkSolc.removeCallback();
        });
    });
});
