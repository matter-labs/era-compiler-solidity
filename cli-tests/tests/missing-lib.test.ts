import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --detect-missing-libraries tests", () => {
    const zksolcCommand = 'zksolc';

    //id1753
    describe(`Run ${zksolcCommand} with Sol + --detect-missing-libraries}`, () => {
        const args = [`${paths.pathToBasicSolContract}`, ` --detect-missing-libraries`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Compilation info is presented", () => {
            expect(result.output).toMatch(/(Missing deployable libraries detection mode is only supported in standard JSON mode.)/i);
        });
    });

    //id1753
    describe(`Run ${zksolcCommand} without Sol and with --detect-missing-libraries}`, () => {
        const args = [`--detect-missing-libraries`];
        const result = executeCommand(zksolcCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Compilation info is presented", () => {
            expect(result.output).toMatch(/(Missing deployable libraries detection mode is only supported in standard JSON mode.)/i);
        });
    });

});
