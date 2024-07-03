# Aligned Service Manager

The Aligned Service Manager is the Entity in charge of handling the reception of new batches to Aligned, their status, and their response.

It is a smart contract which recieves all new batches, with their merkle root and a pointer to where is the batch currently stored. When received, this manager will emit an Event for the [Operators](./operator.md) to know when there is a new batch to verify.

Then, when recieving a response from the [Aggregator](./aggregator.md), with Operator's aggregated BLS signatures, the Aligned Service Manager check the BLS signature to verify the Operators were in fact those who processed the batch and responded its status.

Once verified, it will emit another Event, for anyone interested (for example, the [Explorer](./explorer.md)) to know that the batch was verified by the operators. This batch is now verified and Users can know their proofs inside the batch were proven and verified leveraging Ethereum's security. 

More low-level details on how this Entity works on our [Smart Contracts](../../architecture/3_smart_contracts.md) section.
