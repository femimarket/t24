
if [[ -z "${NETWORK}" ]]; then
  echo specify NETWORK variable - mainnet,testnet,localnet
  exit 1
else
  export NEAR_ENV=$NETWORK
fi

if [[ -z "${NEAR_DIR}" ]]; then
  eval NEAR_DIR="~/.near/$NETWORK"
else
  NEAR_DIR="${NEAR_DIR}"
fi

if [ "$NETWORK" = "testnet" ]; then
  top_level_account="testnet"
  root="mintspace2"
elif [ "$NETWORK" == "mainnet" ]; then
  top_level_account="near"
  root="mintbase1"
elif [ "$NETWORK" == "localnet" ]; then
  top_level_account="near"
  root="test"
else
  echo "invalid network $NETWORK"
  exit 1
fi

root_account="$root.$top_level_account" # MUST ALREADY EXIST WITH LOCAL CRED
contract_account="t24.$root_account"

function setup() {
  build;
  near config add-connection --network-name localnet --connection-name localnet --rpc-url http://localhost:3030 --wallet-url https://testnet.mynearwallet.com/ --explorer-transaction-url https://explorer.testnet.near.org/transactions/
}

function create_localnet() {
  near config add-connection --network-name localnet --connection-name localnet --rpc-url http://localhost:3030 --wallet-url https://testnet.mynearwallet.com/ --explorer-transaction-url https://explorer.testnet.near.org/transactions/
}

function login() {
  near account import-account using-private-key
}

function build() {
  cargo b -r -p t24_near --target wasm32-unknown-unknown &&
  cargo b -r -p t24_near_indexer
}

function run_indexer() {
    ./target/release/t24_near_indexer
}

#near account import-account using-private-key ed25519:49PtFv2iorSN7XnaxKmwiXwBoM328Eim8Rg97rhgiR5BgW94auKAgd8W8nkLmt6HvmCvC7HhgtMCEc59KRLUQwDA network-config localnet

function deploy() {
    build &&
#    near call --network-id $NETWORK --accountId $root_account --gas 300000000000000 $contract_account init "{\"owner\": \"test.near\"}"


    near deploy $contract_account /home/u/Documents/RustOver/t24/target/wasm32-unknown-unknown/release/t24_near.wasm --initFunction init --initArgs "{\"owner\": \"$root_account\"}" --network-id $NETWORK
}

function create_accounts() {
    for i in $contract_account; do
      #      ((z=z%N)); ((z++==0)) && wait
      str="near create-account $i --masterAccount $root_account --initialBalance 20 --network-id $NETWORK"
      echo running $str
      eval $str
    done
}

question=$(
  cat <<EOF
Type number
(-4) run indexer (from scratch)
(-3) run indexer
(-2) build contracts
(-1) login
(0) create create localnet
(1) build
(2) run indexer
(3) deploy contracts
(4) create accounts
EOF
)

function programa {
  if [ -n "$1" ]; then
    response=$1
  else
    echo "$question"
    read -r response
    echo "you chose $response"
  fi

  case $response in
  -4)
    echo "are you sure? y/n"
    read -r answer
    if [ $answer = 'y' ]; then
      run_stateful_indexer
    fi
    programa
    ;;
  -3)
    run_indexer
    programa
    ;;
  -2)
    build_contracts
    programa
    ;;
  -1)
    login
    ;;
  "create-accounts")
    create_accounts
    ;;
  "build")
    build
    ;;
  "run-indexer")
    run_indexer
    ;;
  "deploy")
    deploy
    ;;

  *)
    echo not a command
    programa
    ;;
  esac
}

 if [ -n "$1" ]; then
    programa $1
  else
    programa
  fi
