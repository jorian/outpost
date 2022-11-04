# Outpost

Watches reserves as they change per block.

![](screenshot.png)

You have the choice to watch all reserve currencies and their reserves, based on a selection of coins.

# Future versions

- add support for mempool scanning
- change PBaaS chains
- select specific currency baskets by their name
- 24h change %

# DEV

Updates to the overview are triggered by new blocks using zmq. Put `zmqpubhashblock=tcp://127.0.0.1:27780` in your VRSCTEST.conf.
Run with `RUST_LOG=info LC_ALL=en_US.UTF-8 cargo run --color always >> output.log 2>&1`

