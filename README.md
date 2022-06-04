## Description 
Fiddi is a command line tool that does the boring and complex process of checking and processing/watching transactions on [EVM compatible](https://chainlist.org/) Blockchain.

#### Usage
`./fiddi --block-number 123456789 --rpc https://bsc-dataseed.binance.org/ --port 8081 --ip-address 0.0.0.0 --webhook https://webhookaddress.tld/my-enpoint --http --keep-on `

 ##### flags
 
  "`--block-number`", type: integer/string.
    specify a block number to start indexing with, and checking for transactions in that block, for latest block use latest  , e.g --block-number latest

-  "`--rpc`", type: string. 
	Your node RPC URL


- "`--port`", type: string.    
    give a port that you want to expose to the internet or your local network, the purpose of this flag is to create an endpoint that you can easily use it for adding addresses to watch for their incoming transactions.

- "`--ip-address`", type: string.
	 Ip address for adding a list of addresses to the watchlist.
 

- "`--webhook`", type: string.
	  Your webhook endpoint for notification about incoming transactions. The endpoint must accept post request from the `fiddi`, the request is in JSON format that includes blocknumber, blockhash,sender address, receiver address and recipient address, value, and transaction hash.


- "`--http`", type: bool.
	 This flag is boolean, specify it only when you want to use the adding wallet addresses to watchlist feature.


 
 - "`--keep-on`", type: bool.
	   When this flag is set, after your given --block-number is done checking, it'll then move to the next block, and so on..
   
 
  