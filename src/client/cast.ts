import { establishConnection, establishPayer, castVote } from './main';

async function main() {
  // Establish connection to the cluster
  await establishConnection();

  // our dev wallet
  await establishPayer();

  // run the contract
  await castVote();

  console.log('Success');
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
