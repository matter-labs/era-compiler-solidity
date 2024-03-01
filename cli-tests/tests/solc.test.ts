import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --solc tests", () => {
    const zksolcCommand = 'zksolc';
    const solcCommand = 'solc';


    //id1748
    describe(`Run ${zksolcCommand} with --solc }`, () => {
        console.log(executeCommand('ls', [] ));
        console.log(executeCommand('cd .. && ls', [] ));
        const args = [`${paths.pathToBasicSolContract}`, `--solc`, `${paths.pathToCustomSolc}`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--metadata-hash info is presented", () => {
            expect(result.output).toMatch(/(Compiler run successful)/i);
        });
    });

});
