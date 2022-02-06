# ledger-lint
`ledger-lint` is a script I created to clean up my [ledger-cli](https://www.ledger-cli.org/) (repo [here](https://github.com/ledger/ledger)) files and to start working with rust.

## Current Features
### Declarations First
Complains when account declarations appear after the first transaction.

### Transaction Order
Complains when a transaction appears out of order. Expects transaction dates to ascend.

### Force Payee Info
Complains when an `Income` or `Expense` posting has a) no payee declaration for the entire transaction and b) no payee declaration for this posting.

# Building
`cargo build`