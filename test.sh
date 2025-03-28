extract_txid() {
  echo "$@" | grep -oP "txId: '\K[0-9a-h]+(?=')"
}
OUTPUT="$(cat ./test.txt)"
extract_txid $OUTPUT
