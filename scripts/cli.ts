import { stdin, stderr, stdout } from 'process';
import { spawn } from 'child_process';
import * as readline from 'readline';

export interface CLI {
  say(message: string): void;
  choose<T>(message: string, choices: { match: string; value: T }[]): Promise<T>;
  prompt(message: string): Promise<void>;
  exec(command: string, args: string[]): Promise<void>;
  exit(): void;
}

// One implementation, using Node.js readline
export class CLIUsingReadline implements CLI {
  static create(): CLIUsingReadline {
    return new CLIUsingReadline();
  }

  private constructor() {}

  // That's all we need for now to implement the behavior we want
  private readline = readline.createInterface({
    input: stdin,
    output: stdout,
  });

  say(message: string): void {
    stdout.write(`${message}\n`);
  }

  prompt(message: string): Promise<void> {
    return new Promise((resolve) => this.readline.question(message, () => resolve()));
  }

  exec(command: string, args: string[]): Promise<void> {
    return new Promise((resolve, reject) => {
      const run = spawn(command, args);
      run.stdout.pipe(stdout);
      run.stderr.pipe(stderr);
      run.on('close', resolve);
      run.on('error', reject);
    });
  }

  async choose<T>(message: string, choices: { match: string; value: T }[]): Promise<T> {
    return new Promise((resolve) =>
      this.readline.question(`${message}\n`, (answer) => {
        const choice = choices.find(({ match }) => match.trim().toLowerCase() === answer.trim().toLowerCase());

        // Ask the question again until we got a valid answer
        if (!choice) {
          this.choose(message, choices).then(resolve);
          return;
        }

        resolve(choice.value);
      })
    );
  }

  exit(): void {
    this.readline.close();
  }
}
