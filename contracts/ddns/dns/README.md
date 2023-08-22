# DNS

DNS program enable to register your program and its interface as a DNS record.
The program stores program ids and meta info of their interfaces (name, description and link).

## Register

To register your dapp you have to send message to DNS program with the following payload:

```json
{
  "Register": "you program id"
}
```

Or you can use `idea.gear-tech.io` to do it.

## Get dns records

You can read the state of the Dns program to get all records or filtered ones (by name, id, pattern).
