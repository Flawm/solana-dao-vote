import { establishConnection, establishPayer, testGPA } from './main';

async function main() {
  // Establish connection to the cluster
  await establishConnection();

  // our dev wallet
  await establishPayer();

  await testGPA();

  console.log('Success');
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
