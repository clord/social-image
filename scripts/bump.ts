import { CLI, CLIUsingReadline } from './cli';

bump(CLIUsingReadline.create());

async function bump(cli: CLI) {
  try {
    cli.say('Starting bump');
    await cli.exec('cargo', ['test']);
    await cli.exec('cargo', ['package']);
    await cli.exec('cargo', ['clippy']);
    cli.say('Press `enter` to move to next step');
    await cli.prompt('1: make a branch for the release (e.g., bump-v0.6)');
    await cli.prompt('2: update Cargo.toml version (e.g., v0.6)');
    await cli.prompt(`3: update Cargo.lock by running: 
         cargo build`);
    await cli.prompt('4: commit changes, push up PR and get it to main');
    await cli.prompt(`5: tag main with version (e.g., v0.6); and push it:
         git push --tags`);
    cli.say('Updated!');
  } finally {
    cli.exit();
  }
}
