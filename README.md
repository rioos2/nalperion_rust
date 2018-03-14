# nalperion_rust

The document provides information on *Intergration to Nalperion for licensing using Rust Lang*

This is reference code to help you operate with nalperion, but the way you use it in your software to make the entitlement work needs to be solutioned by you.

# Rust Lang

Tested on [rust 1.24.1](https://rust-lang.org) 

- [Rio/OS Aventura](http://rio.digital)
- Any linux

# Reference

- [Nalperion integration samples (C/C++), Java](https://naldoc.atlassian.net/wiki/spaces/NND/pages/426012/Application+Integration)
- [Nalperion fat client API](https://naldoc.atlassian.net/wiki/spaces/NND/pages/426049/Developers+API+Latest)

# nalperion.rs 

This code assumes that you have `ShaferFileChk.so` downloaded and extracted, available in the `tools/license` directory

# create_trial_or_verify

This method does 

- [ ] NalpLibOpen by using the .so file provided
- [ ] NSLValidateLibrary using CUSTOMER_ID, PRODUCT_D
- [ ] NSLGetLicense 
- [ ] NalpLibClose

The output of the method is

- On error, the error msg has the information
- On success, trial commenced or activated using the license code.

If you have a license code from Nalperion then edit [config.rs](https://github.com/rioadvancement/nalperion_rust/blob/master/src/config.rs)

```

activation_code: Some("09090000090909".to_string()),


```

# Compile and Run

```

cargo build

cd ./target/debug/

./rioos-entitlement


```

Voila !
