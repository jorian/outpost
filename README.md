# Outpost

Watches reserves as they change per block.

![](screenshot.png)

You have the choice to watch all reserve currencies and their reserves, based on a selection of coins.

# Future versions

-   [x] add support for mempool scanning
-   [ ] add color to messages
-   [ ] update price information 
-   [ ] support other PBaaS chains
-   [ ] select specific currency baskets by their name
-   [ ] 24h change %
-   [ ] add settings menu
    - apply filter to log yes/no
    - show log messages based on number of confirmations

# DEV


Run in your terminal of choice with `RUST_LOG=info LC_ALL=en_US.UTF-8 cargo run --color always >> output.log 2>&1`.  
Any development output can be tracked in a new terminal window and `tail -f output.log`.


## ZMQ

This app depends on ZMQ to be configured. Add these in your `vrsctest.conf`:
`zmqpubhashtx=tcp://127.0.0.1:27779`
`zmqpubhashblock=tcp://127.0.0.1:27780`
