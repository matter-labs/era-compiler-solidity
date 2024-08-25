import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --libraries tests", () => {
    const zksolcCommand = 'zksolc';

    //id1750
    xdescribe(`Run ${zksolcCommand} with Sol + --libraries}`, () => {
        const args = [`${paths.pathToBasicSolContract}`, ` --libraries`, `${paths.libraryDefault}`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Compilation info is presented", () => {
            expect(result.output).toMatch(/(Compiler run successful)/i);
        });
    });

    //id1750:II
    describe(`Run ${zksolcCommand} without Sol and with --libraries}`, () => {
        const args = [`--libraries`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Compilation info is presented", () => {
            expect(result.output).toMatch(/(requires a value but none was supplied)/i);
        });
    });

});
