import { CLI, CLIUsingReadline } from './cli';

bump(CLIUsingReadline.create());

async function bump(cli: CLI) {
  try {
    cli.say('Press `enter` to mark step complete and move on');
    await cli.prompt('0: Switch to main; and fetch from origin');
    try {
      await cli.exec('cargo', ['test']);
      await cli.exec('cargo', ['package']);
      await cli.exec('cargo', ['clippy']);
    } catch (error) {
      cli.say('Make sure main is clean');
      cli.fail();
    }
    await cli.prompt('1: make a branch for the release (e.g., bump-v0.6)');
    await cli.prompt('2: in branch, update Cargo.toml version (e.g., v0.6)');
    await cli.prompt(`3: in branch, update Cargo.lock by running: 
         cargo build`);
    await cli.prompt('4: commit changes, push up PR and get it to main');
    await cli.prompt(`5: tag main with version (e.g., v0.6); and push it:
         git push --tags`);
    cli.say('Updated!');
  } catch (e) {
    console.error(e);
    cli.fail();
  } finally {
    cli.exit();
  }
}
