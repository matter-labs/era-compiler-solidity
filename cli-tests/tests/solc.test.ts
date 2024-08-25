import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --solc tests", () => {
    const zksolcCommand = 'zksolc';

    var os = require('os');

    let pathToCustomSolc = executeCommand('which', ['solc']).output;
    if ( os.platform() === 'win32' ) {
        pathToCustomSolc = executeCommand('where', ['solc']).output;
    }

    //id1748
    describe(`Run ${zksolcCommand} with --solc }`, () => {
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
