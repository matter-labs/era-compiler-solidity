import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

describe("Set of --solc tests", () => {
    const zksolcCommand = 'zksolc';
    
    let pathToCustomSolc = executeCommand('which', ['solc']);
        if ( os.platform() === 'win32' ) {
            pathToCustomSolc = executeCommand('where', ['solc']);
        }


    //id1748
    describe(`Run ${zksolcCommand} with --solc }`, () => {
        console.log(executeCommand('ls', [`${pathToCustomSolc}`] ));
        const args = [`${paths.pathToBasicSolContract}`, `--solc`, `${pathToCustomSolc}`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--metadata-hash info is presented", () => {
            expect(result.output).toMatch(/(Compiler run successful)/i);
        });
    });

});
