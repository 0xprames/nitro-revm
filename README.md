# nitro-revm
PoC of Revm execution in a Nitro Enclave

Inspired by [sgx-revm](https://github.com/gakonst/sgx-revm/tree/master) which was completed by Georgios Konstantopoulos and Andrew Miller

## Motivation

SGX enclaves have historically had serious vulnerabilites, allowing for escalation-of-privilege attacks where malicious actors obtained the ability to extract confidential information gated by the software guard extension. [1](https://arstechnica.com/information-technology/2022/08/architectural-bug-in-some-intel-cpus-is-more-bad-news-for-sgx-users/) 

These vulnerabilities have since been patched, and generally are difficult to take advantage of in a cloud hosted environment - The security posture at large public cloud providers is relatively strong (speaking anecdotally from my own previous experiences building multitenant confidential compute systems in the AWS containers/linux organization).

Despite the above, I wanted to see how we can build the same revm execution simulation in an more secure environment.

## Enter AWS Nitro Enclaves

Nitro Enclaves are Amazon's homegrown confidential computing paradigm, based off their Nitro hypervisor technology [2](https://aws.amazon.com/ec2/nitro/nitro-enclaves/)

Nitro Enclaves run in their own virtual machine on the Host VM, effectively providing a (virtualized) kernel isolation boundary. This is a much stronger security boundary than SGX provides, which is by default protected against the previously seen SGX attack vector.  

Slide 44 - 48 in [3](https://cseweb.ucsd.edu/~yiying/cse291-fall22/reading/Nitro.pdf) show why Nitro is more secure than Azure Enclaves/SGX.

## Reproduction using this codebase

1) launch an Amazon Linux 2 M5.xlarge EC2 instance with Nitro Enclaves enabled, make sure to save the private ssh key used while creating this instance. Note: make sure to use AL2 and not Al2022/2023.
2) connect to the instance, and clone this codebase down
3) install `nitro-cli` [4](https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave-cli-install.html)
4) in one ssh terminal:
```
cd <path-to-where-you-cloned-this-codebase>
make server
nitro-cli run-enclave --eif-path nitro-revm-server.eif --cpu-count 2 --memory 256 --debug-mode
# the above will return an enclave CID/ID - make sure to note these down
nitro-cli console --enclave-id $ENCLAVE_ID_FROM_ABOVE_STEP 
```
5) in another ssh terminal to the same instance:
```
cd <path-to-where-you-cloned-this-codebase>
./revm_driver client --cid $ENCLAVE_CID_FROM_ABOVE_STEP --port 7878
```

6) You should see in the console terminal:
```
]
recieved: Payload {
    sender: 0xdafea492d9c6733ae3d56b7ed1adb60692c98bc5,
    amount: 0x000000000000000000000000000000000000000000000000000000000000002a_U256,
}
[nitro-revm/src/main.rs:36] &payload = Payload {
    sender: 0xdafea492d9c6733ae3d56b7ed1adb60692c98bc5,
    amount: 0x000000000000000000000000000000000000000000000000000000000000002a_U256,
}
[nitro-revm/src/main.rs:76] &result = ResultAndState {
    result: Success {
        reason: Stop,
        gas_used: 21000,
        gas_refunded: 0,
        logs: [],
        output: Call(
            b"",
        ),
    },
    state: {
        0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97: Account {
            info: AccountInfo {
                balance: 0x0000000000000000000000000000000000000000000000000000000000000045_U256,
                nonce: 1,
                code_hash: 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
                code: Some(
                    Bytecode {
                        bytecode: "00",
                        state: Analysed {
                            len: 0,
                            jump_map: JumpMap {
                                map: "00",
                            },
                        },
                    },
                ),
            },
            storage: {},
            status: AccountStatus(
                Touched,
            ),
        },
        0x0000000000000000000000000000000000000000: Account {
            info: AccountInfo {
                balance: 0x0_U256,
                nonce: 0,
                code_hash: 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
                code: Some(
                    Bytecode {
                        bytecode: "00",
                        state: Analysed {
                            len: 0,
                            jump_map: JumpMap {
                                map: "00",
                            },
                        },
                    },
                ),
            },
            storage: {},
            status: AccountStatus(
                Touched | LoadedAsNotExisting,
            ),
        },
        0xdafea492d9c6733ae3d56b7ed1adb60692c98bc5: Account {
            info: AccountInfo {
                balance: 0x000000000000000000000000000000000000000000000000000000000000002a_U256,
                nonce: 0,
                code_hash: 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470,
                code: Some(
                    Bytecode {
                        bytecode: "00",
                        state: Analysed {
                            len: 0,
                            jump_map: JumpMap {
                                map: "00",
                            },
                        },
                    },
                ),
            },
            storage: {},
            status: AccountStatus(
                Touched | LoadedAsNotExisting,
            ),
        },
    },
}
```
Demonstrating the revm simulation from `sgx-revm` executing in a secure nitro enclave. 

I.e applications on the host machine have no grasp of the revm simulation being executed.

## Arch Diagram
TODO



