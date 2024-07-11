# Explorer

{% embed url="https://explorer.alignedlayer.com" %}

The Explorer keeps track of [Aligned Service Manager](./3_service_manager_contract.md).

It has an internal state of previous batches, actively listens for new batches and their responses. The Explorer then displays this information for Users to visualize the submitted batches, their states and more useful information in real time.

In the landing page we can see information such as how many [Operators](./4_operator.md) are currently registered and running, how many Batches and how many total Proofs have been verified.

![](../../images/explorer-landing-page.png)

From here, we can search for a specific batch by its Merkle Root, we can directly jump to any one of the last 5 submitted batches, and we can easily go to the `Latest Batches` page, where we can navigate through the various pages of batches of proofs submitted to aligned, ordered by latest submission, and easily check their on-chain status, timestamp, and block number.

![](../../images/explorer-latest-batches.png)

From there we can also click any individual batch Merkle Root to view its details.

From here we can visualize:

- the whole `Merkle Root`
- `Amount of Proofs` in the batch
- Ethereum's `Submission Block Number`, linked to etherscan
- `Submission Transaction Hash`, linked to etherscan
- `Submission Timestamp` of the batch
- `Status`, either `Pending` or `Verified`
- Ethereum's `Response Block Number`, linked to etherscan
- `Response Transaction Hash`, linked to etherscan
- `Response Timestamp` of the batch

![](../../images/explorer-batch-details.png)

