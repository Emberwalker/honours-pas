import * as Lint from "tslint";

export class Formatter extends Lint.Formatters.AbstractFormatter {
  // Derived from the formatter in tslint-loader
  public format(failures: Lint.RuleFailure[]): string {
    const outputLines = failures.map((failure) => {
      const failureString = failure.getFailure();
      const lineAndCharacter = failure.getStartPosition().getLineAndCharacter();
      const positionTuple = "[" + (lineAndCharacter.line + 1) + ", " + (lineAndCharacter.character + 1) + "]";
      const ruleString = " (" + failure.getRuleName() + ")";
      return positionTuple + ": " + failureString + ruleString;
    });
    return outputLines.join("\n") + "\n";
  }
}
