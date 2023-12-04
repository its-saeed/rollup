## Build
```bash
git clone https://github.com/its-saeed/rollup.git
cd rollup
cargo build
```

## Run
```bash
cargo run -- -d from_da.txt -s from_sequencer.txt
```

### Options
These options exist to pass to the executable as input parameters:
  * To specify DA file path:  -d, --da-file <DA_FILE> 
  * To specify SEQ file path: -s, --sequencer-file <SEQ_FILE>  
  * To enable state persist: -p, --persist         
  * To load state from db: -l, --load-state                 
  * -h, --help                       Print help
  * -V, --version                    Print version

### Use mongo
If you want to use mongodb as the key-value database, you need to run it before running the app. Use [`run_mongo.sh`](./run_mongo.sh) to run a dockerized version.

```bash
cargo run -- -d from_da.txt -s from_sequencer.txt --load-state --persist
```


### Output
* Sequencer lies are printed in red.
* Reorgs are printed in yellow. 
* A summary is printed at the end.
* If a reorg is not possible, an error message is printed

## Nex steps
* Fix/update misunderstandings.
* Create a backend endpoint(Maybe a REST) for the node to make runs and tests and queries more simpler.