## Description 
Fiddi is a command line tool that does the boring and complex process of checking and processing/watching transactions on [EVM compatible](https://chainlist.org/) Blockchain.

### Installation

 - Make sure you have rust installed on your system, adn cargo is set.
  	 ```
	   git clone https://github.com/ahmadbrainworks/fiddi
	   cd fiddi
	   cargo build --release
	   cd target/release
	```
	and execute `fiddi` from there.

 - Install from the release branch.
 	 download a release file from the [release branch](https://github.com/ahmadbrainworks/fiddi/releases/) 
	```
	  tar -xvf fiddi-0.1.linux.tar.gz
      cd fiddi/bin
	```
	  and execute `fiddi` from the folder.
   	

#### Usage
`./fiddi --block-number 123456789 --rpc https://bsc-dataseed.binance.org/ --port 8081 --ip-address 0.0.0.0 --webhook https://webhookaddress.tld/my-enpoint --http --keep-on `

 ##### flags
 
  `--block-number`", type: integer/string.
    specify a block number to start indexing with, and checking for transactions in that block, for latest block use "latest"  , e.g --block-number latest

-  `--rpc`, type: string. 
	Your node RPC URL


- `--port`, type: string.    
    give a port that you want to expose to the internet or your local network, the purpose of this flag is to create an endpoint that you can easily use it for adding addresses to watch for their incoming transactions.

- `--ip-address`, type: string.
	 Ip address for adding a list of addresses to the watchlist.
 

- `--webhook`, type: optional<string>.
	  Your webhook endpoint for notification about incoming transactions. The endpoint must accept post request from the `fiddi`, the request is in JSON format that includes blocknumber, blockhash,sender address, receiver address and recipient address, value, and transaction hash.
	  payload to be sent to your `webhook` address:
	```
	  {"blockHash":"0x1169c9501bb552b35e1f297df3ecf52a9aabca5d1ac4ea5f0700e7d7992091fc","blockNumber":"0x117608b","from":"0x038173cdd584df8037ea0126559cd3e1daba0c35","gas":"0xb7390","gasPrice":"0x12a05f200","hash":"0x24cb59a6e9dc65532b935fd8a3dcc5a54d797a3492d5aedc136f1f71c06290a1","input":"0xfb3bdb41000000000000000000000000000000000000001f8def8808b02452c9a00000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000038173cdd584df8037ea0126559cd3e1daba0c350000000000000000000000000000000000000000000000000000000062975e8d0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c000000000000000000000000d8b329d00acb2d11e45bac7c406230497a65105c","nonce":"0x118e","r":"0x3b2b58b5457ce8dc51dcffd77c151b2b46531603f13b395123f933412e1ad57e","s":"0x2a9ec1320006a94a36277f72a5c2936fe38f7fb676f507f734a00daf369f3410","to":"0x10ed43c718714eb63d5aa57b78b54704e256024e","transactionIndex":"0x53","type":"0x0","v":"0x94","value":"0x13f40c891fccb7"}

	```


	 



- `--http`, type: bool.
	 This flag is boolean, specify it only when you want to use the adding wallet addresses to watchlist feature.
	 `endpoint` for adding wallet address to the watchlist;
	 `https://ip-address:port/api/new/address`
          body: 
        ```
	{"address": "0x096f7a3B544e62729a1FAD5c3882A1C65D1a9f72"}
	```


 
 - `--keep-on`, type: bool.
	   When this flag is set, after your given --block-number is done checking, it'll then move to the next block, and so on..
   
 
  
