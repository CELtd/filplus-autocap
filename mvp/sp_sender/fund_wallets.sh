#!/bin/bash

# Set the sender address
FROM_ADDRESS="f3uzg3iiwlq2izgksruktnmtsin2zkogc2agl7y4r6yecy636ecg7uucffwftzwemlgkr3ezjyqa3gpwb64bda"
AMOUNT="5"  # Amount in FIL

# List of recipient addresses
ADDRESSES=(
  f12cifrxvz5vmt6beromytwze5wqavurpb5zx7y3y
  f12nxww2fdsub4ohmcs6qa4juahy4ecmlqaz5mazi
  f135ws7c3wokebsk4p4ycynwanqvrdwgco3puaoky
  f1bqz2oxlb7sw23ydutbs6eum4x54xqot26upb7uq
  f1hzkkrqdg3h2fiwsegfusu543fasv6mc4y624iza
  f1ixvovumsfzdwkumced26rfdcklmhlsr52rrsvjy
  f1lacvxrdqgttmynlltdi23mew3rqtvsp5uuwnmjq
  f1m57me3okb6b5la3lqccclwdnamukxkbaemgax6q
  f1oftfklzvnwzjyjp5qrrz5pbudpgmlmslzgi4yty
  f1onvtgvcn6blnn4wtd7w3vdwl4fk7uvkcccadc3q
  f1sj5c63tp5m64kg37bloierr3ehlxemgcmxc65va
  f1vtz5f25rdbmsmlnyzy7cva43zv5bxfrcot3lcki
)

echo "⛽ Sending $AMOUNT FIL from $FROM_ADDRESS to each recipient..."

for addr in "${ADDRESSES[@]}"; do
  echo "➡️ Sending to $addr..."
  echo "~/lotus-local-net/lotus send --from "$FROM_ADDRESS" "$addr" "$AMOUNT""
  ~/lotus-local-net/lotus send --from "$FROM_ADDRESS" "$addr" "$AMOUNT"
done

echo "✅ All transactions submitted."
