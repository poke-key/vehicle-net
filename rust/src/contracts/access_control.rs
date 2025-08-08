pub use access_control::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod access_control {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_dataMarketplace"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vehicleRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("accessSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accessSessions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("endTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isActive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataRequests"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cleanupExpiredSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cleanupExpiredSessions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKeys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createAccessSession"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createAccessSession",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dataMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dataMarketplace"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract DataMarketplace"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("extendSession"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("extendSession"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAccessSession"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAccessSession"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccessControl.AccessSession",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProductSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProductSessions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSessionRequests"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSessionRequests"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AccessControl.DataRequest[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserActiveSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserActiveSessions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("activeSessions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserSessions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxConcurrentSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxConcurrentSessions",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("productSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("productSessions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestDataAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestDataAccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sessionRequests"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sessionRequests"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wasAuthorized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sessionTimeoutBuffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "sessionTimeoutBuffer",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMaxConcurrentSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setMaxConcurrentSessions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSessionTimeoutBuffer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setSessionTimeoutBuffer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBuffer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("terminateSession"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("terminateSession"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("userSessions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateAccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isValid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timeRemaining"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vehicleRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vehicleRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract VehicleRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessSessionCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessSessionCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("endTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AccessSessionTerminated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccessSessionTerminated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DataAccessRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DataAccessRequested",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sessionKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("requestType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SessionLimitUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SessionLimitUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSessionKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSessionKey"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReentrancyGuardReentrantCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReentrancyGuardReentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SessionExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SessionExpired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SessionNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SessionNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SessionNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SessionNotFound"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TooManyConcurrentSessions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TooManyConcurrentSessions",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnauthorizedAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnauthorizedAccess"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ACCESSCONTROL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC04a\x01\x04W`\x1Fa\x19\x028\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x08W\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x01\x04Wa\0R` a\0K\x83a\x01\x1CV[\x92\x01a\x01\x1CV[`\x01_U3\x15a\0\xF1W`\x01\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U`@Q\x93\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3`\x03`\x06Ua\x01,`\x07U_`\x08U`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x16`\xA0Ra\x17\xD1\x90\x81a\x011\x829`\x80Q\x81\x81\x81a\x03\x7F\x01R\x81\x81a\x07%\x01Ra\x0E\xDB\x01R`\xA0Q\x81a\x04M\x01R\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\x04WV\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\n\xEE\xAA\xD7\x14a\x11}WP\x80c\x11H\0\xBC\x14a\x10\xD2W\x80c\x13\xA5\xA3\"\x14a\x0E\x8EW\x80c\x16o\xF8\n\x14a\r\x8AW\x80c\"7\x15\xDD\x14a\r\x18W\x80c)yu\x12\x14a\x0C\x85W\x80c1\xC3\xA6\x12\x14a\x0CaW\x80c7\xCB\x94\xAA\x14a\t7W\x80c;R%\xFF\x14a\x08\xC3W\x80c?\xB5\x8Ft\x14a\x08\x8FW\x80cR\x90\xCF\x96\x14a\x07TW\x80ci\x9C\xB1V\x14a\x07\x10W\x80cmY}\x13\x14a\x06\xC7W\x80cqP\x18\xA6\x14a\x06lW\x80cs\xC1\xF4\xA0\x14a\x05\xF4W\x80c\x86#)&\x14a\x04\xA4W\x80c\x8D\xA5\xCB[\x14a\x04|W\x80c\x9DV\xF0\x85\x14a\x048W\x80c\x9Dlb\xCF\x14a\x02~W\x80c\xD3x\x1F\x80\x14a\x02aW\x80c\xD9w\\?\x14a\x02\x1BW\x80c\xDF\xD3#\x06\x14a\x01\xFAW\x80c\xE3;\xB6\xEA\x14a\x01\xAEWc\xF2\xFD\xE3\x8B\x14a\x01$W_\x80\xFD[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\x01=a\x12\x84V[a\x01Ea\x17tV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01\x97W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW\x7F\x91B\x8E~&\xAAE\\wE\xC9\xD6\xA3\x8E<\x0F'!'\xED\xB7\xC5\x98\x80\xD6\xAE\xA7\xAF\x0C\x8C%_` `\x045a\x01\xEDa\x17tV[\x80`\x06U`@Q\x90\x81R\xA1\0[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\x02\x13a\x17tV[`\x045`\x07U\0[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`$5`\x045_R`\x04` R`@_ \x80T\x82\x10\x15a\x01\xAAW` \x91a\x02R\x91a\x14\x14V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW` `\x07T`@Q\x90\x81R\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x80_R`\x02` R`@_ `@Q\x90a\x02\xC7\x82a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x81T\x16\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T\x91``\x81\x01\x92\x83R`\xC0`\x06`\xFF`\x04\x85\x01T\x16\x15\x93\x84\x15`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01Ra\x04\x1AWQ`\x07Ta\x03)\x91a\x16\xBCV[B\x11a\x04\x0BW_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\xBDW`\x01\x81\x01T`@\x80Qc\x10\xA9\xF1\xF7`\xE1\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x91\x90\x82`D\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\0W_\x90_\x93a\x03\xCCW[P\x15a\x03\xBDW`\x03\x01U\0[c\x1A'\xEA\xC3`\xE1\x1B_R`\x04_\xFD[\x90Pa\x03\xF1\x91\x92P`@=`@\x11a\x03\xF9W[a\x03\xE9\x81\x83a\x11\xCFV[\x81\x01\x90a\x14)V[\x91\x90\x83a\x03\xB1V[P=a\x03\xDFV[`@Q=_\x82>=\x90\xFD[c\x0F\xE8-%`\xE1\x1B_R`\x04_\xFD[c'@m\xB9`\xE0\x1B_R`\x04_\xFD[c\x96\xC9_\x81`\xE0\x1B_R`\x04_\xFD[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x05` R`@_ \x80T\x90a\x04\xCF\x82a\x13\xFCV[\x91a\x04\xDD`@Q\x93\x84a\x11\xCFV[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x05\x96W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x92\x91_\x90[\x82\x82\x10a\x05-WPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` \x80\x91`?\x19\x89\x82\x03\x01\x85R\x87Q\x90\x84\x80`\xA0\x1B\x03\x82Q\x16\x81R\x82\x82\x01Q\x83\x82\x01R`@\x82\x01Q`@\x82\x01R`\x80\x80a\x05~``\x85\x01Q`\xA0``\x86\x01R`\xA0\x85\x01\x90a\x13\xD8V[\x93\x01Q\x15\x15\x91\x01R\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x05\x1EV[`\x05` `\x01\x92`@Qa\x05\xA9\x81a\x11\xB3V[\x84\x80`\xA0\x1B\x03\x86T\x16\x81R\x84\x86\x01T\x83\x82\x01R`\x02\x86\x01T`@\x82\x01Ra\x05\xD2`\x03\x87\x01a\x138V[``\x82\x01R`\xFF`\x04\x87\x01T\x16\x15\x15`\x80\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x04\xEFV[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x02` R`\xE0`@_ `\x01\x80`\xA0\x1B\x03\x81T\x16\x90`\x01\x81\x01T\x90`\x02\x81\x01T`\x03\x82\x01T`\xFF`\x04\x84\x01T\x16\x91`\x06`\x05\x85\x01T\x94\x01T\x94`@Q\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R\x15\x15`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAWa\x06\x84a\x17tV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAWa\x06\xE0a\x12\x84V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 \x80T`$5\x91\x90\x82\x10\x15a\x01\xAAW` \x91a\x02R\x91a\x14\x14V[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xAAW6`#\x82\x01\x12\x15a\x01\xAAW\x80`\x04\x015\x90a\x07\x91\x82a\x13\xFCV[\x91a\x07\x9F`@Q\x93\x84a\x11\xCFV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\xAAW`$\x01\x90[\x82\x82\x10a\x08\x7FW\x83_[\x81Q\x81\x10\x15a\x08}W\x80a\x07\xDC`\x01\x92\x84a\x14\x9FV[Q_R`\x02` R`@_ `\x04\x81\x01\x90\x81T\x90`\xFF\x82\x16\x90\x81a\x08cW[Pa\x08\tW[PP\x01a\x07\xC6V[`\xFF\x19\x16\x90Ua\x08\x19\x81\x84a\x14\x9FV[Q\x7F\x94\xD9l\x98\xBA\x93\x7F\x8E\xE9\xA0\x84\x02\xBA\xD1M\xF3\xF3%!\xF1\xC1\xA9A'\x06P;;\xA5\xA64 ```@Q` \x81R`\x07` \x82\x01Rf\x11^\x1C\x1A\\\x99Y`\xCA\x1B`@\x82\x01R\xA2\x83\x80a\x08\x01V[a\x08u\x91P`\x03\x01T`\x07T\x90a\x16\xBCV[B\x11\x86a\x07\xFBV[\0[\x815\x81R` \x91\x82\x01\x91\x01a\x07\xBCV[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`@a\x08\xB5a\x08\xADa\x12nV[`\x045a\x16\xC9V[\x82Q\x91\x15\x15\x82R` \x82\x01R\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x04` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\t!Wa\t\x1D\x85a\t\x11\x81\x87\x03\x82a\x11\xCFV[`@Q\x91\x82\x91\x82a\x12\x9AV[\x03\x90\xF3[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x08\xFAV[4a\x01\xAAWa\tE6a\x11\xF1V[_\x82\x81R`\x02` R`@\x90 T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x80_R`\x02` R`@_ `@Q\x90a\t}\x82a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x81T\x16\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T\x91``\x81\x01\x92\x83R`\xC0`\x06`\xFF`\x04\x85\x01T\x16\x15\x93\x84\x15`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01Ra\x04\x1AWQ`\x07Ta\t\xDF\x91a\x16\xBCV[B\x11a\x04\x0BW_\x81\x81R`\x02` R`@\x90 \x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\xBDW`\x01`\x03\x83\x01TB\x11\x15\x92\x83a\x0CLW[\x01\x92\x83T\x93`@Qa\n(\x81a\x11\xB3V[3\x81R` \x81\x01\x95\x86R`@\x81\x01\x95B\x87R``\x82\x01\x96\x84\x88R`\x80\x83\x01\x91\x87\x83R\x86_R`\x05` R`@_ \x80T`\x01`@\x1B\x81\x10\x15a\x0C%Wa\ns\x91`\x01\x82\x01\x81Ua\x12\xD3V[\x94\x90\x94a\x0C9WQ\x84T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x84UQ`\x01\x84\x01UQ`\x02\x83\x01U\x95Q\x80Q\x90\x96\x90`\x03\x83\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C%Wa\n\xC7\x82Ta\x13\0V[\x98`\x1F\x8A\x11a\x0B\xE0W[` \x99P\x89\x90`\x1F\x83\x11`\x01\x14a\x0BvW\x91\x80`\x04\x94\x92a\x0B\x1F\x97\x96\x94_\x92a\x0BkW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x15\x15\x91\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[T\x91\x7Fc\xD7-h\xDA\xD1\x02\xF2\xD2\xBF\x9DZ\xAD\xEAQ(\xA1e\xCA\x8A\x8Em\x97w\x80\xD7_\xFA\xE0\xFE\xEB\xEDa\x0BW`@Q\x93`@\x85R`@\x85\x01\x90a\x13\xD8V[\x92\x85\x87\x82\x01R\x803\x94\x03\x90\xA4`@Q\x90\x81R\xF3[\x01Q\x90P\x8C\x80a\n\xF5V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x0B\xC9WP\x92`\x01\x92\x85\x92a\x0B\x1F\x99\x98\x96`\x04\x98\x96\x10a\x0B\xB1W[PPP\x81\x1B\x01\x90Ua\x0B\tV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8C\x80\x80a\x0B\xA4V[\x92\x93\x8D`\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x0B\x86V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x9A` \x84\x10a\x0C\x1BW[`\x1F\x01`\x05\x1C\x01\x99[\x8A\x81\x10a\x0C\x10WPa\n\xD1V[_\x81U`\x01\x01a\x0C\x03V[\x90\x9AP\x8A\x90a\x0B\xFAV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`\x05\x81\x01a\x0CZ\x81Ta\x14IV[\x90Ua\n\x17V[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\t\x1Da\t\x11a\x0C\x80a\x12\x84V[a\x14\xB3V[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`$5`\x045_R`\x05` R`@_ \x90\x81T\x81\x10\x15a\x01\xAAWa\x0C\xBA\x91a\x12\xD3V[P`\x01\x80`\xA0\x1B\x03\x81T\x16`\x01\x82\x01T\x91a\r\x0C`\x02\x82\x01T\x91`\xFF`\x04a\x0C\xE4`\x03\x84\x01a\x138V[\x92\x01T\x16\x92`@Q\x95\x86\x95\x86R` \x86\x01R`@\x85\x01R`\xA0``\x85\x01R`\xA0\x84\x01\x90a\x13\xD8V[\x90\x15\x15`\x80\x83\x01R\x03\x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x01`\x01`\xA0\x1B\x03a\r9a\x12\x84V[\x16_R`\x03` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\rtWa\t\x1D\x85a\t\x11\x81\x87\x03\x82a\x11\xCFV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\r]V[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_`\xC0`@Qa\r\xAD\x81a\x11\x97V[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R\x80_R`\x02` R`\x01\x80`\xA0\x1B\x03`@_ T\x16\x15a\x04)W_R`\x02` R`\xE0`@_ `@Qa\x0E\x03\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x91\x82\x82R`\x01\x81\x01T` \x83\x01\x90\x81R`\x02\x82\x01T`@\x84\x01\x90\x81R`\x03\x83\x01T\x90``\x85\x01\x91\x82R`\xFF`\x04\x85\x01T\x16\x92`\x80\x86\x01\x93\x15\x15\x84R`\xC0`\x06`\x05\x87\x01T\x96`\xA0\x89\x01\x97\x88R\x01T\x96\x01\x95\x86R`@Q\x96\x87RQ` \x87\x01RQ`@\x86\x01RQ``\x85\x01RQ\x15\x15`\x80\x84\x01RQ`\xA0\x83\x01RQ`\xC0\x82\x01R\xF3[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`\x045a\x0E\xAAa\x12nV[`@\x80Qc\x10\xA9\xF1\xF7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x92\x90\x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x04\0W_\x90_\x93a\x10\xAEW[P\x15a\x03\xBDWa\x0F$\x83a\x14\xB3V[Q`\x06T\x11\x15a\x10\x9FW` \x92\x83\x92\x7F#I\xA0\xFA\t\xEDS\xCB\xDF\xF8\x87\xA1\xCE\xD6\x94W\"R\xD3CZ\xDFl\xE5N3\x0FPa\xC8g\x9Fa\x0F_`\x08Ta\x14IV[\x80`\x08U`@Q\x86\x81\x01\x91`\x01\x80`\xA0\x1B\x03\x86\x16\x95k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83R\x86`4\x83\x01RB`T\x83\x01RD`t\x83\x01R3``\x1B`\x94\x83\x01R`\xA8\x82\x01R`\xA8\x81Ra\x0F\xB8`\xC8\x82a\x11\xCFV[Q\x90 \x94\x85\x92`@Qa\x0F\xCA\x81a\x11\x97V[\x85\x81R`\x06\x83\x82\x01\x88\x81R`@\x83\x01B\x81Ra\x10^``\x85\x01\x86\x81R`\x80\x86\x01\x90`\x01\x82R`\xA0\x87\x01\x93_\x85R`\xC0\x88\x01\x95\x8C\x87R\x8C_R`\x02\x8BR`@_ \x98`\x01\x80`\xA0\x1B\x03\x90`\x01\x80`\xA0\x1B\x03\x90Q\x16\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x8AT\x16\x17\x89UQ`\x01\x89\x01UQ`\x02\x88\x01UQ`\x03\x87\x01UQ\x15\x15`\x04\x86\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[Q`\x05\x84\x01UQ\x91\x01U\x84_R`\x03\x82Ra\x10|\x84`@_ a\x14kV[\x85_R`\x04\x82Ra\x10\x90\x84`@_ a\x14kV[`@Q\x90\x81R\xA4`@Q\x90\x81R\xF3[c9-a\x8B`\xE1\x1B_R`\x04_\xFD[\x90Pa\x10\xCA\x91\x92P`@=`@\x11a\x03\xF9Wa\x03\xE9\x81\x83a\x11\xCFV[\x91\x90\x84a\x0F\x15V[4a\x01\xAAWa\x10\xE06a\x11\xF1V[_\x82\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x81_R`\x02` R`@_ `\x01\x80`\xA0\x1B\x03\x81T\x163\x14\x15\x80a\x11hW[a\x03\xBDW`\x04\x01\x80T`\xFF\x19\x16\x90U`@Q` \x80\x82R\x7F\x94\xD9l\x98\xBA\x93\x7F\x8E\xE9\xA0\x84\x02\xBA\xD1M\xF3\xF3%!\xF1\xC1\xA9A'\x06P;;\xA5\xA64 \x92\x82\x91a\x11c\x91\x90\x83\x01\x90a\x13\xD8V[\x03\x90\xA2\0[P`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x11\x1BV[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW` \x90`\x06T\x81R\xF3[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[\x90`@`\x03\x19\x83\x01\x12a\x01\xAAW`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xAAW\x81`#\x82\x01\x12\x15a\x01\xAAW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C%W`@Q\x92a\x12L`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x11\xCFV[\x82\x84R`$\x83\x83\x01\x01\x11a\x01\xAAW\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xAAWV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xAAWV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12\xBDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12\xB0V[\x80T\x82\x10\x15a\x12\xECW_R`\x05` _ \x91\x02\x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x13.W[` \x83\x10\x14a\x13\x1AWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x13\x0FV[\x90`@Q\x91\x82_\x82T\x92a\x13K\x84a\x13\0V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x13\xB6WP`\x01\x14a\x13rW[Pa\x13p\x92P\x03\x83a\x11\xCFV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x13\x9AWPP\x90` a\x13p\x92\x82\x01\x01_a\x13cV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x13\x81V[\x90P` \x92Pa\x13p\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x13cV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C%W`\x05\x1B` \x01\x90V[\x80T\x82\x10\x15a\x12\xECW_R` _ \x01\x90_\x90V[\x91\x90\x82`@\x91\x03\x12a\x01\xAAW\x81Q\x80\x15\x15\x81\x03a\x01\xAAW` \x90\x92\x01Q\x90V[_\x19\x81\x14a\x14WW`\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x0C%Wa\x14\x88\x91`\x01\x82\x01\x81Ua\x14\x14V[\x81\x92\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90UV[\x80Q\x82\x10\x15a\x12\xECW` \x91`\x05\x1B\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x16_R`\x03` R`@_ \x90`@Q\x80\x83` \x82\x95T\x93\x84\x81R\x01\x90_R` _ \x92_[\x81\x81\x10a\x16\xA3WPPa\x14\xF5\x92P\x03\x83a\x11\xCFV[__[\x83Q\x81\x10\x15a\x15\xA6Wa\x15\x0B\x81\x85a\x14\x9FV[Q_R`\x02` R`@_ `@Qa\x15#\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T\x90\x81``\x82\x01R`\xC0`\x06`\xFF`\x04\x86\x01T\x16\x15\x15\x94\x85`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01R\x81a\x15\x9BW[Pa\x15\x87W[`\x01\x01a\x14\xF8V[\x90a\x15\x93`\x01\x91a\x14IV[\x91\x90Pa\x15\x7FV[\x90PB\x11\x15_a\x15yV[Pa\x15\xB0\x81a\x13\xFCV[\x90a\x15\xBE`@Q\x92\x83a\x11\xCFV[\x80\x82Ra\x15\xCD`\x1F\x19\x91a\x13\xFCV[\x016` \x83\x017_\x92_[\x81Q\x81\x10\x15a\x16\x9CWa\x15\xEB\x81\x83a\x14\x9FV[Q_R`\x02` R`@_ `@Qa\x16\x03\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T\x90\x81``\x82\x01R`\xC0`\x06`\xFF`\x04\x86\x01T\x16\x15\x15\x94\x85`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01R\x81a\x16\x91W[Pa\x16gW[`\x01\x01a\x15\xD8V[\x93a\x16\x89`\x01\x91a\x16x\x87\x85a\x14\x9FV[Qa\x16\x83\x82\x87a\x14\x9FV[Ra\x14IV[\x94\x90Pa\x16_V[\x90PB\x11\x15_a\x16YV[P\x90\x92PPV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a\x14\xE0V[\x91\x90\x82\x01\x80\x92\x11a\x14WWV[\x91\x90\x91_R`\x02` R`@_ `@Q\x92a\x16\xE4\x84a\x11\x97V[\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x86R`\x01\x84\x01T` \x87\x01R`\x02\x84\x01T`@\x87\x01R`\x03\x84\x01T``\x87\x01\x81\x90R`\x04\x85\x01T`\xFF\x16\x15\x80\x15`\x80\x89\x01R`\x05\x86\x01T`\xA0\x89\x01R`\x06\x90\x95\x01T`\xC0\x90\x97\x01\x96\x90\x96R\x91\x16\x14\x80\x15\x91\x90a\x17lW[Pa\x17eW\x81B\x11a\x17eWB\x82\x03\x91\x82\x11a\x14WW`\x01\x91\x90V[_\x91P\x81\x90V[\x90P_a\x17IV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x88WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xC8\xF9\xA2\xD3\x1E\x7FH|q\x81\xF0\xF1p\xEC\xE63\xD9\xE4\xD0:\xD70\xFB[T\x9F\x88\xF1\x824\xEB0dsolcC\0\x08\x1E\x003";
    /// The bytecode of the contract.
    pub static ACCESSCONTROL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\n\xEE\xAA\xD7\x14a\x11}WP\x80c\x11H\0\xBC\x14a\x10\xD2W\x80c\x13\xA5\xA3\"\x14a\x0E\x8EW\x80c\x16o\xF8\n\x14a\r\x8AW\x80c\"7\x15\xDD\x14a\r\x18W\x80c)yu\x12\x14a\x0C\x85W\x80c1\xC3\xA6\x12\x14a\x0CaW\x80c7\xCB\x94\xAA\x14a\t7W\x80c;R%\xFF\x14a\x08\xC3W\x80c?\xB5\x8Ft\x14a\x08\x8FW\x80cR\x90\xCF\x96\x14a\x07TW\x80ci\x9C\xB1V\x14a\x07\x10W\x80cmY}\x13\x14a\x06\xC7W\x80cqP\x18\xA6\x14a\x06lW\x80cs\xC1\xF4\xA0\x14a\x05\xF4W\x80c\x86#)&\x14a\x04\xA4W\x80c\x8D\xA5\xCB[\x14a\x04|W\x80c\x9DV\xF0\x85\x14a\x048W\x80c\x9Dlb\xCF\x14a\x02~W\x80c\xD3x\x1F\x80\x14a\x02aW\x80c\xD9w\\?\x14a\x02\x1BW\x80c\xDF\xD3#\x06\x14a\x01\xFAW\x80c\xE3;\xB6\xEA\x14a\x01\xAEWc\xF2\xFD\xE3\x8B\x14a\x01$W_\x80\xFD[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\x01=a\x12\x84V[a\x01Ea\x17tV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01\x97W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW\x7F\x91B\x8E~&\xAAE\\wE\xC9\xD6\xA3\x8E<\x0F'!'\xED\xB7\xC5\x98\x80\xD6\xAE\xA7\xAF\x0C\x8C%_` `\x045a\x01\xEDa\x17tV[\x80`\x06U`@Q\x90\x81R\xA1\0[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\x02\x13a\x17tV[`\x045`\x07U\0[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`$5`\x045_R`\x04` R`@_ \x80T\x82\x10\x15a\x01\xAAW` \x91a\x02R\x91a\x14\x14V[\x90T\x90`\x03\x1B\x1C`@Q\x90\x81R\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW` `\x07T`@Q\x90\x81R\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x80_R`\x02` R`@_ `@Q\x90a\x02\xC7\x82a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x81T\x16\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T\x91``\x81\x01\x92\x83R`\xC0`\x06`\xFF`\x04\x85\x01T\x16\x15\x93\x84\x15`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01Ra\x04\x1AWQ`\x07Ta\x03)\x91a\x16\xBCV[B\x11a\x04\x0BW_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\xBDW`\x01\x81\x01T`@\x80Qc\x10\xA9\xF1\xF7`\xE1\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x90\x91\x90\x82`D\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x04\0W_\x90_\x93a\x03\xCCW[P\x15a\x03\xBDW`\x03\x01U\0[c\x1A'\xEA\xC3`\xE1\x1B_R`\x04_\xFD[\x90Pa\x03\xF1\x91\x92P`@=`@\x11a\x03\xF9W[a\x03\xE9\x81\x83a\x11\xCFV[\x81\x01\x90a\x14)V[\x91\x90\x83a\x03\xB1V[P=a\x03\xDFV[`@Q=_\x82>=\x90\xFD[c\x0F\xE8-%`\xE1\x1B_R`\x04_\xFD[c'@m\xB9`\xE0\x1B_R`\x04_\xFD[c\x96\xC9_\x81`\xE0\x1B_R`\x04_\xFD[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x05` R`@_ \x80T\x90a\x04\xCF\x82a\x13\xFCV[\x91a\x04\xDD`@Q\x93\x84a\x11\xCFV[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x05\x96W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x90`@\x81`\x05\x1B\x85\x01\x01\x92\x91_\x90[\x82\x82\x10a\x05-WPPPP\x03\x90\xF3[\x91\x93`\x01\x91\x93\x95P` \x80\x91`?\x19\x89\x82\x03\x01\x85R\x87Q\x90\x84\x80`\xA0\x1B\x03\x82Q\x16\x81R\x82\x82\x01Q\x83\x82\x01R`@\x82\x01Q`@\x82\x01R`\x80\x80a\x05~``\x85\x01Q`\xA0``\x86\x01R`\xA0\x85\x01\x90a\x13\xD8V[\x93\x01Q\x15\x15\x91\x01R\x96\x01\x92\x01\x92\x01\x85\x94\x93\x91\x92a\x05\x1EV[`\x05` `\x01\x92`@Qa\x05\xA9\x81a\x11\xB3V[\x84\x80`\xA0\x1B\x03\x86T\x16\x81R\x84\x86\x01T\x83\x82\x01R`\x02\x86\x01T`@\x82\x01Ra\x05\xD2`\x03\x87\x01a\x138V[``\x82\x01R`\xFF`\x04\x87\x01T\x16\x15\x15`\x80\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x04\xEFV[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x02` R`\xE0`@_ `\x01\x80`\xA0\x1B\x03\x81T\x16\x90`\x01\x81\x01T\x90`\x02\x81\x01T`\x03\x82\x01T`\xFF`\x04\x84\x01T\x16\x91`\x06`\x05\x85\x01T\x94\x01T\x94`@Q\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R\x15\x15`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R\xF3[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAWa\x06\x84a\x17tV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAWa\x06\xE0a\x12\x84V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 \x80T`$5\x91\x90\x82\x10\x15a\x01\xAAW` \x91a\x02R\x91a\x14\x14V[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xAAW6`#\x82\x01\x12\x15a\x01\xAAW\x80`\x04\x015\x90a\x07\x91\x82a\x13\xFCV[\x91a\x07\x9F`@Q\x93\x84a\x11\xCFV[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\xAAW`$\x01\x90[\x82\x82\x10a\x08\x7FW\x83_[\x81Q\x81\x10\x15a\x08}W\x80a\x07\xDC`\x01\x92\x84a\x14\x9FV[Q_R`\x02` R`@_ `\x04\x81\x01\x90\x81T\x90`\xFF\x82\x16\x90\x81a\x08cW[Pa\x08\tW[PP\x01a\x07\xC6V[`\xFF\x19\x16\x90Ua\x08\x19\x81\x84a\x14\x9FV[Q\x7F\x94\xD9l\x98\xBA\x93\x7F\x8E\xE9\xA0\x84\x02\xBA\xD1M\xF3\xF3%!\xF1\xC1\xA9A'\x06P;;\xA5\xA64 ```@Q` \x81R`\x07` \x82\x01Rf\x11^\x1C\x1A\\\x99Y`\xCA\x1B`@\x82\x01R\xA2\x83\x80a\x08\x01V[a\x08u\x91P`\x03\x01T`\x07T\x90a\x16\xBCV[B\x11\x86a\x07\xFBV[\0[\x815\x81R` \x91\x82\x01\x91\x01a\x07\xBCV[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`@a\x08\xB5a\x08\xADa\x12nV[`\x045a\x16\xC9V[\x82Q\x91\x15\x15\x82R` \x82\x01R\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_R`\x04` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\t!Wa\t\x1D\x85a\t\x11\x81\x87\x03\x82a\x11\xCFV[`@Q\x91\x82\x91\x82a\x12\x9AV[\x03\x90\xF3[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x08\xFAV[4a\x01\xAAWa\tE6a\x11\xF1V[_\x82\x81R`\x02` R`@\x90 T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x80_R`\x02` R`@_ `@Q\x90a\t}\x82a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x81T\x16\x82R`\x01\x81\x01T` \x83\x01R`\x02\x81\x01T`@\x83\x01R`\x03\x81\x01T\x91``\x81\x01\x92\x83R`\xC0`\x06`\xFF`\x04\x85\x01T\x16\x15\x93\x84\x15`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01Ra\x04\x1AWQ`\x07Ta\t\xDF\x91a\x16\xBCV[B\x11a\x04\x0BW_\x81\x81R`\x02` R`@\x90 \x80T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\xBDW`\x01`\x03\x83\x01TB\x11\x15\x92\x83a\x0CLW[\x01\x92\x83T\x93`@Qa\n(\x81a\x11\xB3V[3\x81R` \x81\x01\x95\x86R`@\x81\x01\x95B\x87R``\x82\x01\x96\x84\x88R`\x80\x83\x01\x91\x87\x83R\x86_R`\x05` R`@_ \x80T`\x01`@\x1B\x81\x10\x15a\x0C%Wa\ns\x91`\x01\x82\x01\x81Ua\x12\xD3V[\x94\x90\x94a\x0C9WQ\x84T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x84UQ`\x01\x84\x01UQ`\x02\x83\x01U\x95Q\x80Q\x90\x96\x90`\x03\x83\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C%Wa\n\xC7\x82Ta\x13\0V[\x98`\x1F\x8A\x11a\x0B\xE0W[` \x99P\x89\x90`\x1F\x83\x11`\x01\x14a\x0BvW\x91\x80`\x04\x94\x92a\x0B\x1F\x97\x96\x94_\x92a\x0BkW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x15\x15\x91\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[T\x91\x7Fc\xD7-h\xDA\xD1\x02\xF2\xD2\xBF\x9DZ\xAD\xEAQ(\xA1e\xCA\x8A\x8Em\x97w\x80\xD7_\xFA\xE0\xFE\xEB\xEDa\x0BW`@Q\x93`@\x85R`@\x85\x01\x90a\x13\xD8V[\x92\x85\x87\x82\x01R\x803\x94\x03\x90\xA4`@Q\x90\x81R\xF3[\x01Q\x90P\x8C\x80a\n\xF5V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x0B\xC9WP\x92`\x01\x92\x85\x92a\x0B\x1F\x99\x98\x96`\x04\x98\x96\x10a\x0B\xB1W[PPP\x81\x1B\x01\x90Ua\x0B\tV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8C\x80\x80a\x0B\xA4V[\x92\x93\x8D`\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x0B\x86V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x9A` \x84\x10a\x0C\x1BW[`\x1F\x01`\x05\x1C\x01\x99[\x8A\x81\x10a\x0C\x10WPa\n\xD1V[_\x81U`\x01\x01a\x0C\x03V[\x90\x9AP\x8A\x90a\x0B\xFAV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`\x05\x81\x01a\x0CZ\x81Ta\x14IV[\x90Ua\n\x17V[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAWa\t\x1Da\t\x11a\x0C\x80a\x12\x84V[a\x14\xB3V[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`$5`\x045_R`\x05` R`@_ \x90\x81T\x81\x10\x15a\x01\xAAWa\x0C\xBA\x91a\x12\xD3V[P`\x01\x80`\xA0\x1B\x03\x81T\x16`\x01\x82\x01T\x91a\r\x0C`\x02\x82\x01T\x91`\xFF`\x04a\x0C\xE4`\x03\x84\x01a\x138V[\x92\x01T\x16\x92`@Q\x95\x86\x95\x86R` \x86\x01R`@\x85\x01R`\xA0``\x85\x01R`\xA0\x84\x01\x90a\x13\xD8V[\x90\x15\x15`\x80\x83\x01R\x03\x90\xF3[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x01`\x01`\xA0\x1B\x03a\r9a\x12\x84V[\x16_R`\x03` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\rtWa\t\x1D\x85a\t\x11\x81\x87\x03\x82a\x11\xCFV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\r]V[4a\x01\xAAW` 6`\x03\x19\x01\x12a\x01\xAAW`\x045_`\xC0`@Qa\r\xAD\x81a\x11\x97V[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R\x80_R`\x02` R`\x01\x80`\xA0\x1B\x03`@_ T\x16\x15a\x04)W_R`\x02` R`\xE0`@_ `@Qa\x0E\x03\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x91\x82\x82R`\x01\x81\x01T` \x83\x01\x90\x81R`\x02\x82\x01T`@\x84\x01\x90\x81R`\x03\x83\x01T\x90``\x85\x01\x91\x82R`\xFF`\x04\x85\x01T\x16\x92`\x80\x86\x01\x93\x15\x15\x84R`\xC0`\x06`\x05\x87\x01T\x96`\xA0\x89\x01\x97\x88R\x01T\x96\x01\x95\x86R`@Q\x96\x87RQ` \x87\x01RQ`@\x86\x01RQ``\x85\x01RQ\x15\x15`\x80\x84\x01RQ`\xA0\x83\x01RQ`\xC0\x82\x01R\xF3[4a\x01\xAAW`@6`\x03\x19\x01\x12a\x01\xAAW`\x045a\x0E\xAAa\x12nV[`@\x80Qc\x10\xA9\xF1\xF7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x92\x90\x82\x80`D\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x04\0W_\x90_\x93a\x10\xAEW[P\x15a\x03\xBDWa\x0F$\x83a\x14\xB3V[Q`\x06T\x11\x15a\x10\x9FW` \x92\x83\x92\x7F#I\xA0\xFA\t\xEDS\xCB\xDF\xF8\x87\xA1\xCE\xD6\x94W\"R\xD3CZ\xDFl\xE5N3\x0FPa\xC8g\x9Fa\x0F_`\x08Ta\x14IV[\x80`\x08U`@Q\x86\x81\x01\x91`\x01\x80`\xA0\x1B\x03\x86\x16\x95k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83R\x86`4\x83\x01RB`T\x83\x01RD`t\x83\x01R3``\x1B`\x94\x83\x01R`\xA8\x82\x01R`\xA8\x81Ra\x0F\xB8`\xC8\x82a\x11\xCFV[Q\x90 \x94\x85\x92`@Qa\x0F\xCA\x81a\x11\x97V[\x85\x81R`\x06\x83\x82\x01\x88\x81R`@\x83\x01B\x81Ra\x10^``\x85\x01\x86\x81R`\x80\x86\x01\x90`\x01\x82R`\xA0\x87\x01\x93_\x85R`\xC0\x88\x01\x95\x8C\x87R\x8C_R`\x02\x8BR`@_ \x98`\x01\x80`\xA0\x1B\x03\x90`\x01\x80`\xA0\x1B\x03\x90Q\x16\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x8AT\x16\x17\x89UQ`\x01\x89\x01UQ`\x02\x88\x01UQ`\x03\x87\x01UQ\x15\x15`\x04\x86\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[Q`\x05\x84\x01UQ\x91\x01U\x84_R`\x03\x82Ra\x10|\x84`@_ a\x14kV[\x85_R`\x04\x82Ra\x10\x90\x84`@_ a\x14kV[`@Q\x90\x81R\xA4`@Q\x90\x81R\xF3[c9-a\x8B`\xE1\x1B_R`\x04_\xFD[\x90Pa\x10\xCA\x91\x92P`@=`@\x11a\x03\xF9Wa\x03\xE9\x81\x83a\x11\xCFV[\x91\x90\x84a\x0F\x15V[4a\x01\xAAWa\x10\xE06a\x11\xF1V[_\x82\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04)W\x81_R`\x02` R`@_ `\x01\x80`\xA0\x1B\x03\x81T\x163\x14\x15\x80a\x11hW[a\x03\xBDW`\x04\x01\x80T`\xFF\x19\x16\x90U`@Q` \x80\x82R\x7F\x94\xD9l\x98\xBA\x93\x7F\x8E\xE9\xA0\x84\x02\xBA\xD1M\xF3\xF3%!\xF1\xC1\xA9A'\x06P;;\xA5\xA64 \x92\x82\x91a\x11c\x91\x90\x83\x01\x90a\x13\xD8V[\x03\x90\xA2\0[P`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15a\x11\x1BV[4a\x01\xAAW_6`\x03\x19\x01\x12a\x01\xAAW` \x90`\x06T\x81R\xF3[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C%W`@RV[\x90`@`\x03\x19\x83\x01\x12a\x01\xAAW`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xAAW\x81`#\x82\x01\x12\x15a\x01\xAAW\x80`\x04\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C%W`@Q\x92a\x12L`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x11\xCFV[\x82\x84R`$\x83\x83\x01\x01\x11a\x01\xAAW\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xAAWV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xAAWV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12\xBDWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12\xB0V[\x80T\x82\x10\x15a\x12\xECW_R`\x05` _ \x91\x02\x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x13.W[` \x83\x10\x14a\x13\x1AWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x13\x0FV[\x90`@Q\x91\x82_\x82T\x92a\x13K\x84a\x13\0V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x13\xB6WP`\x01\x14a\x13rW[Pa\x13p\x92P\x03\x83a\x11\xCFV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x13\x9AWPP\x90` a\x13p\x92\x82\x01\x01_a\x13cV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x13\x81V[\x90P` \x92Pa\x13p\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x13cV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C%W`\x05\x1B` \x01\x90V[\x80T\x82\x10\x15a\x12\xECW_R` _ \x01\x90_\x90V[\x91\x90\x82`@\x91\x03\x12a\x01\xAAW\x81Q\x80\x15\x15\x81\x03a\x01\xAAW` \x90\x92\x01Q\x90V[_\x19\x81\x14a\x14WW`\x01\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x0C%Wa\x14\x88\x91`\x01\x82\x01\x81Ua\x14\x14V[\x81\x92\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90UV[\x80Q\x82\x10\x15a\x12\xECW` \x91`\x05\x1B\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x16_R`\x03` R`@_ \x90`@Q\x80\x83` \x82\x95T\x93\x84\x81R\x01\x90_R` _ \x92_[\x81\x81\x10a\x16\xA3WPPa\x14\xF5\x92P\x03\x83a\x11\xCFV[__[\x83Q\x81\x10\x15a\x15\xA6Wa\x15\x0B\x81\x85a\x14\x9FV[Q_R`\x02` R`@_ `@Qa\x15#\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T\x90\x81``\x82\x01R`\xC0`\x06`\xFF`\x04\x86\x01T\x16\x15\x15\x94\x85`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01R\x81a\x15\x9BW[Pa\x15\x87W[`\x01\x01a\x14\xF8V[\x90a\x15\x93`\x01\x91a\x14IV[\x91\x90Pa\x15\x7FV[\x90PB\x11\x15_a\x15yV[Pa\x15\xB0\x81a\x13\xFCV[\x90a\x15\xBE`@Q\x92\x83a\x11\xCFV[\x80\x82Ra\x15\xCD`\x1F\x19\x91a\x13\xFCV[\x016` \x83\x017_\x92_[\x81Q\x81\x10\x15a\x16\x9CWa\x15\xEB\x81\x83a\x14\x9FV[Q_R`\x02` R`@_ `@Qa\x16\x03\x81a\x11\x97V[`\x01\x80`\xA0\x1B\x03\x82T\x16\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T\x90\x81``\x82\x01R`\xC0`\x06`\xFF`\x04\x86\x01T\x16\x15\x15\x94\x85`\x80\x85\x01R`\x05\x81\x01T`\xA0\x85\x01R\x01T\x91\x01R\x81a\x16\x91W[Pa\x16gW[`\x01\x01a\x15\xD8V[\x93a\x16\x89`\x01\x91a\x16x\x87\x85a\x14\x9FV[Qa\x16\x83\x82\x87a\x14\x9FV[Ra\x14IV[\x94\x90Pa\x16_V[\x90PB\x11\x15_a\x16YV[P\x90\x92PPV[\x84T\x83R`\x01\x94\x85\x01\x94\x87\x94P` \x90\x93\x01\x92\x01a\x14\xE0V[\x91\x90\x82\x01\x80\x92\x11a\x14WWV[\x91\x90\x91_R`\x02` R`@_ `@Q\x92a\x16\xE4\x84a\x11\x97V[\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x86R`\x01\x84\x01T` \x87\x01R`\x02\x84\x01T`@\x87\x01R`\x03\x84\x01T``\x87\x01\x81\x90R`\x04\x85\x01T`\xFF\x16\x15\x80\x15`\x80\x89\x01R`\x05\x86\x01T`\xA0\x89\x01R`\x06\x90\x95\x01T`\xC0\x90\x97\x01\x96\x90\x96R\x91\x16\x14\x80\x15\x91\x90a\x17lW[Pa\x17eW\x81B\x11a\x17eWB\x82\x03\x91\x82\x11a\x14WW`\x01\x91\x90V[_\x91P\x81\x90V[\x90P_a\x17IV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x88WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xC8\xF9\xA2\xD3\x1E\x7FH|q\x81\xF0\xF1p\xEC\xE63\xD9\xE4\xD0:\xD70\xFB[T\x9F\x88\xF1\x824\xEB0dsolcC\0\x08\x1E\x003";
    /// The deployed bytecode of the contract.
    pub static ACCESSCONTROL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AccessControl<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AccessControl<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AccessControl<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AccessControl<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AccessControl<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AccessControl))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AccessControl<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACCESSCONTROL_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ACCESSCONTROL_ABI.clone(),
                ACCESSCONTROL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `accessSessions` (0x73c1f4a0) function
        pub fn access_sessions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([115, 193, 244, 160], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cleanupExpiredSessions` (0x5290cf96) function
        pub fn cleanup_expired_sessions(
            &self,
            session_keys: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 144, 207, 150], session_keys)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAccessSession` (0x13a5a322) function
        pub fn create_access_session(
            &self,
            product_id: ::ethers::core::types::U256,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 165, 163, 34], (product_id, user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dataMarketplace` (0x699cb156) function
        pub fn data_marketplace(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 156, 177, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extendSession` (0x9d6c62cf) function
        pub fn extend_session(
            &self,
            session_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 108, 98, 207], session_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccessSession` (0x166ff80a) function
        pub fn get_access_session(
            &self,
            session_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, AccessSession> {
            self.0
                .method_hash([22, 111, 248, 10], session_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProductSessions` (0x3b5225ff) function
        pub fn get_product_sessions(
            &self,
            product_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([59, 82, 37, 255], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSessionRequests` (0x86232926) function
        pub fn get_session_requests(
            &self,
            session_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<DataRequest>,
        > {
            self.0
                .method_hash([134, 35, 41, 38], session_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserActiveSessions` (0x31c3a612) function
        pub fn get_user_active_sessions(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([49, 195, 166, 18], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserSessions` (0x223715dd) function
        pub fn get_user_sessions(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([34, 55, 21, 221], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxConcurrentSessions` (0x0aeeaad7) function
        pub fn max_concurrent_sessions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 238, 170, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `productSessions` (0xd9775c3f) function
        pub fn product_sessions(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([217, 119, 92, 63], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestDataAccess` (0x37cb94aa) function
        pub fn request_data_access(
            &self,
            session_key: [u8; 32],
            request_type: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 203, 148, 170], (session_key, request_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sessionRequests` (0x29797512) function
        pub fn session_requests(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::std::string::String,
                bool,
            ),
        > {
            self.0
                .method_hash([41, 121, 117, 18], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sessionTimeoutBuffer` (0xd3781f80) function
        pub fn session_timeout_buffer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 120, 31, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMaxConcurrentSessions` (0xe33bb6ea) function
        pub fn set_max_concurrent_sessions(
            &self,
            new_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 59, 182, 234], new_limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSessionTimeoutBuffer` (0xdfd32306) function
        pub fn set_session_timeout_buffer(
            &self,
            new_buffer: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 211, 35, 6], new_buffer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `terminateSession` (0x114800bc) function
        pub fn terminate_session(
            &self,
            session_key: [u8; 32],
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 72, 0, 188], (session_key, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userSessions` (0x6d597d13) function
        pub fn user_sessions(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([109, 89, 125, 19], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateAccess` (0x3fb58f74) function
        pub fn validate_access(
            &self,
            session_key: [u8; 32],
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([63, 181, 143, 116], (session_key, user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vehicleRegistry` (0x9d56f085) function
        pub fn vehicle_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 86, 240, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccessSessionCreated` event
        pub fn access_session_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccessSessionCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AccessSessionTerminated` event
        pub fn access_session_terminated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccessSessionTerminatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DataAccessRequested` event
        pub fn data_access_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataAccessRequestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SessionLimitUpdated` event
        pub fn session_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SessionLimitUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccessControlEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AccessControl<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidSessionKey` with signature `InvalidSessionKey()` and selector `0xbf10e9ba`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSessionKey", abi = "InvalidSessionKey()")]
    pub struct InvalidSessionKey;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `ReentrancyGuardReentrantCall` with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ReentrancyGuardReentrantCall",
        abi = "ReentrancyGuardReentrantCall()"
    )]
    pub struct ReentrancyGuardReentrantCall;
    ///Custom Error type `SessionExpired` with signature `SessionExpired()` and selector `0x1fd05a4a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SessionExpired", abi = "SessionExpired()")]
    pub struct SessionExpired;
    ///Custom Error type `SessionNotActive` with signature `SessionNotActive()` and selector `0x27406db9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SessionNotActive", abi = "SessionNotActive()")]
    pub struct SessionNotActive;
    ///Custom Error type `SessionNotFound` with signature `SessionNotFound()` and selector `0x96c95f81`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SessionNotFound", abi = "SessionNotFound()")]
    pub struct SessionNotFound;
    ///Custom Error type `TooManyConcurrentSessions` with signature `TooManyConcurrentSessions()` and selector `0x725ac316`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TooManyConcurrentSessions", abi = "TooManyConcurrentSessions()")]
    pub struct TooManyConcurrentSessions;
    ///Custom Error type `UnauthorizedAccess` with signature `UnauthorizedAccess()` and selector `0x344fd586`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnauthorizedAccess", abi = "UnauthorizedAccess()")]
    pub struct UnauthorizedAccess;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccessControlErrors {
        InvalidSessionKey(InvalidSessionKey),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        SessionExpired(SessionExpired),
        SessionNotActive(SessionNotActive),
        SessionNotFound(SessionNotFound),
        TooManyConcurrentSessions(TooManyConcurrentSessions),
        UnauthorizedAccess(UnauthorizedAccess),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AccessControlErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidSessionKey as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSessionKey(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <SessionExpired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionExpired(decoded));
            }
            if let Ok(decoded) = <SessionNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionNotActive(decoded));
            }
            if let Ok(decoded) = <SessionNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionNotFound(decoded));
            }
            if let Ok(decoded) = <TooManyConcurrentSessions as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TooManyConcurrentSessions(decoded));
            }
            if let Ok(decoded) = <UnauthorizedAccess as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnauthorizedAccess(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccessControlErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSessionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TooManyConcurrentSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnauthorizedAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AccessControlErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidSessionKey as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SessionExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SessionNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SessionNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TooManyConcurrentSessions as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnauthorizedAccess as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AccessControlErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSessionKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SessionExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::SessionNotActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::SessionNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::TooManyConcurrentSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnauthorizedAccess(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AccessControlErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSessionKey> for AccessControlErrors {
        fn from(value: InvalidSessionKey) -> Self {
            Self::InvalidSessionKey(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for AccessControlErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for AccessControlErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for AccessControlErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<SessionExpired> for AccessControlErrors {
        fn from(value: SessionExpired) -> Self {
            Self::SessionExpired(value)
        }
    }
    impl ::core::convert::From<SessionNotActive> for AccessControlErrors {
        fn from(value: SessionNotActive) -> Self {
            Self::SessionNotActive(value)
        }
    }
    impl ::core::convert::From<SessionNotFound> for AccessControlErrors {
        fn from(value: SessionNotFound) -> Self {
            Self::SessionNotFound(value)
        }
    }
    impl ::core::convert::From<TooManyConcurrentSessions> for AccessControlErrors {
        fn from(value: TooManyConcurrentSessions) -> Self {
            Self::TooManyConcurrentSessions(value)
        }
    }
    impl ::core::convert::From<UnauthorizedAccess> for AccessControlErrors {
        fn from(value: UnauthorizedAccess) -> Self {
            Self::UnauthorizedAccess(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AccessSessionCreated",
        abi = "AccessSessionCreated(bytes32,address,uint256,uint256)"
    )]
    pub struct AccessSessionCreatedFilter {
        #[ethevent(indexed)]
        pub session_key: [u8; 32],
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AccessSessionTerminated",
        abi = "AccessSessionTerminated(bytes32,string)"
    )]
    pub struct AccessSessionTerminatedFilter {
        #[ethevent(indexed)]
        pub session_key: [u8; 32],
        pub reason: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DataAccessRequested",
        abi = "DataAccessRequested(bytes32,address,uint256,string,bool)"
    )]
    pub struct DataAccessRequestedFilter {
        #[ethevent(indexed)]
        pub session_key: [u8; 32],
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        pub request_type: ::std::string::String,
        pub authorized: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SessionLimitUpdated", abi = "SessionLimitUpdated(uint256)")]
    pub struct SessionLimitUpdatedFilter {
        pub new_limit: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccessControlEvents {
        AccessSessionCreatedFilter(AccessSessionCreatedFilter),
        AccessSessionTerminatedFilter(AccessSessionTerminatedFilter),
        DataAccessRequestedFilter(DataAccessRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SessionLimitUpdatedFilter(SessionLimitUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AccessControlEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccessSessionCreatedFilter::decode_log(log) {
                return Ok(AccessControlEvents::AccessSessionCreatedFilter(decoded));
            }
            if let Ok(decoded) = AccessSessionTerminatedFilter::decode_log(log) {
                return Ok(AccessControlEvents::AccessSessionTerminatedFilter(decoded));
            }
            if let Ok(decoded) = DataAccessRequestedFilter::decode_log(log) {
                return Ok(AccessControlEvents::DataAccessRequestedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AccessControlEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SessionLimitUpdatedFilter::decode_log(log) {
                return Ok(AccessControlEvents::SessionLimitUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AccessControlEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessSessionCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AccessSessionTerminatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DataAccessRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SessionLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccessSessionCreatedFilter> for AccessControlEvents {
        fn from(value: AccessSessionCreatedFilter) -> Self {
            Self::AccessSessionCreatedFilter(value)
        }
    }
    impl ::core::convert::From<AccessSessionTerminatedFilter> for AccessControlEvents {
        fn from(value: AccessSessionTerminatedFilter) -> Self {
            Self::AccessSessionTerminatedFilter(value)
        }
    }
    impl ::core::convert::From<DataAccessRequestedFilter> for AccessControlEvents {
        fn from(value: DataAccessRequestedFilter) -> Self {
            Self::DataAccessRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AccessControlEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SessionLimitUpdatedFilter> for AccessControlEvents {
        fn from(value: SessionLimitUpdatedFilter) -> Self {
            Self::SessionLimitUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `accessSessions` function with signature `accessSessions(bytes32)` and selector `0x73c1f4a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accessSessions", abi = "accessSessions(bytes32)")]
    pub struct AccessSessionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `cleanupExpiredSessions` function with signature `cleanupExpiredSessions(bytes32[])` and selector `0x5290cf96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "cleanupExpiredSessions",
        abi = "cleanupExpiredSessions(bytes32[])"
    )]
    pub struct CleanupExpiredSessionsCall {
        pub session_keys: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `createAccessSession` function with signature `createAccessSession(uint256,address)` and selector `0x13a5a322`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createAccessSession",
        abi = "createAccessSession(uint256,address)"
    )]
    pub struct CreateAccessSessionCall {
        pub product_id: ::ethers::core::types::U256,
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `dataMarketplace` function with signature `dataMarketplace()` and selector `0x699cb156`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dataMarketplace", abi = "dataMarketplace()")]
    pub struct DataMarketplaceCall;
    ///Container type for all input parameters for the `extendSession` function with signature `extendSession(bytes32)` and selector `0x9d6c62cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "extendSession", abi = "extendSession(bytes32)")]
    pub struct ExtendSessionCall {
        pub session_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getAccessSession` function with signature `getAccessSession(bytes32)` and selector `0x166ff80a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAccessSession", abi = "getAccessSession(bytes32)")]
    pub struct GetAccessSessionCall {
        pub session_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getProductSessions` function with signature `getProductSessions(uint256)` and selector `0x3b5225ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getProductSessions", abi = "getProductSessions(uint256)")]
    pub struct GetProductSessionsCall {
        pub product_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSessionRequests` function with signature `getSessionRequests(bytes32)` and selector `0x86232926`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSessionRequests", abi = "getSessionRequests(bytes32)")]
    pub struct GetSessionRequestsCall {
        pub session_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUserActiveSessions` function with signature `getUserActiveSessions(address)` and selector `0x31c3a612`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUserActiveSessions", abi = "getUserActiveSessions(address)")]
    pub struct GetUserActiveSessionsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserSessions` function with signature `getUserSessions(address)` and selector `0x223715dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUserSessions", abi = "getUserSessions(address)")]
    pub struct GetUserSessionsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxConcurrentSessions` function with signature `maxConcurrentSessions()` and selector `0x0aeeaad7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxConcurrentSessions", abi = "maxConcurrentSessions()")]
    pub struct MaxConcurrentSessionsCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `productSessions` function with signature `productSessions(uint256,uint256)` and selector `0xd9775c3f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "productSessions", abi = "productSessions(uint256,uint256)")]
    pub struct ProductSessionsCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `requestDataAccess` function with signature `requestDataAccess(bytes32,string)` and selector `0x37cb94aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "requestDataAccess", abi = "requestDataAccess(bytes32,string)")]
    pub struct RequestDataAccessCall {
        pub session_key: [u8; 32],
        pub request_type: ::std::string::String,
    }
    ///Container type for all input parameters for the `sessionRequests` function with signature `sessionRequests(bytes32,uint256)` and selector `0x29797512`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sessionRequests", abi = "sessionRequests(bytes32,uint256)")]
    pub struct SessionRequestsCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `sessionTimeoutBuffer` function with signature `sessionTimeoutBuffer()` and selector `0xd3781f80`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sessionTimeoutBuffer", abi = "sessionTimeoutBuffer()")]
    pub struct SessionTimeoutBufferCall;
    ///Container type for all input parameters for the `setMaxConcurrentSessions` function with signature `setMaxConcurrentSessions(uint256)` and selector `0xe33bb6ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setMaxConcurrentSessions",
        abi = "setMaxConcurrentSessions(uint256)"
    )]
    pub struct SetMaxConcurrentSessionsCall {
        pub new_limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSessionTimeoutBuffer` function with signature `setSessionTimeoutBuffer(uint256)` and selector `0xdfd32306`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setSessionTimeoutBuffer",
        abi = "setSessionTimeoutBuffer(uint256)"
    )]
    pub struct SetSessionTimeoutBufferCall {
        pub new_buffer: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `terminateSession` function with signature `terminateSession(bytes32,string)` and selector `0x114800bc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "terminateSession", abi = "terminateSession(bytes32,string)")]
    pub struct TerminateSessionCall {
        pub session_key: [u8; 32],
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userSessions` function with signature `userSessions(address,uint256)` and selector `0x6d597d13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "userSessions", abi = "userSessions(address,uint256)")]
    pub struct UserSessionsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `validateAccess` function with signature `validateAccess(bytes32,address)` and selector `0x3fb58f74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validateAccess", abi = "validateAccess(bytes32,address)")]
    pub struct ValidateAccessCall {
        pub session_key: [u8; 32],
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `vehicleRegistry` function with signature `vehicleRegistry()` and selector `0x9d56f085`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vehicleRegistry", abi = "vehicleRegistry()")]
    pub struct VehicleRegistryCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccessControlCalls {
        AccessSessions(AccessSessionsCall),
        CleanupExpiredSessions(CleanupExpiredSessionsCall),
        CreateAccessSession(CreateAccessSessionCall),
        DataMarketplace(DataMarketplaceCall),
        ExtendSession(ExtendSessionCall),
        GetAccessSession(GetAccessSessionCall),
        GetProductSessions(GetProductSessionsCall),
        GetSessionRequests(GetSessionRequestsCall),
        GetUserActiveSessions(GetUserActiveSessionsCall),
        GetUserSessions(GetUserSessionsCall),
        MaxConcurrentSessions(MaxConcurrentSessionsCall),
        Owner(OwnerCall),
        ProductSessions(ProductSessionsCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequestDataAccess(RequestDataAccessCall),
        SessionRequests(SessionRequestsCall),
        SessionTimeoutBuffer(SessionTimeoutBufferCall),
        SetMaxConcurrentSessions(SetMaxConcurrentSessionsCall),
        SetSessionTimeoutBuffer(SetSessionTimeoutBufferCall),
        TerminateSession(TerminateSessionCall),
        TransferOwnership(TransferOwnershipCall),
        UserSessions(UserSessionsCall),
        ValidateAccess(ValidateAccessCall),
        VehicleRegistry(VehicleRegistryCall),
    }
    impl ::ethers::core::abi::AbiDecode for AccessControlCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessSessions(decoded));
            }
            if let Ok(decoded) = <CleanupExpiredSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CleanupExpiredSessions(decoded));
            }
            if let Ok(decoded) = <CreateAccessSessionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAccessSession(decoded));
            }
            if let Ok(decoded) = <DataMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataMarketplace(decoded));
            }
            if let Ok(decoded) = <ExtendSessionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExtendSession(decoded));
            }
            if let Ok(decoded) = <GetAccessSessionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAccessSession(decoded));
            }
            if let Ok(decoded) = <GetProductSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProductSessions(decoded));
            }
            if let Ok(decoded) = <GetSessionRequestsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSessionRequests(decoded));
            }
            if let Ok(decoded) = <GetUserActiveSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserActiveSessions(decoded));
            }
            if let Ok(decoded) = <GetUserSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUserSessions(decoded));
            }
            if let Ok(decoded) = <MaxConcurrentSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxConcurrentSessions(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProductSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProductSessions(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RequestDataAccessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestDataAccess(decoded));
            }
            if let Ok(decoded) = <SessionRequestsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionRequests(decoded));
            }
            if let Ok(decoded) = <SessionTimeoutBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SessionTimeoutBuffer(decoded));
            }
            if let Ok(decoded) = <SetMaxConcurrentSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMaxConcurrentSessions(decoded));
            }
            if let Ok(decoded) = <SetSessionTimeoutBufferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSessionTimeoutBuffer(decoded));
            }
            if let Ok(decoded) = <TerminateSessionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TerminateSession(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UserSessionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserSessions(decoded));
            }
            if let Ok(decoded) = <ValidateAccessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateAccess(decoded));
            }
            if let Ok(decoded) = <VehicleRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleRegistry(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccessControlCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccessSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CleanupExpiredSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAccessSession(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DataMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtendSession(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAccessSession(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProductSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSessionRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserActiveSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxConcurrentSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProductSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestDataAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionRequests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SessionTimeoutBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMaxConcurrentSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSessionTimeoutBuffer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TerminateSession(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserSessions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AccessControlCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessSessions(element) => ::core::fmt::Display::fmt(element, f),
                Self::CleanupExpiredSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateAccessSession(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DataMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtendSession(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccessSession(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSessionRequests(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserActiveSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserSessions(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxConcurrentSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductSessions(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestDataAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::SessionRequests(element) => ::core::fmt::Display::fmt(element, f),
                Self::SessionTimeoutBuffer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMaxConcurrentSessions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSessionTimeoutBuffer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TerminateSession(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserSessions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::VehicleRegistry(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessSessionsCall> for AccessControlCalls {
        fn from(value: AccessSessionsCall) -> Self {
            Self::AccessSessions(value)
        }
    }
    impl ::core::convert::From<CleanupExpiredSessionsCall> for AccessControlCalls {
        fn from(value: CleanupExpiredSessionsCall) -> Self {
            Self::CleanupExpiredSessions(value)
        }
    }
    impl ::core::convert::From<CreateAccessSessionCall> for AccessControlCalls {
        fn from(value: CreateAccessSessionCall) -> Self {
            Self::CreateAccessSession(value)
        }
    }
    impl ::core::convert::From<DataMarketplaceCall> for AccessControlCalls {
        fn from(value: DataMarketplaceCall) -> Self {
            Self::DataMarketplace(value)
        }
    }
    impl ::core::convert::From<ExtendSessionCall> for AccessControlCalls {
        fn from(value: ExtendSessionCall) -> Self {
            Self::ExtendSession(value)
        }
    }
    impl ::core::convert::From<GetAccessSessionCall> for AccessControlCalls {
        fn from(value: GetAccessSessionCall) -> Self {
            Self::GetAccessSession(value)
        }
    }
    impl ::core::convert::From<GetProductSessionsCall> for AccessControlCalls {
        fn from(value: GetProductSessionsCall) -> Self {
            Self::GetProductSessions(value)
        }
    }
    impl ::core::convert::From<GetSessionRequestsCall> for AccessControlCalls {
        fn from(value: GetSessionRequestsCall) -> Self {
            Self::GetSessionRequests(value)
        }
    }
    impl ::core::convert::From<GetUserActiveSessionsCall> for AccessControlCalls {
        fn from(value: GetUserActiveSessionsCall) -> Self {
            Self::GetUserActiveSessions(value)
        }
    }
    impl ::core::convert::From<GetUserSessionsCall> for AccessControlCalls {
        fn from(value: GetUserSessionsCall) -> Self {
            Self::GetUserSessions(value)
        }
    }
    impl ::core::convert::From<MaxConcurrentSessionsCall> for AccessControlCalls {
        fn from(value: MaxConcurrentSessionsCall) -> Self {
            Self::MaxConcurrentSessions(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AccessControlCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProductSessionsCall> for AccessControlCalls {
        fn from(value: ProductSessionsCall) -> Self {
            Self::ProductSessions(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AccessControlCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequestDataAccessCall> for AccessControlCalls {
        fn from(value: RequestDataAccessCall) -> Self {
            Self::RequestDataAccess(value)
        }
    }
    impl ::core::convert::From<SessionRequestsCall> for AccessControlCalls {
        fn from(value: SessionRequestsCall) -> Self {
            Self::SessionRequests(value)
        }
    }
    impl ::core::convert::From<SessionTimeoutBufferCall> for AccessControlCalls {
        fn from(value: SessionTimeoutBufferCall) -> Self {
            Self::SessionTimeoutBuffer(value)
        }
    }
    impl ::core::convert::From<SetMaxConcurrentSessionsCall> for AccessControlCalls {
        fn from(value: SetMaxConcurrentSessionsCall) -> Self {
            Self::SetMaxConcurrentSessions(value)
        }
    }
    impl ::core::convert::From<SetSessionTimeoutBufferCall> for AccessControlCalls {
        fn from(value: SetSessionTimeoutBufferCall) -> Self {
            Self::SetSessionTimeoutBuffer(value)
        }
    }
    impl ::core::convert::From<TerminateSessionCall> for AccessControlCalls {
        fn from(value: TerminateSessionCall) -> Self {
            Self::TerminateSession(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AccessControlCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UserSessionsCall> for AccessControlCalls {
        fn from(value: UserSessionsCall) -> Self {
            Self::UserSessions(value)
        }
    }
    impl ::core::convert::From<ValidateAccessCall> for AccessControlCalls {
        fn from(value: ValidateAccessCall) -> Self {
            Self::ValidateAccess(value)
        }
    }
    impl ::core::convert::From<VehicleRegistryCall> for AccessControlCalls {
        fn from(value: VehicleRegistryCall) -> Self {
            Self::VehicleRegistry(value)
        }
    }
    ///Container type for all return fields from the `accessSessions` function with signature `accessSessions(bytes32)` and selector `0x73c1f4a0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccessSessionsReturn {
        pub user: ::ethers::core::types::Address,
        pub product_id: ::ethers::core::types::U256,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub is_active: bool,
        pub data_requests: ::ethers::core::types::U256,
        pub session_key: [u8; 32],
    }
    ///Container type for all return fields from the `createAccessSession` function with signature `createAccessSession(uint256,address)` and selector `0x13a5a322`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateAccessSessionReturn {
        pub session_key: [u8; 32],
    }
    ///Container type for all return fields from the `dataMarketplace` function with signature `dataMarketplace()` and selector `0x699cb156`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DataMarketplaceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAccessSession` function with signature `getAccessSession(bytes32)` and selector `0x166ff80a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAccessSessionReturn(pub AccessSession);
    ///Container type for all return fields from the `getProductSessions` function with signature `getProductSessions(uint256)` and selector `0x3b5225ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetProductSessionsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getSessionRequests` function with signature `getSessionRequests(bytes32)` and selector `0x86232926`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSessionRequestsReturn(pub ::std::vec::Vec<DataRequest>);
    ///Container type for all return fields from the `getUserActiveSessions` function with signature `getUserActiveSessions(address)` and selector `0x31c3a612`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetUserActiveSessionsReturn {
        pub active_sessions: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `getUserSessions` function with signature `getUserSessions(address)` and selector `0x223715dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetUserSessionsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `maxConcurrentSessions` function with signature `maxConcurrentSessions()` and selector `0x0aeeaad7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxConcurrentSessionsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `productSessions` function with signature `productSessions(uint256,uint256)` and selector `0xd9775c3f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProductSessionsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `requestDataAccess` function with signature `requestDataAccess(bytes32,string)` and selector `0x37cb94aa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RequestDataAccessReturn {
        pub authorized: bool,
    }
    ///Container type for all return fields from the `sessionRequests` function with signature `sessionRequests(bytes32,uint256)` and selector `0x29797512`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SessionRequestsReturn {
        pub user: ::ethers::core::types::Address,
        pub product_id: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub request_type: ::std::string::String,
        pub was_authorized: bool,
    }
    ///Container type for all return fields from the `sessionTimeoutBuffer` function with signature `sessionTimeoutBuffer()` and selector `0xd3781f80`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SessionTimeoutBufferReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userSessions` function with signature `userSessions(address,uint256)` and selector `0x6d597d13`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UserSessionsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `validateAccess` function with signature `validateAccess(bytes32,address)` and selector `0x3fb58f74`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidateAccessReturn {
        pub is_valid: bool,
        pub time_remaining: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `vehicleRegistry` function with signature `vehicleRegistry()` and selector `0x9d56f085`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VehicleRegistryReturn(pub ::ethers::core::types::Address);
    ///`AccessSession(address,uint256,uint256,uint256,bool,uint256,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccessSession {
        pub user: ::ethers::core::types::Address,
        pub product_id: ::ethers::core::types::U256,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub is_active: bool,
        pub data_requests: ::ethers::core::types::U256,
        pub session_key: [u8; 32],
    }
    ///`DataRequest(address,uint256,uint256,string,bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DataRequest {
        pub user: ::ethers::core::types::Address,
        pub product_id: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
        pub request_type: ::std::string::String,
        pub was_authorized: bool,
    }
}
