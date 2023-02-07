const path = require('path');
const programDir = path.join(__dirname, 'programs/sporting-labs-stake-pool');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'sdk', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'sporting_labs_stake_pool',
  programId: '41MZASop6YTB5UmYNSDFxFJ4QYEMeDY9f7WcABLUmfoB',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};