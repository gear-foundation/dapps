# Decentralized Internet

Decentralized Internet demonstrates on-chain server-less approach to web sites and web applications hosting. Unlike server-based DNS built on centralized components and services, decentralized solutions running on the blockchain are characterized by boosted data security, enhanced data reconciliation, minimized system weak points, optimized resource allocation, and demonstrated great fault tolerance. It brings all the benefits of decentralization such as censorship resistance, security resilience, high transparency.

Briefly the solution consists of DNS smart contract that is uploaded on-chain. It lists programs (smart-contracts) that are also uploaded on-chain and registered in DNS contract. Hosted programs may have the user interface that resides on IPFS.

## Connect your dApp to the Decentralized Internet

To connect your program to the Decentralized Internet on Gear Network it's necessary to have a variable of type `Option<DnsMeta>` in your program that will contain metadata of the DNS record.

```rust
pub struct DnsMeta {
    pub name: String,
    pub link: String,
    pub description: String,
}
```

One more thing that you need to do is to include the following enum variants:

1. in handle_input type

- `GetDnsMeta` - it has to be the first variant of the enum
- `SetDnsMeta(DnsMeta)` - it is needed to setup the dns record

2. in handle_output type

- `DnsMeta(Option<DnsMeta>)` - it also has to be the first variant of the enum

After your program has been uploaded on chain you need to build your frontend to a single html file and upload it to IPFS.

1. Download and install IPFS Desktop - https://github.com/ipfs/ipfs-desktop
2. Upload your built web app using 'Files' tab
3. Get file link by pressing option dots button on file and choose 'Share link'

The next step is to send Metadata to your program using the `SetDnsMeta` enum variant. Where you need to set name, link (that is link to html file on IPFS) and description.

To register your dApp in DNS, you need to send a message to the DNS program. You can do it through https://idea.gear-tech.io/ find DNS program and send message `Register` with the id of your program.

## Open and use dApp

Firstly you need to download the dns.html file from releases and open it in your browser.
If you have your dApp registered in DNS program you will see it in the list of available dApps. Just click the "Open" button and your interface will be opened in the new tab.
