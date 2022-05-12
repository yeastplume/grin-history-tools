const { spawn } = require('child_process');
const { spawnSync } = require('child_process');
const debug = require('debug')('grin-tools-web:jest.beforeall.js');
require('dotenv').config({ path: '.env' });


module.exports = async () => {
  return new Promise((resolve, reject) => {
    // console.log('current directory: ', __dirname);
    const env = {
      PATH: process.env.PATH,
      NODE_ENV: 'development',
      // NODE_ENV: 'test',
      PORT: process.env.PORT || 3000,
      BRIDALLIVE_RUN_SYNC: false,
    };
    console.log('\n$ killall grin-history-tools-web');
    spawnSync('killall', ['grin-history-tools-web'], { stdio: 'inherit' });
    console.log('\n$ cargo build --workspace');
    const build = spawnSync('cargo', ['build', '--workspace'], { stdio: 'inherit' });
    debug('build = %o', build);
    if (build.status !== 0) {
      reject(build.status);
      return;
    }
    const bot = 'target/debug/grin-history-tools-web';
    console.log('\n$ ', bot);
    const api = spawn(bot, [], {
      // cwd: path.join(__dirname),
      detached: true,
      // comment this to see logs inline w/ tests.
      stdio: ['ignore', 'inherit', 'inherit'],
      env,
    });

    api.on('error', (err) => {
      console.error('Failed to start ', bot);
      reject(err);
    });

    api.on('close', (code) => {
      console.log(`child process exited with code ${code}`);
      reject(code);
    });

    global.api = api;
    setTimeout(() => {
      console.log('started API ');
      resolve();
    }, 20); // wait 5 more seconds.
  });
};