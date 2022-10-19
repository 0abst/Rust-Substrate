# Getting familiar with Rust / Substrate

In order to get more familiar with Rust, the following tasks have been done:

  * Rustlings exercises (https://github.com/rust-lang/rustlings):
  
    * Most of the exercises from Rustlings have been completed

  * Udemy course:
  
    * Learn Rust by Building Real Applications (https://www.udemy.com/course/rust-fundamentals/)
    
  * Studying the book:
  
    * https://doc.rust-lang.org/book/
    
    * The implementation of the Multithreaded Web Server (chapter 20) can be found in the repository
    
Then, to be more familiar with Substrate, tutorials on Substrate have been followed (https://docs.substrate.io/tutorials/). More in particular,

  * Build a local blockchain (https://docs.substrate.io/tutorials/get-started/build-local-blockchain/)
  
  	* A screenshot can be found in the repository to show the node is up
    
  * Add a pallet to the runtime (https://docs.substrate.io/tutorials/work-with-pallets/add-a-pallet/)

    * Nicks pallet: Allows blockchain users to pay a deposit to reserve a nickname for an account they control

    * As shown in the tutorial, to add the Nicks pallet, the files Cargo.toml and src/lib.rs of the Runtime folder shall be modified. These can be found in the repository
     
  * Develop smart contracts (https://docs.substrate.io/tutorials/smart-contracts/)
  
      * Several smart contracts have been compiled and then deployed. Interactions with the Gui also took place. The contracts are:
    
      	* Flipper (my_contract)
      	
      	* Increment
      	
      	* ERC20
      	
Several .contract files can be found in the repository as well as a screenshot of the interaction with the UI can be found in the folder. The .contract file combines the Wasm and metadata into one file and can be used for instantiation. In order to build contracts, the use of the Docker image provided by Parity was needed (https://github.com/paritytech/cargo-contract)

  * Exercises on exercism (https://exercism.org/) have been done. 
  
      * The scripts done can be found in the folder *Exercises exercism*

  * Tests:

      * Unit test
    
      * Debug
     
      * Benchmark
      
      	  * Use of the Criterion library

Smart contracts testing with the ink playground has also been done (https://ink.substrate.io/). For instance:

  * ERC20
    
  * ERC721




  
  
