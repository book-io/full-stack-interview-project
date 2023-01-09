# Book.io Full Stack Engineer Interview Project

Thank you for your interest in working at Book.io. We hope that you find this interview project fun and reasonable and wish you the best of luck.

## Instructions

We need to create a simple Rust program to list the books available for a given stake address. We also need to be able to tell if a book exists at any address associated with the stake address.

1. Fork this repository.
1. Create a branch off the `main` branch.
1. Create a free account at https://www.tangocrypto.com/.
1. Copy `.env.dist` to `.env`.
1. Create a testnet app and copy your `APP_ID` and `API_KEY` into `.env`.
1. Implement the `CardanoApi` trait for `TangoClient` in [src/cardano/tango/client.rs](src/cardano/tango/client.rs).
1. Implement the `Bookshelf` functionality in [src/model/bookshelf.rs](src/model/bookshelf.rs).

**IMPORTANT**: Please do not modify any code in [main.rs](src/main.rs). You can add dependencies to [Cargo.toml](Cargo.toml), add structs and functions in the [cardano](src/cardano) module, [tango](src/cardano/tango) module, and can add (but not remove) functions to the [CardanoApi](src/cardano/api.rs) trait if you wish.

You can test your program using the following stake address: `stake_test1uqz3rd4valdhl0mypqsn7dr3gq52qu6v3tlhapvjzlcr6nsw7h74w`.

## Deliverable

Once you finish the project, please submit a pull request to the `interview` branch. If you submit the PR to the wrong branch, your application will not be considered as we put a lot of value on being able to follow instructions. Once your pull request is submitted, please copy the following questions into an email, answer them, and send it to careers at book dot io (replacing the "at" and "dot" appropriately to make a valid email address).

```
1. What is your favorite thing about Cardano?
2. What is your least favorite thing about Cardano?
3. What is your favorite thing to do outside of coding?
4. What is the link to your interview project pull request?
```

## Follow up

After you have submitted the completed project (and assuming it compiles and works correctly) we will follow up to schedule a technical interview. This interview will be roughly 1 hour in length and we will spend a considerable amount of time discussing the interview project. Please be prepared to explain every line of code you wrote and the reason for implementing things the way you did. We are less interested in the code and more interested in the way you think and solve problems. The remainder of the technical interview will cover various parts of a standard web stack.


Good luck to all who apply and have a great day.