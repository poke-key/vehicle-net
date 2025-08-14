pub use data_marketplace::*;
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
pub mod data_marketplace {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("MAX_PLATFORM_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_PLATFORM_FEE"),
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
                    ::std::borrow::ToOwned::to_owned("buyerPurchases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("buyerPurchases"),
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
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountPaid"),
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
                                    name: ::std::borrow::ToOwned::to_owned("accessToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dataProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dataProducts"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pricePerHour"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDuration"),
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
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apiEndpoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("createdAt"),
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
                    ::std::borrow::ToOwned::to_owned("getBuyerPurchases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBuyerPurchases"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataMarketplace.Purchase[]",
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
                    ::std::borrow::ToOwned::to_owned("getDataProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDataProduct"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataMarketplace.DataProduct",
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
                    ::std::borrow::ToOwned::to_owned("getProductPurchases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProductPurchases",
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataMarketplace.Purchase[]",
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
                    ::std::borrow::ToOwned::to_owned("getStreamingPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStreamingPayment",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("streamingId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataMarketplace.StreamingPayment",
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
                    ::std::borrow::ToOwned::to_owned("getTotalProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalProducts"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalStreamingPayments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStreamingPayments",
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
                    ::std::borrow::ToOwned::to_owned("hasValidAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasValidAccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("listDataProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("listDataProduct"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pricePerHour"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apiEndpoint"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("platformFeeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("platformFeeRate"),
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
                    ::std::borrow::ToOwned::to_owned("productCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("productCounter"),
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
                    ::std::borrow::ToOwned::to_owned("productPurchases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("productPurchases"),
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
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountPaid"),
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
                                    name: ::std::borrow::ToOwned::to_owned("accessToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("purchaseDataAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("purchaseDataAccess"),
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
                                    name: ::std::borrow::ToOwned::to_owned("durationInSeconds"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("setPlatformFeeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPlatformFeeRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeRate"),
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
                    ::std::borrow::ToOwned::to_owned("startStreamingPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "startStreamingPayment",
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
                                    name: ::std::borrow::ToOwned::to_owned("ratePerSecond"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("streamingCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("streamingCounter"),
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
                    ::std::borrow::ToOwned::to_owned("streamingPayments"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("streamingPayments"),
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
                            ],
                            outputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ratePerSecond"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastWithdrawal"),
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
                                    name: ::std::borrow::ToOwned::to_owned("isActive"),
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
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDataProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDataProduct"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newPricePerHour"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newDescription"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newApiEndpoint"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawPlatformFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawPlatformFees",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawStreamingPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawStreamingPayment",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("streamingId"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DataProductListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DataProductListed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dataType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pricePerHour"),
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
                    ::std::borrow::ToOwned::to_owned("DataProductUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DataProductUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isActive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DataPurchased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DataPurchased"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("duration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountPaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("purchaseIndex"),
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
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PlatformFeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PlatformFeeUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeRate"),
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
                    ::std::borrow::ToOwned::to_owned("StreamingPaymentStarted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StreamingPaymentStarted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("streamingId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("buyer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ratePerSecond"),
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
                    ::std::borrow::ToOwned::to_owned("StreamingPaymentWithdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StreamingPaymentWithdrawn",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("streamingId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleWallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("AccessExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AccessExpired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EnforcedPause"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExpectedPause"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientPayment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDuration"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeeRate"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPrice"),
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
                    ::std::borrow::ToOwned::to_owned("ProductNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ProductNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProductNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ProductNotFound"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("StreamingNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StreamingNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnauthorizedSeller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnauthorizedSeller"),
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
    pub static DATAMARKETPLACE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04a\x01\x03W`\x1Fa%\xEE8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x07W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\x03WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\x03W`\x01_U3\x15a\0\xF0W`\x01\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x163`\x08\x81\x81\x1Ba\x01\0`\x01`\xA8\x1B\x03\x16\x92\x90\x92\x17\x90\x93U`@Q\x93\x92\x91\x90\x1C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3`\xFA`\x08U`\x80Ra$\xD2\x90\x81a\x01\x1C\x829`\x80Q\x81\x81\x81a\x04S\x01R\x81\x81a\x0B\xFE\x01R\x81\x81a\x12\x82\x01R\x81\x81a\x17\xF2\x01Ra\x1C(\x01R\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0E\x91\x91Y\x14a\x1A\xFDW\x80c\x14\xFF&n\x14a\x03\xA4W\x80c\x1F\x16e;\x14a\x15\xB5W\x80c!S\xE3\xEE\x14a\x19pW\x80c#\x0C\x8D\xD3\x14a\x17\x0CW\x80c4\xB7\xBF\xEF\x14a\x16\xC7W\x80c8\xBB\xD4\xE1\x14a\x16TW\x80c9\x98\xA6\x81\x14a\x168W\x80c?K\xA8:\x14a\x15\xD2W\x80cW\n\xEA\x8A\x14a\x15\xB5W\x80c\\\x97Z\xBB\x14a\x15\x93W\x80c_zim\x14a\x11\xDCW\x80cqP\x18\xA6\x14a\x11}W\x80cvp\n\x8B\x14a\x10\xB6W\x80c\x84V\xCBY\x14a\x10^W\x80c\x85\xA3C\x93\x14a\x0FuW\x80c\x86kt\x80\x14a\r\xB1W\x80c\x8D\xA5\xCB[\x14a\r\x85W\x80c\x92\x7F\xEF.\x14a\r W\x80c\x96\x96HH\x14a\x0C-W\x80c\x9DV\xF0\x85\x14a\x0B\xE9W\x80c\xB8\xB6P\xBF\x14a\x0B\nW\x80c\xCDV\xF0\xAC\x14a\n\x92W\x80c\xD0\xB7\x83\x0B\x14a\n\x0FW\x80c\xD4\xD8\xDF\xFA\x14a\x03\xC1W\x80c\xDC3\t\xAD\x14a\x03\xA4W\x80c\xEE\xCA\x08\xF0\x14a\x03\x87W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xEEWc\xFC\xC3&p\x14a\x01XW_\x80\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_a\x01\0`@Qa\x01|\x81a\x1E\xB3V[\x82\x81R``` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R```\xC0\x82\x01R```\xE0\x82\x01R\x01R\x80\x15\x80\x15a\x02\xDFW[a\x02\xD0W_R`\x02` R`@_ `@Qa\x01\xD2\x81a\x1E\xB3V[\x81T\x81Ra\x01\xE2`\x01\x83\x01a\x1F\x0BV[\x91` \x82\x01\x92\x83Ra\x02\xC4`\x02\x82\x01T\x91`@\x84\x01\x92\x83Ra\x02\xB0`\x03\x82\x01T\x93``\x86\x01\x94\x85R`\x04\x83\x01T`\x80\x87\x01\x90\x81R`\xFF`\x05\x85\x01T\x16`\xA0\x88\x01\x90\x15\x15\x81Ra\x023`\x06\x86\x01a\x1F\x0BV[\x91`\xC0\x89\x01\x92\x83R`\x08a\x02I`\x07\x88\x01a\x1F\x0BV[\x96`\xE0\x8B\x01\x97\x88R\x01T\x97a\x01\0\x8A\x01\x98\x89Ra\x02\x83`@Q\x9B\x8C\x9B` \x8DRQ` \x8D\x01RQa\x01 `@\x8D\x01Ra\x01@\x8C\x01\x90a\x1F\xABV[\x94Q``\x8B\x01RQ`\x80\x8A\x01RQ`\xA0\x89\x01RQ\x15\x15`\xC0\x88\x01RQ\x86\x82\x03`\x1F\x19\x01`\xE0\x88\x01Ra\x1F\xABV[\x90Q\x84\x82\x03`\x1F\x19\x01a\x01\0\x86\x01Ra\x1F\xABV[\x90Qa\x01 \x83\x01R\x03\x90\xF3[cy\xDEJ\xF5`\xE0\x1B_R`\x04_\xFD[P`\x06T\x81\x11a\x01\xB7V[_\x80\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAWa\x03\x07a\x1E8V[a\x03\x0Fa$rV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x03tW`\x01\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16`\x08\x93\x84\x1Ba\x01\0`\x01`\xA8\x1B\x03\x16\x17\x90\x91U\x90\x1C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x08T`@Q\x90\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x07T`@Q\x90\x81R\xF3[4a\x02\xEAW`\xE06`\x03\x19\x01\x12a\x02\xEAW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x03\xF4\x906\x90`\x04\x01a ,V[`\x845\x90`D5`d5`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x04\x1D\x906\x90`\x04\x01a ,V[\x93`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x04=\x906\x90`\x04\x01a ,V[`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x81`$\x81\x85Z\xFA\x90\x81\x15a\t\x9DW_\x91a\t\xE5W[P3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\t\xD6Wa\x04\xACa$WV[\x84\x15a\t\xC8W\x83\x15\x80\x15a\t\xC0W[\x80\x15a\t\xB7W[a\t\xA8W` `$\x91`@Q\x92\x83\x80\x92c&,\x83\xF3`\xE1\x1B\x82R\x8C`\x04\x83\x01RZ\xFA\x90\x81\x15a\t\x9DW_\x91a\tcW[P\x15a\tTWa\x05\x03`\x06Ta$+V[\x95\x86`\x06U`@Q\x93a\x05\x15\x85a\x1E\xB3V[\x88\x85R` \x85\x01\x93\x87\x85R`@\x86\x01\x91\x87\x83R``\x87\x01\x90\x81R`\x80\x87\x01\x91\x82R`\xA0\x87\x01\x92`\x01\x84R`\xC0\x88\x01\x94\x85R`\xE0\x88\x01\x95\x86Ra\x01\0\x88\x01\x96B\x88R\x8B_R`\x02` R`@_ \x98Q\x89U`\x01\x89\x01\x90Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x05\x87\x83Ta\x1E{V[`\x1F\x81\x11a\t\x0FW[P` \x90`\x1F\x83\x11`\x01\x14a\x08\xA4Wa\x05\xEF\x97\x96\x95\x94\x93\x92\x91_\x91\x83a\x08\x99W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q`\x02\x89\x01UQ`\x03\x88\x01UQ`\x04\x87\x01UQ\x15\x15`\x05\x86\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[Q\x80Q`\x06\x85\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7W\x81\x90a\x06\x11\x84Ta\x1E{V[`\x1F\x81\x11a\x08IW[P` \x90`\x1F\x83\x11`\x01\x14a\x07\xE6W_\x92a\x07\xDBW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x80Q`\x07\x84\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x06d\x83Ta\x1E{V[`\x1F\x81\x11a\x07\x82W[P` \x90`\x1F\x83\x11`\x01\x14a\x06\xF4W\x92\x82`\x08\x93a\x06\xDE\x98\x96\x93\x7F\x8D\x9B\\O\xBE\"\x9F\x11F<\x06T\xF8)n\xD2\x83\x1A\x0C\xE7y\x8E!\xE8`y]\x14\x06\xC1g\xDD\x9A\x98\x96_\x92a\x06\xE9W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x91\x01U`@Q\x92\x83\x92`@\x84R`@\x84\x01\x90a\x1F\xABV[\x90` \x83\x01R\x03\x90\xA3\0[\x01Q\x90P\x8C\x80a\x06\xB2V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x07jWP\x93a\x06\xDE\x98\x96\x93\x7F\x8D\x9B\\O\xBE\"\x9F\x11F<\x06T\xF8)n\xD2\x83\x1A\x0C\xE7y\x8E!\xE8`y]\x14\x06\xC1g\xDD\x9A\x98\x96\x93`\x01\x93\x83`\x08\x98\x10a\x07RW[PPP\x81\x1B\x01\x90Ua\x06\xC6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8C\x80\x80a\x07EV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x04V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x07\xBDW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\xB2WPa\x06mV[_\x81U`\x01\x01a\x07\xA5V[\x90\x91P\x81\x90a\x07\x9CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P\x8A\x80a\x060V[_\x85\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x081WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x08\x19W[PPP\x81\x1B\x01\x90Ua\x06DV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8A\x80\x80a\x08\x0CV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\xF6V[\x90\x91P\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x08\x8FW[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08\x81WPa\x06\x1AV[_\x81U\x84\x93P`\x01\x01a\x08tV[\x90\x91P\x81\x90a\x08fV[\x01Q\x90P_\x80a\x05\xB1V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x08\xF7WP\x91`\x01\x93\x91\x85a\x05\xEF\x9B\x9A\x99\x98\x97\x96\x94\x10a\x08\xDFW[PPP\x81\x1B\x01\x90Ua\x05\xC5V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08\xD2V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x08\xB4V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\tJW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\t?WPa\x05\x90V[_\x81U`\x01\x01a\t2V[\x90\x91P\x81\x90a\t)V[c,\xDDa\x87`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\t\x95W[\x81a\t~` \x93\x83a\x1E\xEAV[\x81\x01\x03\x12a\x02\xEAWa\t\x8F\x90a!\x97V[\x88a\x04\xF2V[=\x91Pa\tqV[`@Q=_\x82>=\x90\xFD[cv\x16d\x01`\xE0\x1B_R`\x04_\xFD[P\x82\x84\x11a\x04\xC2V[P\x82\x15a\x04\xBBV[b\xBF\xC9!`\xE0\x1B_R`\x04_\xFD[c\x01\x96\x96a`\xE5\x1B_R`\x04_\xFD[a\n\x01\x91P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x81\x01\x90a!\xA4V[\x96PPPPPPP\x89a\x04\x92V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\n'a$rV[_\x80\x80\x80`\x01\x80`\xA0\x1B\x03`\x01T`\x08\x1C\x16G\x90Z\xF1a\nEa\">V[P\x15a\nMW\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FPlatform fee withdrawal failed\0\0`D\x82\x01R`d\x90\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x05` R`\xE0`@_ \x80T\x90`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16\x90`\x02\x81\x01T`\x03\x82\x01T`\x04\x83\x01T\x91`\xFF`\x06`\x05\x86\x01T\x95\x01T\x16\x94`@Q\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R\x15\x15`\xC0\x82\x01R\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x03` R`@_ \x80T`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x91a\x0BM` \x83`\x05\x1B\x01\x84a\x1E\xEAV[\x81\x83R` \x83\x01\x90_R` _ _\x91[\x83\x83\x10a\x0BwW`@Q\x80a\x0Bs\x87\x82a rV[\x03\x90\xF3[`\x07` `\x01\x92`@Qa\x0B\x8A\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x0B\xD6`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0B^V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW_`\xC0`@Qa\x0CM\x81a\x1E\xCFV[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\x045_R`\x05` R`\xE0`@_ `@Qa\x0C\x8C\x81a\x1E\xCFV[\x81T\x91\x82\x82R`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16` \x83\x01\x90\x81R`\x02\x82\x01T`@\x84\x01\x90\x81R`\x03\x83\x01T\x90``\x85\x01\x91\x82R`\x04\x84\x01T\x92`\x80\x86\x01\x93\x84R`\xC0`\xFF`\x06`\x05\x88\x01T\x97`\xA0\x8A\x01\x98\x89R\x01T\x16\x96\x01\x95\x15\x15\x86R`@Q\x96\x87R`\x01\x80`\xA0\x1B\x03\x90Q\x16` \x87\x01RQ`@\x86\x01RQ``\x85\x01RQ`\x80\x84\x01RQ`\xA0\x83\x01RQ\x15\x15`\xC0\x82\x01R\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045a\r<a$rV[a\x03\xE8\x81\x11a\rvW` \x81\x7FEa\rX\x11E\x92M\xD7\t\nP\x17\xE5\xF2\xB1\xD6\xF4\"\x13\xBB.\x95p\x7F\xF8hF\xBB\xFC\xB1\xCA\x92`\x08U`@Q\x90\x81R\xA1\0[c\n\xDA\xD23`\xE3\x1B_R`\x04_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW`\x01T`@Q`\x08\x91\x90\x91\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[a\r\xBA6a\x1E\"V[\x90\x80\x15\x80\x15a\x0FjW[a\x02\xD0Wa\r\xD0a$9V[a\r\xD8a$WV[\x80_R`\x02` R`@_ `@Qa\r\xF0\x81a\x1E\xB3V[\x81T\x81Ra\x0E\0`\x01\x83\x01a\x1F\x0BV[` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T`\x80\x82\x01Ra\x01\0`\x08`\xFF`\x05\x85\x01T\x16\x93`\xA0\x84\x01\x94\x15\x15\x85Ra\x0EF`\x06\x82\x01a\x1F\x0BV[`\xC0\x85\x01Ra\x0EW`\x07\x82\x01a\x1F\x0BV[`\xE0\x85\x01R\x01T\x91\x01RQ\x15a\tTW4\x15a\x0F[Wa\x0Ex`\x07Ta$+V[\x80`\x07Ua\x0F)`@Qa\x0E\x8B\x81a\x1E\xCFV[\x83\x81R`\x06` \x82\x013\x81R`@\x83\x01\x87\x81R``\x84\x014\x81R`\x80\x85\x01\x90B\x82R`\xA0\x86\x01\x92B\x84R`\xC0\x87\x01\x94`\x01\x86R\x89_R`\x05` R`@_ \x97Q\x88U`\x01\x80`\xA0\x1B\x03\x90Q\x16`\x01\x88\x01\x90`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90UQ`\x02\x87\x01UQ`\x03\x86\x01UQ`\x04\x85\x01UQ`\x05\x84\x01UQ\x15\x15\x91\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`@Q\x92\x83R\x7F\x0F\x03r\xA6\x1D\x9A\xC0_}Of\x01NP\xC4\xC3\x15\xF3\xA2s[\xF4s~\xD1\xBC\xDF\xF0\xB4\xF9\x9Bb` 3\x94\xA4`\x01_U\0[c\xCD\x1C\x88g`\xE0\x1B_R`\x04_\xFD[P`\x06T\x81\x11a\r\xC4V[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x01`\x01`\xA0\x1B\x03a\x0F\x96a\x1E8V[\x16_R`\x04` R`@_ \x80T`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x91a\x0F\xC6` \x83`\x05\x1B\x01\x84a\x1E\xEAV[\x81\x83R` \x83\x01\x90_R` _ _\x91[\x83\x83\x10a\x0F\xECW`@Q\x80a\x0Bs\x87\x82a rV[`\x07` `\x01\x92`@Qa\x0F\xFF\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x10K`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\xD7V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x10va$rV[a\x10~a$WV[`\x01`\xFF\x19\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x02` R`@_ \x80Ta\x10\xE3`\x01\x83\x01a\x1F\x0BV[\x91a\x11r`\x02\x82\x01T\x91a\x11d`\x03\x82\x01T\x93`\x04\x83\x01T`\xFF`\x05\x85\x01T\x16a\x11\x0F`\x06\x86\x01a\x1F\x0BV[\x91`\x08a\x11\x1E`\x07\x88\x01a\x1F\x0BV[\x96\x01T\x97a\x11@`@Q\x9B\x8C\x9B\x8CRa\x01 ` \x8D\x01Ra\x01 \x8C\x01\x90a\x1F\xABV[\x94`@\x8B\x01R``\x8A\x01R`\x80\x89\x01R\x15\x15`\xA0\x88\x01R\x86\x82\x03`\xC0\x88\x01Ra\x1F\xABV[\x90\x84\x82\x03`\xE0\x86\x01Ra\x1F\xABV[\x90a\x01\0\x83\x01R\x03\x90\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x11\x95a$rV[`\x01\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x08\x1C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xEAW`\xA06`\x03\x19\x01\x12a\x02\xEAW`\x045`$5`D5\x80\x15\x15\x90\x81\x81\x03a\x02\xEAW`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x12 \x906\x90`\x04\x01a ,V[\x90`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x12@\x906\x90`\x04\x01a ,V[\x91\x85\x15\x80\x15a\x15\x88W[a\x02\xD0Wa\x12Va$WV[_\x86\x81R`\x02` R`@\x80\x82 \x80T\x91Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x93\x91\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\x9DW_\x91a\x15fW[P3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\t\xD6Wa\x12\xEB\x90\x86a\x15[W[`\x05\x84\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[\x80Q`\x06\x83\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x13\n\x83Ta\x1E{V[`\x1F\x81\x11a\x15\x16W[P` \x90`\x1F\x83\x11`\x01\x14a\x14\xAFW`\x07\x94\x93\x92\x91_\x91\x83a\x14\xA4W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[\x01\x90\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x13a\x83Ta\x1E{V[`\x1F\x81\x11a\x14_W[P` \x90`\x1F\x83\x11`\x01\x14a\x13\xD6W\x91\x80\x7F\x1A,\xDA\x8B\x0Bg\xF9\x08\xE9\xA6\xFF\x05\xCBN9s\xDE \x13\x11QQ\x83x<\t\x86\xAE\x97\x1F;\x13\x96\x94\x92`@\x96\x94_\x92a\x13\xCBW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[\x82Q\x91\x82R` \x82\x01R\xA2\0[\x01Q\x90P\x88\x80a\x13\xAAV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x14GWP\x92`\x01\x92\x85\x92`@\x98\x96\x7F\x1A,\xDA\x8B\x0Bg\xF9\x08\xE9\xA6\xFF\x05\xCBN9s\xDE \x13\x11QQ\x83x<\t\x86\xAE\x97\x1F;\x13\x9A\x98\x96\x10a\x14/W[PPP\x81\x1B\x01\x90Ua\x13\xBEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x88\x80\x80a\x14\"V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x13\xE6V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x14\x9AW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x14\x8FWPa\x13jV[_\x81U`\x01\x01a\x14\x82V[\x90\x91P\x81\x90a\x14yV[\x01Q\x90P\x89\x80a\x130V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x14\xFEWP\x91`\x01\x93\x91\x85`\x07\x98\x97\x96\x94\x10a\x14\xE6W[PPP\x81\x1B\x01\x90Ua\x13DV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x89\x80\x80a\x14\xD9V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x14\xBFV[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x15QW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x15FWPa\x13\x13V[_\x81U`\x01\x01a\x159V[\x90\x91P\x81\x90a\x150V[\x86`\x02\x85\x01Ua\x12\xD6V[a\x15z\x91P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x96PPPPPPP\x88a\x12\xBAV[P`\x06T\x86\x11a\x12JV[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x06T`@Q\x90\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x15\xEAa$rV[`\x01T`\xFF\x81\x16\x15a\x16)W`\xFF\x19\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[c\x8D\xFC +`\xE0\x1B_R`\x04_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `@Qa\x03\xE8\x81R\xF3[4a\x02\xEAWa\x16b6a\x1E\"V[\x90_R`\x03` R`@_ \x90\x81T\x81\x10\x15a\x02\xEAWa\x16\x81\x91a\x1ENV[P\x80Ta\x0Bs`\x01\x80`\xA0\x1B\x03`\x01\x84\x01T\x16\x92`\x02\x81\x01T\x90`\x03\x81\x01T`\x04\x82\x01T\x90a\x16\xBA`\x06`\xFF`\x05\x86\x01T\x16\x94\x01a\x1F\x0BV[\x93`@Q\x97\x88\x97\x88a\x1F\xCFV[4a\x02\xEAW`@6`\x03\x19\x01\x12a\x02\xEAWa\x16\xE0a\x1E8V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 \x80T`$5\x90\x81\x10\x15a\x02\xEAWa\x16\x81\x91a\x1ENV[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045a\x17(a$9V[a\x170a$WV[\x80_R`\x05` R`@_ \x90`\x06\x82\x01\x80T`\xFF\x81\x16\x15a\x19aW\x83T_R`\x02` R`@_ `\x08`@Q\x91a\x17h\x83a\x1E\xB3V[\x80T\x83Ra\x17x`\x01\x82\x01a\x1F\x0BV[` \x84\x01R`\x02\x81\x01T`@\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\xFF`\x05\x82\x01T\x16\x15\x15`\xA0\x84\x01Ra\x17\xB6`\x06\x82\x01a\x1F\x0BV[`\xC0\x84\x01Ra\x17\xC7`\x07\x82\x01a\x1F\x0BV[`\xE0\x84\x01R\x01Ta\x01\0\x82\x01RQ`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x91_\x83`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\x9DW_\x93_\x91a\x19;W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xD6W`\x04\x85\x01\x91`\x03a\x18[a\x18P\x85TBa!0V[`\x02\x89\x01T\x90a!\x1DV[\x96\x01\x91\x82T\x91\x82\x88\x11a\x19)W[PP\x85a\x18u\x91a!0V[\x90UB\x90U_\x80\x80\x80a\x18\x97a'\x10a\x18\x90`\x08T\x8Aa!\x1DV[\x04\x88a!0V[\x85Z\xF1a\x18\xA2a\">V[P\x15a\x18\xE4W`@Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\xC9V\xD4\x8B\x8B\xAA\x879uO\xF3\xD8\xEE%\xB8\xD8\x83\xD2H9a:\xD4\xD2\x85m\xF5pf\xEBe\xA8\x90` \x90\xA3`\x01_U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FStreaming payment failed\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\xFF\x19\x16\x90U\x94P\x84\x80a\x18ua\x18iV[\x90Pa\x19R\x91\x93P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x98\x96PPPPPP\x92\x86a\x18+V[cX\xA0\xD9Y`\xE1\x1B_R`\x04_\xFD[4a\x02\xEAW`@6`\x03\x19\x01\x12a\x02\xEAWa\x19\x89a\x1E8V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 \x80T`$5\x90`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x92a\x19\xC7` \x83`\x05\x1B\x01\x85a\x1E\xEAV[\x81\x84R` \x84\x01\x90_R` _ _\x91[\x83\x83\x10a\x1A\x8BW\x85_\x80\x87\x81[\x84Q\x81\x10\x15a\x1AyW\x81a\x19\xF9\x82\x87a$\x17V[QQ\x14\x80a\x1AbW[\x80a\x1AJW[a\x1A\x15W[`\x01\x01a\x19\xE5V[`\x01\x93P\x82``a\x1A&\x83\x88a$\x17V[Q\x01Q\x11\x15a\x1A\rW\x91P`\x01``a\x1A?\x84\x87a$\x17V[Q\x01Q\x92\x90Pa\x1A\rV[P``a\x1AW\x82\x87a$\x17V[Q\x01QB\x11\x15a\x1A\x08V[P`\xA0a\x1Ao\x82\x87a$\x17V[Q\x01Q\x15\x15a\x1A\x02V[`@\x84\x84\x82Q\x91\x15\x15\x82R` \x82\x01R\xF3[`\x07` `\x01\x92`@Qa\x1A\x9E\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x1A\xEA`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x19\xD8V[a\x1B\x066a\x1E\"V[\x90\x80\x15\x80\x15a\x1E\x17W[a\x02\xD0Wa\x1B\x1Ca$9V[a\x1B$a$WV[\x80_R`\x02` R`@_ `@Q\x90a\x1B=\x82a\x1E\xB3V[\x80T\x82Ra\x1BM`\x01\x82\x01a\x1F\x0BV[` \x83\x01R`\x02\x81\x01T\x90`@\x83\x01\x91\x82R`\x03\x81\x01T``\x84\x01\x90\x81R`\x04\x82\x01T\x91`\x80\x85\x01\x92\x83R`\x08`\xFF`\x05\x83\x01T\x16\x91`\xA0\x87\x01\x92\x15\x15\x83Ra\x1B\x98`\x06\x82\x01a\x1F\x0BV[`\xC0\x88\x01Ra\x1B\xA9`\x07\x82\x01a\x1F\x0BV[`\xE0\x88\x01R\x01Ta\x01\0\x86\x01RQ\x15a\tTWQ\x85\x10\x90\x81\x15a\x1E\x0CW[Pa\t\xA8Wa\x0E\x0F\x84\x01\x90\x81\x85\x11a\x1D\x91Wa\x0E\x10a\x1B\xE8\x92\x04\x90Qa!\x1DV[\x90\x814\x10a\x0F[Wa\x1C\ta'\x10a\x1C\x02`\x08T\x85a!\x1DV[\x04\x83a!0V[\x90Q`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90_\x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\x9DW_\x92\x83\x92\x83\x92\x83\x92\x83\x91a\x1D\xEAW[PZ\xF1a\x1Cra\">V[P\x15a\x1D\xA5W\x81_R`\x03` R`@_ T\x90\x83B\x01\x80B\x11a\x1D\x91W`@Q\x90a\x1C\x9D\x82a\x1E\xCFV[\x84\x82R3` \x83\x01RB`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R`\x01`\xA0\x82\x01Ra\x1C\xFD` \x91`@Qa\x1C\xD1\x84\x82a\x1E\xEAV[_\x81R`\xC0\x82\x01R\x85_R`\x03\x83Ra\x1C\xED\x81`@_ a\"mV[3_R`\x04\x83R`@_ a\"mV[\x814\x11a\x1D?W[`@Q\x94\x85R\x84\x01R`@\x83\x01R\x7F/\\\xBC\x8BX\x94\x9Cm\x94Y-r7\xAA\x1A'\xE9\xE6\xAC\xDC\xB1\xB8*J\x1D\x0B'\xDD\xFD\xAF\xE5\xEA``3\x93\xA3`\x01_U\0[_\x80\x80\x80a\x1DM\x864a!0V[3Z\xF1a\x1DXa\">V[Pa\x1D\x05W`d\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`\r`$\x82\x01Rl\x14\x99Y\x9D[\x99\x08\x19\x98Z[\x19Y`\x9A\x1B`D\x82\x01R\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FPayment to vehicle failed\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[a\x1D\xFE\x91P=\x80\x85\x83>a\t\xF9\x81\x83a\x1E\xEAV[PPPPPP\x90P\x89a\x1CgV[\x90PQ\x84\x11\x85a\x1B\xC7V[P`\x06T\x81\x11a\x1B\x10V[`@\x90`\x03\x19\x01\x12a\x02\xEAW`\x045\x90`$5\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xEAWV[\x80T\x82\x10\x15a\x1EgW_R`\x07` _ \x91\x02\x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x1E\xA9W[` \x83\x10\x14a\x1E\x95WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x1E\x8AV[a\x01 \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[\x90`@Q\x91\x82_\x82T\x92a\x1F\x1E\x84a\x1E{V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x1F\x89WP`\x01\x14a\x1FEW[Pa\x1FC\x92P\x03\x83a\x1E\xEAV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x1FmWPP\x90` a\x1FC\x92\x82\x01\x01_a\x1F6V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x1FTV[\x90P` \x92Pa\x1FC\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x1F6V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x93\x90\x92`\xE0\x95\x92a \x0E\x98\x97\x94\x86R`\x01\x80`\xA0\x1B\x03\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R\x15\x15`\xA0\x82\x01R\x81`\xC0\x82\x01R\x01\x90a\x1F\xABV[\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x02\xEAW\x805\x90a C\x82a \x11V[\x92a Q`@Q\x94\x85a\x1E\xEAV[\x82\x84R` \x83\x83\x01\x01\x11a\x02\xEAW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a \xA4WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a!\x0E`\x01\x93`?\x19\x86\x82\x03\x01\x87R`\xE0`\xC0\x8BQ\x80Q\x84R\x87\x80`\xA0\x1B\x03\x86\x82\x01Q\x16\x86\x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q\x15\x15`\xA0\x85\x01R\x01Q\x91\x81`\xC0\x82\x01R\x01\x90a\x1F\xABV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a \x95V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1D\x91WV[\x91\x90\x82\x03\x91\x82\x11a\x1D\x91WV[\x81`\x1F\x82\x01\x12\x15a\x02\xEAW\x80Q\x90a!T\x82a \x11V[\x92a!b`@Q\x94\x85a\x1E\xEAV[\x82\x84R` \x83\x83\x01\x01\x11a\x02\xEAW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xEAWV[Q\x90\x81\x15\x15\x82\x03a\x02\xEAWV[\x90a\x01\0\x82\x82\x03\x12a\x02\xEAW\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAW\x81a!\xCC\x91\x84\x01a!=V[\x92a!\xD9` \x84\x01a!\x83V[\x92`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAW\x83a!\xF9\x91\x83\x01a!=V[\x92``\x82\x01Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xEAWa\"\x19\x91\x83\x01a!=V[\x91`\x80\x82\x01Q\x91a\",`\xA0\x82\x01a!\x97V[\x91a \x0E`\xE0`\xC0\x84\x01Q\x93\x01a!\x83V[=\x15a\"hW=\x90a\"O\x82a \x11V[\x91a\"]`@Q\x93\x84a\x1E\xEAV[\x82R=_` \x84\x01>V[``\x90V[\x91\x90\x91\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xC7Wa\"\x92\x91`\x01\x82\x01\x81Ua\x1ENV[a$\x04W\x82Q\x81U` \x83\x01Q`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@\x83\x01Q`\x02\x82\x01U``\x83\x01Q`\x03\x82\x01U`\x80\x83\x01Q`\x04\x82\x01U`\xA0\x83\x01Q`\x05\x82\x01\x80T`\xFF\x19\x16\x91\x15\x15`\xFF\x16\x91\x90\x91\x17\x90U`\xC0\x90`\x06\x01\x92\x01Q\x91\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7Wa#\x1F\x82Ta\x1E{V[`\x1F\x81\x11a#\xBFW[P` `\x1F\x82\x11`\x01\x14a#aW\x81\x92\x93\x94_\x92a#VW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01Q\x90P_\x80a#AV[`\x1F\x19\x82\x16\x90\x83_R\x80_ \x91_[\x81\x81\x10a#\xA7WP\x95\x83`\x01\x95\x96\x97\x10a#\x8FW[PPP\x81\x1B\x01\x90UV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a#\x85V[\x91\x92` `\x01\x81\x92\x86\x8B\x01Q\x81U\x01\x94\x01\x92\x01a#pV[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a#\xFAW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a#\xEFWPa#(V[_\x81U`\x01\x01a#\xE2V[\x90\x91P\x81\x90a#\xD9V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1EgW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x1D\x91W`\x01\x01\x90V[`\x02_T\x14a$HW`\x02_UV[c>\xE5\xAE\xB5`\xE0\x1B_R`\x04_\xFD[`\xFF`\x01T\x16a$cWV[c\xD9<\x06e`\xE0\x1B_R`\x04_\xFD[`\x01T`\x08\x1C`\x01`\x01`\xA0\x1B\x03\x163\x03a$\x89WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 w\xA0\xD6\xC6\x80\x96CH\xF1;\x84g\xDBh\x16\xCC\x81\xD6\xE6BU-u\x16R\x106\x08n\xA4\xB9\xDFdsolcC\0\x08\x1E\x003";
    /// The bytecode of the contract.
    pub static DATAMARKETPLACE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0E\x91\x91Y\x14a\x1A\xFDW\x80c\x14\xFF&n\x14a\x03\xA4W\x80c\x1F\x16e;\x14a\x15\xB5W\x80c!S\xE3\xEE\x14a\x19pW\x80c#\x0C\x8D\xD3\x14a\x17\x0CW\x80c4\xB7\xBF\xEF\x14a\x16\xC7W\x80c8\xBB\xD4\xE1\x14a\x16TW\x80c9\x98\xA6\x81\x14a\x168W\x80c?K\xA8:\x14a\x15\xD2W\x80cW\n\xEA\x8A\x14a\x15\xB5W\x80c\\\x97Z\xBB\x14a\x15\x93W\x80c_zim\x14a\x11\xDCW\x80cqP\x18\xA6\x14a\x11}W\x80cvp\n\x8B\x14a\x10\xB6W\x80c\x84V\xCBY\x14a\x10^W\x80c\x85\xA3C\x93\x14a\x0FuW\x80c\x86kt\x80\x14a\r\xB1W\x80c\x8D\xA5\xCB[\x14a\r\x85W\x80c\x92\x7F\xEF.\x14a\r W\x80c\x96\x96HH\x14a\x0C-W\x80c\x9DV\xF0\x85\x14a\x0B\xE9W\x80c\xB8\xB6P\xBF\x14a\x0B\nW\x80c\xCDV\xF0\xAC\x14a\n\x92W\x80c\xD0\xB7\x83\x0B\x14a\n\x0FW\x80c\xD4\xD8\xDF\xFA\x14a\x03\xC1W\x80c\xDC3\t\xAD\x14a\x03\xA4W\x80c\xEE\xCA\x08\xF0\x14a\x03\x87W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xEEWc\xFC\xC3&p\x14a\x01XW_\x80\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_a\x01\0`@Qa\x01|\x81a\x1E\xB3V[\x82\x81R``` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R```\xC0\x82\x01R```\xE0\x82\x01R\x01R\x80\x15\x80\x15a\x02\xDFW[a\x02\xD0W_R`\x02` R`@_ `@Qa\x01\xD2\x81a\x1E\xB3V[\x81T\x81Ra\x01\xE2`\x01\x83\x01a\x1F\x0BV[\x91` \x82\x01\x92\x83Ra\x02\xC4`\x02\x82\x01T\x91`@\x84\x01\x92\x83Ra\x02\xB0`\x03\x82\x01T\x93``\x86\x01\x94\x85R`\x04\x83\x01T`\x80\x87\x01\x90\x81R`\xFF`\x05\x85\x01T\x16`\xA0\x88\x01\x90\x15\x15\x81Ra\x023`\x06\x86\x01a\x1F\x0BV[\x91`\xC0\x89\x01\x92\x83R`\x08a\x02I`\x07\x88\x01a\x1F\x0BV[\x96`\xE0\x8B\x01\x97\x88R\x01T\x97a\x01\0\x8A\x01\x98\x89Ra\x02\x83`@Q\x9B\x8C\x9B` \x8DRQ` \x8D\x01RQa\x01 `@\x8D\x01Ra\x01@\x8C\x01\x90a\x1F\xABV[\x94Q``\x8B\x01RQ`\x80\x8A\x01RQ`\xA0\x89\x01RQ\x15\x15`\xC0\x88\x01RQ\x86\x82\x03`\x1F\x19\x01`\xE0\x88\x01Ra\x1F\xABV[\x90Q\x84\x82\x03`\x1F\x19\x01a\x01\0\x86\x01Ra\x1F\xABV[\x90Qa\x01 \x83\x01R\x03\x90\xF3[cy\xDEJ\xF5`\xE0\x1B_R`\x04_\xFD[P`\x06T\x81\x11a\x01\xB7V[_\x80\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAWa\x03\x07a\x1E8V[a\x03\x0Fa$rV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x03tW`\x01\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16`\x08\x93\x84\x1Ba\x01\0`\x01`\xA8\x1B\x03\x16\x17\x90\x91U\x90\x1C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x08T`@Q\x90\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x07T`@Q\x90\x81R\xF3[4a\x02\xEAW`\xE06`\x03\x19\x01\x12a\x02\xEAW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x03\xF4\x906\x90`\x04\x01a ,V[`\x845\x90`D5`d5`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x04\x1D\x906\x90`\x04\x01a ,V[\x93`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x04=\x906\x90`\x04\x01a ,V[`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x81`$\x81\x85Z\xFA\x90\x81\x15a\t\x9DW_\x91a\t\xE5W[P3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\t\xD6Wa\x04\xACa$WV[\x84\x15a\t\xC8W\x83\x15\x80\x15a\t\xC0W[\x80\x15a\t\xB7W[a\t\xA8W` `$\x91`@Q\x92\x83\x80\x92c&,\x83\xF3`\xE1\x1B\x82R\x8C`\x04\x83\x01RZ\xFA\x90\x81\x15a\t\x9DW_\x91a\tcW[P\x15a\tTWa\x05\x03`\x06Ta$+V[\x95\x86`\x06U`@Q\x93a\x05\x15\x85a\x1E\xB3V[\x88\x85R` \x85\x01\x93\x87\x85R`@\x86\x01\x91\x87\x83R``\x87\x01\x90\x81R`\x80\x87\x01\x91\x82R`\xA0\x87\x01\x92`\x01\x84R`\xC0\x88\x01\x94\x85R`\xE0\x88\x01\x95\x86Ra\x01\0\x88\x01\x96B\x88R\x8B_R`\x02` R`@_ \x98Q\x89U`\x01\x89\x01\x90Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x05\x87\x83Ta\x1E{V[`\x1F\x81\x11a\t\x0FW[P` \x90`\x1F\x83\x11`\x01\x14a\x08\xA4Wa\x05\xEF\x97\x96\x95\x94\x93\x92\x91_\x91\x83a\x08\x99W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q`\x02\x89\x01UQ`\x03\x88\x01UQ`\x04\x87\x01UQ\x15\x15`\x05\x86\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[Q\x80Q`\x06\x85\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7W\x81\x90a\x06\x11\x84Ta\x1E{V[`\x1F\x81\x11a\x08IW[P` \x90`\x1F\x83\x11`\x01\x14a\x07\xE6W_\x92a\x07\xDBW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x80Q`\x07\x84\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x06d\x83Ta\x1E{V[`\x1F\x81\x11a\x07\x82W[P` \x90`\x1F\x83\x11`\x01\x14a\x06\xF4W\x92\x82`\x08\x93a\x06\xDE\x98\x96\x93\x7F\x8D\x9B\\O\xBE\"\x9F\x11F<\x06T\xF8)n\xD2\x83\x1A\x0C\xE7y\x8E!\xE8`y]\x14\x06\xC1g\xDD\x9A\x98\x96_\x92a\x06\xE9W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[Q\x91\x01U`@Q\x92\x83\x92`@\x84R`@\x84\x01\x90a\x1F\xABV[\x90` \x83\x01R\x03\x90\xA3\0[\x01Q\x90P\x8C\x80a\x06\xB2V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x07jWP\x93a\x06\xDE\x98\x96\x93\x7F\x8D\x9B\\O\xBE\"\x9F\x11F<\x06T\xF8)n\xD2\x83\x1A\x0C\xE7y\x8E!\xE8`y]\x14\x06\xC1g\xDD\x9A\x98\x96\x93`\x01\x93\x83`\x08\x98\x10a\x07RW[PPP\x81\x1B\x01\x90Ua\x06\xC6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8C\x80\x80a\x07EV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x04V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x07\xBDW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\xB2WPa\x06mV[_\x81U`\x01\x01a\x07\xA5V[\x90\x91P\x81\x90a\x07\x9CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P\x8A\x80a\x060V[_\x85\x81R\x82\x81 \x93P`\x1F\x19\x85\x16\x90[\x81\x81\x10a\x081WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x08\x19W[PPP\x81\x1B\x01\x90Ua\x06DV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8A\x80\x80a\x08\x0CV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\xF6V[\x90\x91P\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x08\x8FW[\x90`\x1F\x85\x94\x93\x92\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08\x81WPa\x06\x1AV[_\x81U\x84\x93P`\x01\x01a\x08tV[\x90\x91P\x81\x90a\x08fV[\x01Q\x90P_\x80a\x05\xB1V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x08\xF7WP\x91`\x01\x93\x91\x85a\x05\xEF\x9B\x9A\x99\x98\x97\x96\x94\x10a\x08\xDFW[PPP\x81\x1B\x01\x90Ua\x05\xC5V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x08\xD2V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x08\xB4V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\tJW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\t?WPa\x05\x90V[_\x81U`\x01\x01a\t2V[\x90\x91P\x81\x90a\t)V[c,\xDDa\x87`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\t\x95W[\x81a\t~` \x93\x83a\x1E\xEAV[\x81\x01\x03\x12a\x02\xEAWa\t\x8F\x90a!\x97V[\x88a\x04\xF2V[=\x91Pa\tqV[`@Q=_\x82>=\x90\xFD[cv\x16d\x01`\xE0\x1B_R`\x04_\xFD[P\x82\x84\x11a\x04\xC2V[P\x82\x15a\x04\xBBV[b\xBF\xC9!`\xE0\x1B_R`\x04_\xFD[c\x01\x96\x96a`\xE5\x1B_R`\x04_\xFD[a\n\x01\x91P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x81\x01\x90a!\xA4V[\x96PPPPPPP\x89a\x04\x92V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\n'a$rV[_\x80\x80\x80`\x01\x80`\xA0\x1B\x03`\x01T`\x08\x1C\x16G\x90Z\xF1a\nEa\">V[P\x15a\nMW\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FPlatform fee withdrawal failed\0\0`D\x82\x01R`d\x90\xFD[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x05` R`\xE0`@_ \x80T\x90`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16\x90`\x02\x81\x01T`\x03\x82\x01T`\x04\x83\x01T\x91`\xFF`\x06`\x05\x86\x01T\x95\x01T\x16\x94`@Q\x96\x87R` \x87\x01R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R\x15\x15`\xC0\x82\x01R\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x03` R`@_ \x80T`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x91a\x0BM` \x83`\x05\x1B\x01\x84a\x1E\xEAV[\x81\x83R` \x83\x01\x90_R` _ _\x91[\x83\x83\x10a\x0BwW`@Q\x80a\x0Bs\x87\x82a rV[\x03\x90\xF3[`\x07` `\x01\x92`@Qa\x0B\x8A\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x0B\xD6`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0B^V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW_`\xC0`@Qa\x0CM\x81a\x1E\xCFV[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\x045_R`\x05` R`\xE0`@_ `@Qa\x0C\x8C\x81a\x1E\xCFV[\x81T\x91\x82\x82R`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16` \x83\x01\x90\x81R`\x02\x82\x01T`@\x84\x01\x90\x81R`\x03\x83\x01T\x90``\x85\x01\x91\x82R`\x04\x84\x01T\x92`\x80\x86\x01\x93\x84R`\xC0`\xFF`\x06`\x05\x88\x01T\x97`\xA0\x8A\x01\x98\x89R\x01T\x16\x96\x01\x95\x15\x15\x86R`@Q\x96\x87R`\x01\x80`\xA0\x1B\x03\x90Q\x16` \x87\x01RQ`@\x86\x01RQ``\x85\x01RQ`\x80\x84\x01RQ`\xA0\x83\x01RQ\x15\x15`\xC0\x82\x01R\xF3[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045a\r<a$rV[a\x03\xE8\x81\x11a\rvW` \x81\x7FEa\rX\x11E\x92M\xD7\t\nP\x17\xE5\xF2\xB1\xD6\xF4\"\x13\xBB.\x95p\x7F\xF8hF\xBB\xFC\xB1\xCA\x92`\x08U`@Q\x90\x81R\xA1\0[c\n\xDA\xD23`\xE3\x1B_R`\x04_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW`\x01T`@Q`\x08\x91\x90\x91\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[a\r\xBA6a\x1E\"V[\x90\x80\x15\x80\x15a\x0FjW[a\x02\xD0Wa\r\xD0a$9V[a\r\xD8a$WV[\x80_R`\x02` R`@_ `@Qa\r\xF0\x81a\x1E\xB3V[\x81T\x81Ra\x0E\0`\x01\x83\x01a\x1F\x0BV[` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T`\x80\x82\x01Ra\x01\0`\x08`\xFF`\x05\x85\x01T\x16\x93`\xA0\x84\x01\x94\x15\x15\x85Ra\x0EF`\x06\x82\x01a\x1F\x0BV[`\xC0\x85\x01Ra\x0EW`\x07\x82\x01a\x1F\x0BV[`\xE0\x85\x01R\x01T\x91\x01RQ\x15a\tTW4\x15a\x0F[Wa\x0Ex`\x07Ta$+V[\x80`\x07Ua\x0F)`@Qa\x0E\x8B\x81a\x1E\xCFV[\x83\x81R`\x06` \x82\x013\x81R`@\x83\x01\x87\x81R``\x84\x014\x81R`\x80\x85\x01\x90B\x82R`\xA0\x86\x01\x92B\x84R`\xC0\x87\x01\x94`\x01\x86R\x89_R`\x05` R`@_ \x97Q\x88U`\x01\x80`\xA0\x1B\x03\x90Q\x16`\x01\x88\x01\x90`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90UQ`\x02\x87\x01UQ`\x03\x86\x01UQ`\x04\x85\x01UQ`\x05\x84\x01UQ\x15\x15\x91\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[`@Q\x92\x83R\x7F\x0F\x03r\xA6\x1D\x9A\xC0_}Of\x01NP\xC4\xC3\x15\xF3\xA2s[\xF4s~\xD1\xBC\xDF\xF0\xB4\xF9\x9Bb` 3\x94\xA4`\x01_U\0[c\xCD\x1C\x88g`\xE0\x1B_R`\x04_\xFD[P`\x06T\x81\x11a\r\xC4V[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x01`\x01`\xA0\x1B\x03a\x0F\x96a\x1E8V[\x16_R`\x04` R`@_ \x80T`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x91a\x0F\xC6` \x83`\x05\x1B\x01\x84a\x1E\xEAV[\x81\x83R` \x83\x01\x90_R` _ _\x91[\x83\x83\x10a\x0F\xECW`@Q\x80a\x0Bs\x87\x82a rV[`\x07` `\x01\x92`@Qa\x0F\xFF\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x10K`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x0F\xD7V[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x10va$rV[a\x10~a$WV[`\x01`\xFF\x19\x81T\x16\x17`\x01U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045_R`\x02` R`@_ \x80Ta\x10\xE3`\x01\x83\x01a\x1F\x0BV[\x91a\x11r`\x02\x82\x01T\x91a\x11d`\x03\x82\x01T\x93`\x04\x83\x01T`\xFF`\x05\x85\x01T\x16a\x11\x0F`\x06\x86\x01a\x1F\x0BV[\x91`\x08a\x11\x1E`\x07\x88\x01a\x1F\x0BV[\x96\x01T\x97a\x11@`@Q\x9B\x8C\x9B\x8CRa\x01 ` \x8D\x01Ra\x01 \x8C\x01\x90a\x1F\xABV[\x94`@\x8B\x01R``\x8A\x01R`\x80\x89\x01R\x15\x15`\xA0\x88\x01R\x86\x82\x03`\xC0\x88\x01Ra\x1F\xABV[\x90\x84\x82\x03`\xE0\x86\x01Ra\x1F\xABV[\x90a\x01\0\x83\x01R\x03\x90\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x11\x95a$rV[`\x01\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x08\x1C`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x02\xEAW`\xA06`\x03\x19\x01\x12a\x02\xEAW`\x045`$5`D5\x80\x15\x15\x90\x81\x81\x03a\x02\xEAW`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x12 \x906\x90`\x04\x01a ,V[\x90`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAWa\x12@\x906\x90`\x04\x01a ,V[\x91\x85\x15\x80\x15a\x15\x88W[a\x02\xD0Wa\x12Va$WV[_\x86\x81R`\x02` R`@\x80\x82 \x80T\x91Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x93\x91\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\t\x9DW_\x91a\x15fW[P3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\t\xD6Wa\x12\xEB\x90\x86a\x15[W[`\x05\x84\x01\x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90UV[\x80Q`\x06\x83\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x13\n\x83Ta\x1E{V[`\x1F\x81\x11a\x15\x16W[P` \x90`\x1F\x83\x11`\x01\x14a\x14\xAFW`\x07\x94\x93\x92\x91_\x91\x83a\x14\xA4W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[\x01\x90\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07\xC7Wa\x13a\x83Ta\x1E{V[`\x1F\x81\x11a\x14_W[P` \x90`\x1F\x83\x11`\x01\x14a\x13\xD6W\x91\x80\x7F\x1A,\xDA\x8B\x0Bg\xF9\x08\xE9\xA6\xFF\x05\xCBN9s\xDE \x13\x11QQ\x83x<\t\x86\xAE\x97\x1F;\x13\x96\x94\x92`@\x96\x94_\x92a\x13\xCBW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[\x82Q\x91\x82R` \x82\x01R\xA2\0[\x01Q\x90P\x88\x80a\x13\xAAV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x14GWP\x92`\x01\x92\x85\x92`@\x98\x96\x7F\x1A,\xDA\x8B\x0Bg\xF9\x08\xE9\xA6\xFF\x05\xCBN9s\xDE \x13\x11QQ\x83x<\t\x86\xAE\x97\x1F;\x13\x9A\x98\x96\x10a\x14/W[PPP\x81\x1B\x01\x90Ua\x13\xBEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x88\x80\x80a\x14\"V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x13\xE6V[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x14\x9AW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x14\x8FWPa\x13jV[_\x81U`\x01\x01a\x14\x82V[\x90\x91P\x81\x90a\x14yV[\x01Q\x90P\x89\x80a\x130V[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x14\xFEWP\x91`\x01\x93\x91\x85`\x07\x98\x97\x96\x94\x10a\x14\xE6W[PPP\x81\x1B\x01\x90Ua\x13DV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x89\x80\x80a\x14\xD9V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x14\xBFV[\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x15QW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x15FWPa\x13\x13V[_\x81U`\x01\x01a\x159V[\x90\x91P\x81\x90a\x150V[\x86`\x02\x85\x01Ua\x12\xD6V[a\x15z\x91P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x96PPPPPPP\x88a\x12\xBAV[P`\x06T\x86\x11a\x12JV[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\xFF`\x01T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `\x06T`@Q\x90\x81R\xF3[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAWa\x15\xEAa$rV[`\x01T`\xFF\x81\x16\x15a\x16)W`\xFF\x19\x16`\x01U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[c\x8D\xFC +`\xE0\x1B_R`\x04_\xFD[4a\x02\xEAW_6`\x03\x19\x01\x12a\x02\xEAW` `@Qa\x03\xE8\x81R\xF3[4a\x02\xEAWa\x16b6a\x1E\"V[\x90_R`\x03` R`@_ \x90\x81T\x81\x10\x15a\x02\xEAWa\x16\x81\x91a\x1ENV[P\x80Ta\x0Bs`\x01\x80`\xA0\x1B\x03`\x01\x84\x01T\x16\x92`\x02\x81\x01T\x90`\x03\x81\x01T`\x04\x82\x01T\x90a\x16\xBA`\x06`\xFF`\x05\x86\x01T\x16\x94\x01a\x1F\x0BV[\x93`@Q\x97\x88\x97\x88a\x1F\xCFV[4a\x02\xEAW`@6`\x03\x19\x01\x12a\x02\xEAWa\x16\xE0a\x1E8V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 \x80T`$5\x90\x81\x10\x15a\x02\xEAWa\x16\x81\x91a\x1ENV[4a\x02\xEAW` 6`\x03\x19\x01\x12a\x02\xEAW`\x045a\x17(a$9V[a\x170a$WV[\x80_R`\x05` R`@_ \x90`\x06\x82\x01\x80T`\xFF\x81\x16\x15a\x19aW\x83T_R`\x02` R`@_ `\x08`@Q\x91a\x17h\x83a\x1E\xB3V[\x80T\x83Ra\x17x`\x01\x82\x01a\x1F\x0BV[` \x84\x01R`\x02\x81\x01T`@\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\xFF`\x05\x82\x01T\x16\x15\x15`\xA0\x84\x01Ra\x17\xB6`\x06\x82\x01a\x1F\x0BV[`\xC0\x84\x01Ra\x17\xC7`\x07\x82\x01a\x1F\x0BV[`\xE0\x84\x01R\x01Ta\x01\0\x82\x01RQ`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x91_\x83`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\x9DW_\x93_\x91a\x19;W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xD6W`\x04\x85\x01\x91`\x03a\x18[a\x18P\x85TBa!0V[`\x02\x89\x01T\x90a!\x1DV[\x96\x01\x91\x82T\x91\x82\x88\x11a\x19)W[PP\x85a\x18u\x91a!0V[\x90UB\x90U_\x80\x80\x80a\x18\x97a'\x10a\x18\x90`\x08T\x8Aa!\x1DV[\x04\x88a!0V[\x85Z\xF1a\x18\xA2a\">V[P\x15a\x18\xE4W`@Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x16\x91\x7F\xC9V\xD4\x8B\x8B\xAA\x879uO\xF3\xD8\xEE%\xB8\xD8\x83\xD2H9a:\xD4\xD2\x85m\xF5pf\xEBe\xA8\x90` \x90\xA3`\x01_U\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FStreaming payment failed\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\xFF\x19\x16\x90U\x94P\x84\x80a\x18ua\x18iV[\x90Pa\x19R\x91\x93P=\x80_\x83>a\t\xF9\x81\x83a\x1E\xEAV[\x98\x96PPPPPP\x92\x86a\x18+V[cX\xA0\xD9Y`\xE1\x1B_R`\x04_\xFD[4a\x02\xEAW`@6`\x03\x19\x01\x12a\x02\xEAWa\x19\x89a\x1E8V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 \x80T`$5\x90`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`@Q\x92a\x19\xC7` \x83`\x05\x1B\x01\x85a\x1E\xEAV[\x81\x84R` \x84\x01\x90_R` _ _\x91[\x83\x83\x10a\x1A\x8BW\x85_\x80\x87\x81[\x84Q\x81\x10\x15a\x1AyW\x81a\x19\xF9\x82\x87a$\x17V[QQ\x14\x80a\x1AbW[\x80a\x1AJW[a\x1A\x15W[`\x01\x01a\x19\xE5V[`\x01\x93P\x82``a\x1A&\x83\x88a$\x17V[Q\x01Q\x11\x15a\x1A\rW\x91P`\x01``a\x1A?\x84\x87a$\x17V[Q\x01Q\x92\x90Pa\x1A\rV[P``a\x1AW\x82\x87a$\x17V[Q\x01QB\x11\x15a\x1A\x08V[P`\xA0a\x1Ao\x82\x87a$\x17V[Q\x01Q\x15\x15a\x1A\x02V[`@\x84\x84\x82Q\x91\x15\x15\x82R` \x82\x01R\xF3[`\x07` `\x01\x92`@Qa\x1A\x9E\x81a\x1E\xCFV[\x85T\x81R\x84\x80`\xA0\x1B\x03\x85\x87\x01T\x16\x83\x82\x01R`\x02\x86\x01T`@\x82\x01R`\x03\x86\x01T``\x82\x01R`\x04\x86\x01T`\x80\x82\x01R`\xFF`\x05\x87\x01T\x16\x15\x15`\xA0\x82\x01Ra\x1A\xEA`\x06\x87\x01a\x1F\x0BV[`\xC0\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x19\xD8V[a\x1B\x066a\x1E\"V[\x90\x80\x15\x80\x15a\x1E\x17W[a\x02\xD0Wa\x1B\x1Ca$9V[a\x1B$a$WV[\x80_R`\x02` R`@_ `@Q\x90a\x1B=\x82a\x1E\xB3V[\x80T\x82Ra\x1BM`\x01\x82\x01a\x1F\x0BV[` \x83\x01R`\x02\x81\x01T\x90`@\x83\x01\x91\x82R`\x03\x81\x01T``\x84\x01\x90\x81R`\x04\x82\x01T\x91`\x80\x85\x01\x92\x83R`\x08`\xFF`\x05\x83\x01T\x16\x91`\xA0\x87\x01\x92\x15\x15\x83Ra\x1B\x98`\x06\x82\x01a\x1F\x0BV[`\xC0\x88\x01Ra\x1B\xA9`\x07\x82\x01a\x1F\x0BV[`\xE0\x88\x01R\x01Ta\x01\0\x86\x01RQ\x15a\tTWQ\x85\x10\x90\x81\x15a\x1E\x0CW[Pa\t\xA8Wa\x0E\x0F\x84\x01\x90\x81\x85\x11a\x1D\x91Wa\x0E\x10a\x1B\xE8\x92\x04\x90Qa!\x1DV[\x90\x814\x10a\x0F[Wa\x1C\ta'\x10a\x1C\x02`\x08T\x85a!\x1DV[\x04\x83a!0V[\x90Q`@Qc\\]J\xFD`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x90_\x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\t\x9DW_\x92\x83\x92\x83\x92\x83\x92\x83\x91a\x1D\xEAW[PZ\xF1a\x1Cra\">V[P\x15a\x1D\xA5W\x81_R`\x03` R`@_ T\x90\x83B\x01\x80B\x11a\x1D\x91W`@Q\x90a\x1C\x9D\x82a\x1E\xCFV[\x84\x82R3` \x83\x01RB`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R`\x01`\xA0\x82\x01Ra\x1C\xFD` \x91`@Qa\x1C\xD1\x84\x82a\x1E\xEAV[_\x81R`\xC0\x82\x01R\x85_R`\x03\x83Ra\x1C\xED\x81`@_ a\"mV[3_R`\x04\x83R`@_ a\"mV[\x814\x11a\x1D?W[`@Q\x94\x85R\x84\x01R`@\x83\x01R\x7F/\\\xBC\x8BX\x94\x9Cm\x94Y-r7\xAA\x1A'\xE9\xE6\xAC\xDC\xB1\xB8*J\x1D\x0B'\xDD\xFD\xAF\xE5\xEA``3\x93\xA3`\x01_U\0[_\x80\x80\x80a\x1DM\x864a!0V[3Z\xF1a\x1DXa\">V[Pa\x1D\x05W`d\x90`@Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`\r`$\x82\x01Rl\x14\x99Y\x9D[\x99\x08\x19\x98Z[\x19Y`\x9A\x1B`D\x82\x01R\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FPayment to vehicle failed\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[a\x1D\xFE\x91P=\x80\x85\x83>a\t\xF9\x81\x83a\x1E\xEAV[PPPPPP\x90P\x89a\x1CgV[\x90PQ\x84\x11\x85a\x1B\xC7V[P`\x06T\x81\x11a\x1B\x10V[`@\x90`\x03\x19\x01\x12a\x02\xEAW`\x045\x90`$5\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xEAWV[\x80T\x82\x10\x15a\x1EgW_R`\x07` _ \x91\x02\x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x1E\xA9W[` \x83\x10\x14a\x1E\x95WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x1E\x8AV[a\x01 \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\xC7W`@RV[\x90`@Q\x91\x82_\x82T\x92a\x1F\x1E\x84a\x1E{V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x1F\x89WP`\x01\x14a\x1FEW[Pa\x1FC\x92P\x03\x83a\x1E\xEAV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x1FmWPP\x90` a\x1FC\x92\x82\x01\x01_a\x1F6V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x1FTV[\x90P` \x92Pa\x1FC\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x1F6V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x93\x90\x92`\xE0\x95\x92a \x0E\x98\x97\x94\x86R`\x01\x80`\xA0\x1B\x03\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R\x15\x15`\xA0\x82\x01R\x81`\xC0\x82\x01R\x01\x90a\x1F\xABV[\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x02\xEAW\x805\x90a C\x82a \x11V[\x92a Q`@Q\x94\x85a\x1E\xEAV[\x82\x84R` \x83\x83\x01\x01\x11a\x02\xEAW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a \xA4WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a!\x0E`\x01\x93`?\x19\x86\x82\x03\x01\x87R`\xE0`\xC0\x8BQ\x80Q\x84R\x87\x80`\xA0\x1B\x03\x86\x82\x01Q\x16\x86\x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q\x15\x15`\xA0\x85\x01R\x01Q\x91\x81`\xC0\x82\x01R\x01\x90a\x1F\xABV[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a \x95V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1D\x91WV[\x91\x90\x82\x03\x91\x82\x11a\x1D\x91WV[\x81`\x1F\x82\x01\x12\x15a\x02\xEAW\x80Q\x90a!T\x82a \x11V[\x92a!b`@Q\x94\x85a\x1E\xEAV[\x82\x84R` \x83\x83\x01\x01\x11a\x02\xEAW\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xEAWV[Q\x90\x81\x15\x15\x82\x03a\x02\xEAWV[\x90a\x01\0\x82\x82\x03\x12a\x02\xEAW\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAW\x81a!\xCC\x91\x84\x01a!=V[\x92a!\xD9` \x84\x01a!\x83V[\x92`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xEAW\x83a!\xF9\x91\x83\x01a!=V[\x92``\x82\x01Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xEAWa\"\x19\x91\x83\x01a!=V[\x91`\x80\x82\x01Q\x91a\",`\xA0\x82\x01a!\x97V[\x91a \x0E`\xE0`\xC0\x84\x01Q\x93\x01a!\x83V[=\x15a\"hW=\x90a\"O\x82a \x11V[\x91a\"]`@Q\x93\x84a\x1E\xEAV[\x82R=_` \x84\x01>V[``\x90V[\x91\x90\x91\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x07\xC7Wa\"\x92\x91`\x01\x82\x01\x81Ua\x1ENV[a$\x04W\x82Q\x81U` \x83\x01Q`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@\x83\x01Q`\x02\x82\x01U``\x83\x01Q`\x03\x82\x01U`\x80\x83\x01Q`\x04\x82\x01U`\xA0\x83\x01Q`\x05\x82\x01\x80T`\xFF\x19\x16\x91\x15\x15`\xFF\x16\x91\x90\x91\x17\x90U`\xC0\x90`\x06\x01\x92\x01Q\x91\x82Q`\x01`\x01`@\x1B\x03\x81\x11a\x07\xC7Wa#\x1F\x82Ta\x1E{V[`\x1F\x81\x11a#\xBFW[P` `\x1F\x82\x11`\x01\x14a#aW\x81\x92\x93\x94_\x92a#VW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01Q\x90P_\x80a#AV[`\x1F\x19\x82\x16\x90\x83_R\x80_ \x91_[\x81\x81\x10a#\xA7WP\x95\x83`\x01\x95\x96\x97\x10a#\x8FW[PPP\x81\x1B\x01\x90UV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a#\x85V[\x91\x92` `\x01\x81\x92\x86\x8B\x01Q\x81U\x01\x94\x01\x92\x01a#pV[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a#\xFAW[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a#\xEFWPa#(V[_\x81U`\x01\x01a#\xE2V[\x90\x91P\x81\x90a#\xD9V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x1EgW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x1D\x91W`\x01\x01\x90V[`\x02_T\x14a$HW`\x02_UV[c>\xE5\xAE\xB5`\xE0\x1B_R`\x04_\xFD[`\xFF`\x01T\x16a$cWV[c\xD9<\x06e`\xE0\x1B_R`\x04_\xFD[`\x01T`\x08\x1C`\x01`\x01`\xA0\x1B\x03\x163\x03a$\x89WV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 w\xA0\xD6\xC6\x80\x96CH\xF1;\x84g\xDBh\x16\xCC\x81\xD6\xE6BU-u\x16R\x106\x08n\xA4\xB9\xDFdsolcC\0\x08\x1E\x003";
    /// The deployed bytecode of the contract.
    pub static DATAMARKETPLACE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DataMarketplace<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DataMarketplace<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DataMarketplace<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DataMarketplace<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DataMarketplace<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DataMarketplace))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DataMarketplace<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DATAMARKETPLACE_ABI.clone(),
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
                DATAMARKETPLACE_ABI.clone(),
                DATAMARKETPLACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_PLATFORM_FEE` (0x3998a681) function
        pub fn max_platform_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 152, 166, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `buyerPurchases` (0x34b7bfef) function
        pub fn buyer_purchases(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::std::string::String,
            ),
        > {
            self.0
                .method_hash([52, 183, 191, 239], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dataProducts` (0x76700a8b) function
        pub fn data_products(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([118, 112, 10, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBuyerPurchases` (0x85a34393) function
        pub fn get_buyer_purchases(
            &self,
            buyer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Purchase>> {
            self.0
                .method_hash([133, 163, 67, 147], buyer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDataProduct` (0xfcc32670) function
        pub fn get_data_product(
            &self,
            product_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, DataProduct> {
            self.0
                .method_hash([252, 195, 38, 112], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProductPurchases` (0xb8b650bf) function
        pub fn get_product_purchases(
            &self,
            product_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Purchase>> {
            self.0
                .method_hash([184, 182, 80, 191], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStreamingPayment` (0x96964848) function
        pub fn get_streaming_payment(
            &self,
            streaming_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StreamingPayment> {
            self.0
                .method_hash([150, 150, 72, 72], streaming_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalProducts` (0x570aea8a) function
        pub fn get_total_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 10, 234, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStreamingPayments` (0x14ff266e) function
        pub fn get_total_streaming_payments(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 255, 38, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasValidAccess` (0x2153e3ee) function
        pub fn has_valid_access(
            &self,
            buyer: ::ethers::core::types::Address,
            product_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([33, 83, 227, 238], (buyer, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listDataProduct` (0xd4d8dffa) function
        pub fn list_data_product(
            &self,
            vehicle_id: ::ethers::core::types::U256,
            data_type: ::std::string::String,
            price_per_hour: ::ethers::core::types::U256,
            min_duration: ::ethers::core::types::U256,
            max_duration: ::ethers::core::types::U256,
            description: ::std::string::String,
            api_endpoint: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [212, 216, 223, 250],
                    (
                        vehicle_id,
                        data_type,
                        price_per_hour,
                        min_duration,
                        max_duration,
                        description,
                        api_endpoint,
                    ),
                )
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
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `platformFeeRate` (0xeeca08f0) function
        pub fn platform_fee_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([238, 202, 8, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `productCounter` (0x1f16653b) function
        pub fn product_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 22, 101, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `productPurchases` (0x38bbd4e1) function
        pub fn product_purchases(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::std::string::String,
            ),
        > {
            self.0
                .method_hash([56, 187, 212, 225], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `purchaseDataAccess` (0x0e919159) function
        pub fn purchase_data_access(
            &self,
            product_id: ::ethers::core::types::U256,
            duration_in_seconds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 145, 145, 89], (product_id, duration_in_seconds))
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
        ///Calls the contract's `setPlatformFeeRate` (0x927fef2e) function
        pub fn set_platform_fee_rate(
            &self,
            new_fee_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 127, 239, 46], new_fee_rate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startStreamingPayment` (0x866b7480) function
        pub fn start_streaming_payment(
            &self,
            product_id: ::ethers::core::types::U256,
            rate_per_second: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 107, 116, 128], (product_id, rate_per_second))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `streamingCounter` (0xdc3309ad) function
        pub fn streaming_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 51, 9, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `streamingPayments` (0xcd56f0ac) function
        pub fn streaming_payments(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([205, 86, 240, 172], p0)
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
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDataProduct` (0x5f7a696d) function
        pub fn update_data_product(
            &self,
            product_id: ::ethers::core::types::U256,
            new_price_per_hour: ::ethers::core::types::U256,
            is_active: bool,
            new_description: ::std::string::String,
            new_api_endpoint: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [95, 122, 105, 109],
                    (
                        product_id,
                        new_price_per_hour,
                        is_active,
                        new_description,
                        new_api_endpoint,
                    ),
                )
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
        ///Calls the contract's `withdrawPlatformFees` (0xd0b7830b) function
        pub fn withdraw_platform_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 183, 131, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStreamingPayment` (0x230c8dd3) function
        pub fn withdraw_streaming_payment(
            &self,
            streaming_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 12, 141, 211], streaming_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DataProductListed` event
        pub fn data_product_listed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataProductListedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DataProductUpdated` event
        pub fn data_product_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataProductUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DataPurchased` event
        pub fn data_purchased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataPurchasedFilter,
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
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PlatformFeeUpdated` event
        pub fn platform_fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PlatformFeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StreamingPaymentStarted` event
        pub fn streaming_payment_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StreamingPaymentStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StreamingPaymentWithdrawn` event
        pub fn streaming_payment_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StreamingPaymentWithdrawnFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataMarketplaceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DataMarketplace<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessExpired` with signature `AccessExpired()` and selector `0xfdfb7e2e`
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
    #[etherror(name = "AccessExpired", abi = "AccessExpired()")]
    pub struct AccessExpired;
    ///Custom Error type `EnforcedPause` with signature `EnforcedPause()` and selector `0xd93c0665`
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
    #[etherror(name = "EnforcedPause", abi = "EnforcedPause()")]
    pub struct EnforcedPause;
    ///Custom Error type `ExpectedPause` with signature `ExpectedPause()` and selector `0x8dfc202b`
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
    #[etherror(name = "ExpectedPause", abi = "ExpectedPause()")]
    pub struct ExpectedPause;
    ///Custom Error type `InsufficientPayment` with signature `InsufficientPayment()` and selector `0xcd1c8867`
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
    #[etherror(name = "InsufficientPayment", abi = "InsufficientPayment()")]
    pub struct InsufficientPayment;
    ///Custom Error type `InvalidDuration` with signature `InvalidDuration()` and selector `0x76166401`
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
    #[etherror(name = "InvalidDuration", abi = "InvalidDuration()")]
    pub struct InvalidDuration;
    ///Custom Error type `InvalidFeeRate` with signature `InvalidFeeRate()` and selector `0x56d69198`
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
    #[etherror(name = "InvalidFeeRate", abi = "InvalidFeeRate()")]
    pub struct InvalidFeeRate;
    ///Custom Error type `InvalidPrice` with signature `InvalidPrice()` and selector `0x00bfc921`
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
    #[etherror(name = "InvalidPrice", abi = "InvalidPrice()")]
    pub struct InvalidPrice;
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
    ///Custom Error type `ProductNotActive` with signature `ProductNotActive()` and selector `0x2cdd6187`
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
    #[etherror(name = "ProductNotActive", abi = "ProductNotActive()")]
    pub struct ProductNotActive;
    ///Custom Error type `ProductNotFound` with signature `ProductNotFound()` and selector `0x79de4af5`
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
    #[etherror(name = "ProductNotFound", abi = "ProductNotFound()")]
    pub struct ProductNotFound;
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
    ///Custom Error type `StreamingNotActive` with signature `StreamingNotActive()` and selector `0xb141b2b2`
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
    #[etherror(name = "StreamingNotActive", abi = "StreamingNotActive()")]
    pub struct StreamingNotActive;
    ///Custom Error type `UnauthorizedSeller` with signature `UnauthorizedSeller()` and selector `0x32d2cc20`
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
    #[etherror(name = "UnauthorizedSeller", abi = "UnauthorizedSeller()")]
    pub struct UnauthorizedSeller;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DataMarketplaceErrors {
        AccessExpired(AccessExpired),
        EnforcedPause(EnforcedPause),
        ExpectedPause(ExpectedPause),
        InsufficientPayment(InsufficientPayment),
        InvalidDuration(InvalidDuration),
        InvalidFeeRate(InvalidFeeRate),
        InvalidPrice(InvalidPrice),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        ProductNotActive(ProductNotActive),
        ProductNotFound(ProductNotFound),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        StreamingNotActive(StreamingNotActive),
        UnauthorizedSeller(UnauthorizedSeller),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DataMarketplaceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccessExpired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccessExpired(decoded));
            }
            if let Ok(decoded) = <EnforcedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnforcedPause(decoded));
            }
            if let Ok(decoded) = <ExpectedPause as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectedPause(decoded));
            }
            if let Ok(decoded) = <InsufficientPayment as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientPayment(decoded));
            }
            if let Ok(decoded) = <InvalidDuration as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDuration(decoded));
            }
            if let Ok(decoded) = <InvalidFeeRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeRate(decoded));
            }
            if let Ok(decoded) = <InvalidPrice as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPrice(decoded));
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
            if let Ok(decoded) = <ProductNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProductNotActive(decoded));
            }
            if let Ok(decoded) = <ProductNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProductNotFound(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardReentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardReentrantCall(decoded));
            }
            if let Ok(decoded) = <StreamingNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StreamingNotActive(decoded));
            }
            if let Ok(decoded) = <UnauthorizedSeller as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnauthorizedSeller(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DataMarketplaceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnforcedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectedPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProductNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProductNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StreamingNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnauthorizedSeller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DataMarketplaceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EnforcedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExpectedPause as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientPayment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPrice as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProductNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ProductNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyGuardReentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StreamingNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnauthorizedSeller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DataMarketplaceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnforcedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectedPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientPayment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProductNotActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StreamingNotActive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnauthorizedSeller(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DataMarketplaceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessExpired> for DataMarketplaceErrors {
        fn from(value: AccessExpired) -> Self {
            Self::AccessExpired(value)
        }
    }
    impl ::core::convert::From<EnforcedPause> for DataMarketplaceErrors {
        fn from(value: EnforcedPause) -> Self {
            Self::EnforcedPause(value)
        }
    }
    impl ::core::convert::From<ExpectedPause> for DataMarketplaceErrors {
        fn from(value: ExpectedPause) -> Self {
            Self::ExpectedPause(value)
        }
    }
    impl ::core::convert::From<InsufficientPayment> for DataMarketplaceErrors {
        fn from(value: InsufficientPayment) -> Self {
            Self::InsufficientPayment(value)
        }
    }
    impl ::core::convert::From<InvalidDuration> for DataMarketplaceErrors {
        fn from(value: InvalidDuration) -> Self {
            Self::InvalidDuration(value)
        }
    }
    impl ::core::convert::From<InvalidFeeRate> for DataMarketplaceErrors {
        fn from(value: InvalidFeeRate) -> Self {
            Self::InvalidFeeRate(value)
        }
    }
    impl ::core::convert::From<InvalidPrice> for DataMarketplaceErrors {
        fn from(value: InvalidPrice) -> Self {
            Self::InvalidPrice(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for DataMarketplaceErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for DataMarketplaceErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<ProductNotActive> for DataMarketplaceErrors {
        fn from(value: ProductNotActive) -> Self {
            Self::ProductNotActive(value)
        }
    }
    impl ::core::convert::From<ProductNotFound> for DataMarketplaceErrors {
        fn from(value: ProductNotFound) -> Self {
            Self::ProductNotFound(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for DataMarketplaceErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<StreamingNotActive> for DataMarketplaceErrors {
        fn from(value: StreamingNotActive) -> Self {
            Self::StreamingNotActive(value)
        }
    }
    impl ::core::convert::From<UnauthorizedSeller> for DataMarketplaceErrors {
        fn from(value: UnauthorizedSeller) -> Self {
            Self::UnauthorizedSeller(value)
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
        name = "DataProductListed",
        abi = "DataProductListed(uint256,uint256,string,uint256)"
    )]
    pub struct DataProductListedFilter {
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub vehicle_id: ::ethers::core::types::U256,
        pub data_type: ::std::string::String,
        pub price_per_hour: ::ethers::core::types::U256,
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
        name = "DataProductUpdated",
        abi = "DataProductUpdated(uint256,uint256,bool)"
    )]
    pub struct DataProductUpdatedFilter {
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        pub new_price: ::ethers::core::types::U256,
        pub is_active: bool,
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
        name = "DataPurchased",
        abi = "DataPurchased(uint256,address,uint256,uint256,uint256)"
    )]
    pub struct DataPurchasedFilter {
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub buyer: ::ethers::core::types::Address,
        pub duration: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub purchase_index: ::ethers::core::types::U256,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(name = "PlatformFeeUpdated", abi = "PlatformFeeUpdated(uint256)")]
    pub struct PlatformFeeUpdatedFilter {
        pub new_fee_rate: ::ethers::core::types::U256,
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
        name = "StreamingPaymentStarted",
        abi = "StreamingPaymentStarted(uint256,uint256,address,uint256)"
    )]
    pub struct StreamingPaymentStartedFilter {
        #[ethevent(indexed)]
        pub streaming_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub product_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub buyer: ::ethers::core::types::Address,
        pub rate_per_second: ::ethers::core::types::U256,
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
        name = "StreamingPaymentWithdrawn",
        abi = "StreamingPaymentWithdrawn(uint256,uint256,address)"
    )]
    pub struct StreamingPaymentWithdrawnFilter {
        #[ethevent(indexed)]
        pub streaming_id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub vehicle_wallet: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DataMarketplaceEvents {
        DataProductListedFilter(DataProductListedFilter),
        DataProductUpdatedFilter(DataProductUpdatedFilter),
        DataPurchasedFilter(DataPurchasedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PlatformFeeUpdatedFilter(PlatformFeeUpdatedFilter),
        StreamingPaymentStartedFilter(StreamingPaymentStartedFilter),
        StreamingPaymentWithdrawnFilter(StreamingPaymentWithdrawnFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DataMarketplaceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DataProductListedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::DataProductListedFilter(decoded));
            }
            if let Ok(decoded) = DataProductUpdatedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::DataProductUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DataPurchasedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::DataPurchasedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PlatformFeeUpdatedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::PlatformFeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = StreamingPaymentStartedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::StreamingPaymentStartedFilter(decoded));
            }
            if let Ok(decoded) = StreamingPaymentWithdrawnFilter::decode_log(log) {
                return Ok(
                    DataMarketplaceEvents::StreamingPaymentWithdrawnFilter(decoded),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(DataMarketplaceEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DataMarketplaceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DataProductListedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DataProductUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DataPurchasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlatformFeeUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StreamingPaymentStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StreamingPaymentWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DataProductListedFilter> for DataMarketplaceEvents {
        fn from(value: DataProductListedFilter) -> Self {
            Self::DataProductListedFilter(value)
        }
    }
    impl ::core::convert::From<DataProductUpdatedFilter> for DataMarketplaceEvents {
        fn from(value: DataProductUpdatedFilter) -> Self {
            Self::DataProductUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DataPurchasedFilter> for DataMarketplaceEvents {
        fn from(value: DataPurchasedFilter) -> Self {
            Self::DataPurchasedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DataMarketplaceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for DataMarketplaceEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PlatformFeeUpdatedFilter> for DataMarketplaceEvents {
        fn from(value: PlatformFeeUpdatedFilter) -> Self {
            Self::PlatformFeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StreamingPaymentStartedFilter> for DataMarketplaceEvents {
        fn from(value: StreamingPaymentStartedFilter) -> Self {
            Self::StreamingPaymentStartedFilter(value)
        }
    }
    impl ::core::convert::From<StreamingPaymentWithdrawnFilter>
    for DataMarketplaceEvents {
        fn from(value: StreamingPaymentWithdrawnFilter) -> Self {
            Self::StreamingPaymentWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for DataMarketplaceEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_PLATFORM_FEE` function with signature `MAX_PLATFORM_FEE()` and selector `0x3998a681`
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
    #[ethcall(name = "MAX_PLATFORM_FEE", abi = "MAX_PLATFORM_FEE()")]
    pub struct MaxPlatformFeeCall;
    ///Container type for all input parameters for the `buyerPurchases` function with signature `buyerPurchases(address,uint256)` and selector `0x34b7bfef`
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
    #[ethcall(name = "buyerPurchases", abi = "buyerPurchases(address,uint256)")]
    pub struct BuyerPurchasesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `dataProducts` function with signature `dataProducts(uint256)` and selector `0x76700a8b`
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
    #[ethcall(name = "dataProducts", abi = "dataProducts(uint256)")]
    pub struct DataProductsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getBuyerPurchases` function with signature `getBuyerPurchases(address)` and selector `0x85a34393`
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
    #[ethcall(name = "getBuyerPurchases", abi = "getBuyerPurchases(address)")]
    pub struct GetBuyerPurchasesCall {
        pub buyer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDataProduct` function with signature `getDataProduct(uint256)` and selector `0xfcc32670`
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
    #[ethcall(name = "getDataProduct", abi = "getDataProduct(uint256)")]
    pub struct GetDataProductCall {
        pub product_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getProductPurchases` function with signature `getProductPurchases(uint256)` and selector `0xb8b650bf`
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
    #[ethcall(name = "getProductPurchases", abi = "getProductPurchases(uint256)")]
    pub struct GetProductPurchasesCall {
        pub product_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStreamingPayment` function with signature `getStreamingPayment(uint256)` and selector `0x96964848`
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
    #[ethcall(name = "getStreamingPayment", abi = "getStreamingPayment(uint256)")]
    pub struct GetStreamingPaymentCall {
        pub streaming_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalProducts` function with signature `getTotalProducts()` and selector `0x570aea8a`
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
    #[ethcall(name = "getTotalProducts", abi = "getTotalProducts()")]
    pub struct GetTotalProductsCall;
    ///Container type for all input parameters for the `getTotalStreamingPayments` function with signature `getTotalStreamingPayments()` and selector `0x14ff266e`
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
    #[ethcall(name = "getTotalStreamingPayments", abi = "getTotalStreamingPayments()")]
    pub struct GetTotalStreamingPaymentsCall;
    ///Container type for all input parameters for the `hasValidAccess` function with signature `hasValidAccess(address,uint256)` and selector `0x2153e3ee`
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
    #[ethcall(name = "hasValidAccess", abi = "hasValidAccess(address,uint256)")]
    pub struct HasValidAccessCall {
        pub buyer: ::ethers::core::types::Address,
        pub product_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `listDataProduct` function with signature `listDataProduct(uint256,string,uint256,uint256,uint256,string,string)` and selector `0xd4d8dffa`
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
        name = "listDataProduct",
        abi = "listDataProduct(uint256,string,uint256,uint256,uint256,string,string)"
    )]
    pub struct ListDataProductCall {
        pub vehicle_id: ::ethers::core::types::U256,
        pub data_type: ::std::string::String,
        pub price_per_hour: ::ethers::core::types::U256,
        pub min_duration: ::ethers::core::types::U256,
        pub max_duration: ::ethers::core::types::U256,
        pub description: ::std::string::String,
        pub api_endpoint: ::std::string::String,
    }
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
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `platformFeeRate` function with signature `platformFeeRate()` and selector `0xeeca08f0`
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
    #[ethcall(name = "platformFeeRate", abi = "platformFeeRate()")]
    pub struct PlatformFeeRateCall;
    ///Container type for all input parameters for the `productCounter` function with signature `productCounter()` and selector `0x1f16653b`
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
    #[ethcall(name = "productCounter", abi = "productCounter()")]
    pub struct ProductCounterCall;
    ///Container type for all input parameters for the `productPurchases` function with signature `productPurchases(uint256,uint256)` and selector `0x38bbd4e1`
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
    #[ethcall(name = "productPurchases", abi = "productPurchases(uint256,uint256)")]
    pub struct ProductPurchasesCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `purchaseDataAccess` function with signature `purchaseDataAccess(uint256,uint256)` and selector `0x0e919159`
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
    #[ethcall(name = "purchaseDataAccess", abi = "purchaseDataAccess(uint256,uint256)")]
    pub struct PurchaseDataAccessCall {
        pub product_id: ::ethers::core::types::U256,
        pub duration_in_seconds: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `setPlatformFeeRate` function with signature `setPlatformFeeRate(uint256)` and selector `0x927fef2e`
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
    #[ethcall(name = "setPlatformFeeRate", abi = "setPlatformFeeRate(uint256)")]
    pub struct SetPlatformFeeRateCall {
        pub new_fee_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `startStreamingPayment` function with signature `startStreamingPayment(uint256,uint256)` and selector `0x866b7480`
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
        name = "startStreamingPayment",
        abi = "startStreamingPayment(uint256,uint256)"
    )]
    pub struct StartStreamingPaymentCall {
        pub product_id: ::ethers::core::types::U256,
        pub rate_per_second: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `streamingCounter` function with signature `streamingCounter()` and selector `0xdc3309ad`
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
    #[ethcall(name = "streamingCounter", abi = "streamingCounter()")]
    pub struct StreamingCounterCall;
    ///Container type for all input parameters for the `streamingPayments` function with signature `streamingPayments(uint256)` and selector `0xcd56f0ac`
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
    #[ethcall(name = "streamingPayments", abi = "streamingPayments(uint256)")]
    pub struct StreamingPaymentsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `updateDataProduct` function with signature `updateDataProduct(uint256,uint256,bool,string,string)` and selector `0x5f7a696d`
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
        name = "updateDataProduct",
        abi = "updateDataProduct(uint256,uint256,bool,string,string)"
    )]
    pub struct UpdateDataProductCall {
        pub product_id: ::ethers::core::types::U256,
        pub new_price_per_hour: ::ethers::core::types::U256,
        pub is_active: bool,
        pub new_description: ::std::string::String,
        pub new_api_endpoint: ::std::string::String,
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
    ///Container type for all input parameters for the `withdrawPlatformFees` function with signature `withdrawPlatformFees()` and selector `0xd0b7830b`
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
    #[ethcall(name = "withdrawPlatformFees", abi = "withdrawPlatformFees()")]
    pub struct WithdrawPlatformFeesCall;
    ///Container type for all input parameters for the `withdrawStreamingPayment` function with signature `withdrawStreamingPayment(uint256)` and selector `0x230c8dd3`
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
        name = "withdrawStreamingPayment",
        abi = "withdrawStreamingPayment(uint256)"
    )]
    pub struct WithdrawStreamingPaymentCall {
        pub streaming_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DataMarketplaceCalls {
        MaxPlatformFee(MaxPlatformFeeCall),
        BuyerPurchases(BuyerPurchasesCall),
        DataProducts(DataProductsCall),
        GetBuyerPurchases(GetBuyerPurchasesCall),
        GetDataProduct(GetDataProductCall),
        GetProductPurchases(GetProductPurchasesCall),
        GetStreamingPayment(GetStreamingPaymentCall),
        GetTotalProducts(GetTotalProductsCall),
        GetTotalStreamingPayments(GetTotalStreamingPaymentsCall),
        HasValidAccess(HasValidAccessCall),
        ListDataProduct(ListDataProductCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PlatformFeeRate(PlatformFeeRateCall),
        ProductCounter(ProductCounterCall),
        ProductPurchases(ProductPurchasesCall),
        PurchaseDataAccess(PurchaseDataAccessCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPlatformFeeRate(SetPlatformFeeRateCall),
        StartStreamingPayment(StartStreamingPaymentCall),
        StreamingCounter(StreamingCounterCall),
        StreamingPayments(StreamingPaymentsCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateDataProduct(UpdateDataProductCall),
        VehicleRegistry(VehicleRegistryCall),
        WithdrawPlatformFees(WithdrawPlatformFeesCall),
        WithdrawStreamingPayment(WithdrawStreamingPaymentCall),
    }
    impl ::ethers::core::abi::AbiDecode for DataMarketplaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxPlatformFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) = <BuyerPurchasesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BuyerPurchases(decoded));
            }
            if let Ok(decoded) = <DataProductsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataProducts(decoded));
            }
            if let Ok(decoded) = <GetBuyerPurchasesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBuyerPurchases(decoded));
            }
            if let Ok(decoded) = <GetDataProductCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDataProduct(decoded));
            }
            if let Ok(decoded) = <GetProductPurchasesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProductPurchases(decoded));
            }
            if let Ok(decoded) = <GetStreamingPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStreamingPayment(decoded));
            }
            if let Ok(decoded) = <GetTotalProductsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalProducts(decoded));
            }
            if let Ok(decoded) = <GetTotalStreamingPaymentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStreamingPayments(decoded));
            }
            if let Ok(decoded) = <HasValidAccessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasValidAccess(decoded));
            }
            if let Ok(decoded) = <ListDataProductCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ListDataProduct(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PlatformFeeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PlatformFeeRate(decoded));
            }
            if let Ok(decoded) = <ProductCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProductCounter(decoded));
            }
            if let Ok(decoded) = <ProductPurchasesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProductPurchases(decoded));
            }
            if let Ok(decoded) = <PurchaseDataAccessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PurchaseDataAccess(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetPlatformFeeRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPlatformFeeRate(decoded));
            }
            if let Ok(decoded) = <StartStreamingPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartStreamingPayment(decoded));
            }
            if let Ok(decoded) = <StreamingCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StreamingCounter(decoded));
            }
            if let Ok(decoded) = <StreamingPaymentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StreamingPayments(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateDataProductCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDataProduct(decoded));
            }
            if let Ok(decoded) = <VehicleRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleRegistry(decoded));
            }
            if let Ok(decoded) = <WithdrawPlatformFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawPlatformFees(decoded));
            }
            if let Ok(decoded) = <WithdrawStreamingPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawStreamingPayment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DataMarketplaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxPlatformFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BuyerPurchases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DataProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBuyerPurchases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDataProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProductPurchases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStreamingPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStreamingPayments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasValidAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListDataProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PlatformFeeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProductCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProductPurchases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PurchaseDataAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPlatformFeeRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartStreamingPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StreamingCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StreamingPayments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateDataProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawPlatformFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawStreamingPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DataMarketplaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxPlatformFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::BuyerPurchases(element) => ::core::fmt::Display::fmt(element, f),
                Self::DataProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBuyerPurchases(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDataProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductPurchases(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStreamingPayment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalStreamingPayments(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasValidAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListDataProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlatformFeeRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductPurchases(element) => ::core::fmt::Display::fmt(element, f),
                Self::PurchaseDataAccess(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPlatformFeeRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartStreamingPayment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StreamingCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StreamingPayments(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDataProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::VehicleRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawPlatformFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawStreamingPayment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MaxPlatformFeeCall> for DataMarketplaceCalls {
        fn from(value: MaxPlatformFeeCall) -> Self {
            Self::MaxPlatformFee(value)
        }
    }
    impl ::core::convert::From<BuyerPurchasesCall> for DataMarketplaceCalls {
        fn from(value: BuyerPurchasesCall) -> Self {
            Self::BuyerPurchases(value)
        }
    }
    impl ::core::convert::From<DataProductsCall> for DataMarketplaceCalls {
        fn from(value: DataProductsCall) -> Self {
            Self::DataProducts(value)
        }
    }
    impl ::core::convert::From<GetBuyerPurchasesCall> for DataMarketplaceCalls {
        fn from(value: GetBuyerPurchasesCall) -> Self {
            Self::GetBuyerPurchases(value)
        }
    }
    impl ::core::convert::From<GetDataProductCall> for DataMarketplaceCalls {
        fn from(value: GetDataProductCall) -> Self {
            Self::GetDataProduct(value)
        }
    }
    impl ::core::convert::From<GetProductPurchasesCall> for DataMarketplaceCalls {
        fn from(value: GetProductPurchasesCall) -> Self {
            Self::GetProductPurchases(value)
        }
    }
    impl ::core::convert::From<GetStreamingPaymentCall> for DataMarketplaceCalls {
        fn from(value: GetStreamingPaymentCall) -> Self {
            Self::GetStreamingPayment(value)
        }
    }
    impl ::core::convert::From<GetTotalProductsCall> for DataMarketplaceCalls {
        fn from(value: GetTotalProductsCall) -> Self {
            Self::GetTotalProducts(value)
        }
    }
    impl ::core::convert::From<GetTotalStreamingPaymentsCall> for DataMarketplaceCalls {
        fn from(value: GetTotalStreamingPaymentsCall) -> Self {
            Self::GetTotalStreamingPayments(value)
        }
    }
    impl ::core::convert::From<HasValidAccessCall> for DataMarketplaceCalls {
        fn from(value: HasValidAccessCall) -> Self {
            Self::HasValidAccess(value)
        }
    }
    impl ::core::convert::From<ListDataProductCall> for DataMarketplaceCalls {
        fn from(value: ListDataProductCall) -> Self {
            Self::ListDataProduct(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DataMarketplaceCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for DataMarketplaceCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for DataMarketplaceCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PlatformFeeRateCall> for DataMarketplaceCalls {
        fn from(value: PlatformFeeRateCall) -> Self {
            Self::PlatformFeeRate(value)
        }
    }
    impl ::core::convert::From<ProductCounterCall> for DataMarketplaceCalls {
        fn from(value: ProductCounterCall) -> Self {
            Self::ProductCounter(value)
        }
    }
    impl ::core::convert::From<ProductPurchasesCall> for DataMarketplaceCalls {
        fn from(value: ProductPurchasesCall) -> Self {
            Self::ProductPurchases(value)
        }
    }
    impl ::core::convert::From<PurchaseDataAccessCall> for DataMarketplaceCalls {
        fn from(value: PurchaseDataAccessCall) -> Self {
            Self::PurchaseDataAccess(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DataMarketplaceCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPlatformFeeRateCall> for DataMarketplaceCalls {
        fn from(value: SetPlatformFeeRateCall) -> Self {
            Self::SetPlatformFeeRate(value)
        }
    }
    impl ::core::convert::From<StartStreamingPaymentCall> for DataMarketplaceCalls {
        fn from(value: StartStreamingPaymentCall) -> Self {
            Self::StartStreamingPayment(value)
        }
    }
    impl ::core::convert::From<StreamingCounterCall> for DataMarketplaceCalls {
        fn from(value: StreamingCounterCall) -> Self {
            Self::StreamingCounter(value)
        }
    }
    impl ::core::convert::From<StreamingPaymentsCall> for DataMarketplaceCalls {
        fn from(value: StreamingPaymentsCall) -> Self {
            Self::StreamingPayments(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DataMarketplaceCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for DataMarketplaceCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateDataProductCall> for DataMarketplaceCalls {
        fn from(value: UpdateDataProductCall) -> Self {
            Self::UpdateDataProduct(value)
        }
    }
    impl ::core::convert::From<VehicleRegistryCall> for DataMarketplaceCalls {
        fn from(value: VehicleRegistryCall) -> Self {
            Self::VehicleRegistry(value)
        }
    }
    impl ::core::convert::From<WithdrawPlatformFeesCall> for DataMarketplaceCalls {
        fn from(value: WithdrawPlatformFeesCall) -> Self {
            Self::WithdrawPlatformFees(value)
        }
    }
    impl ::core::convert::From<WithdrawStreamingPaymentCall> for DataMarketplaceCalls {
        fn from(value: WithdrawStreamingPaymentCall) -> Self {
            Self::WithdrawStreamingPayment(value)
        }
    }
    ///Container type for all return fields from the `MAX_PLATFORM_FEE` function with signature `MAX_PLATFORM_FEE()` and selector `0x3998a681`
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
    pub struct MaxPlatformFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `buyerPurchases` function with signature `buyerPurchases(address,uint256)` and selector `0x34b7bfef`
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
    pub struct BuyerPurchasesReturn {
        pub product_id: ::ethers::core::types::U256,
        pub buyer: ::ethers::core::types::Address,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub is_active: bool,
        pub access_token: ::std::string::String,
    }
    ///Container type for all return fields from the `dataProducts` function with signature `dataProducts(uint256)` and selector `0x76700a8b`
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
    pub struct DataProductsReturn {
        pub vehicle_id: ::ethers::core::types::U256,
        pub data_type: ::std::string::String,
        pub price_per_hour: ::ethers::core::types::U256,
        pub min_duration: ::ethers::core::types::U256,
        pub max_duration: ::ethers::core::types::U256,
        pub is_active: bool,
        pub description: ::std::string::String,
        pub api_endpoint: ::std::string::String,
        pub created_at: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBuyerPurchases` function with signature `getBuyerPurchases(address)` and selector `0x85a34393`
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
    pub struct GetBuyerPurchasesReturn(pub ::std::vec::Vec<Purchase>);
    ///Container type for all return fields from the `getDataProduct` function with signature `getDataProduct(uint256)` and selector `0xfcc32670`
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
    pub struct GetDataProductReturn(pub DataProduct);
    ///Container type for all return fields from the `getProductPurchases` function with signature `getProductPurchases(uint256)` and selector `0xb8b650bf`
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
    pub struct GetProductPurchasesReturn(pub ::std::vec::Vec<Purchase>);
    ///Container type for all return fields from the `getStreamingPayment` function with signature `getStreamingPayment(uint256)` and selector `0x96964848`
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
    pub struct GetStreamingPaymentReturn(pub StreamingPayment);
    ///Container type for all return fields from the `getTotalProducts` function with signature `getTotalProducts()` and selector `0x570aea8a`
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
    pub struct GetTotalProductsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalStreamingPayments` function with signature `getTotalStreamingPayments()` and selector `0x14ff266e`
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
    pub struct GetTotalStreamingPaymentsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasValidAccess` function with signature `hasValidAccess(address,uint256)` and selector `0x2153e3ee`
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
    pub struct HasValidAccessReturn(pub bool, pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `platformFeeRate` function with signature `platformFeeRate()` and selector `0xeeca08f0`
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
    pub struct PlatformFeeRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `productCounter` function with signature `productCounter()` and selector `0x1f16653b`
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
    pub struct ProductCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `productPurchases` function with signature `productPurchases(uint256,uint256)` and selector `0x38bbd4e1`
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
    pub struct ProductPurchasesReturn {
        pub product_id: ::ethers::core::types::U256,
        pub buyer: ::ethers::core::types::Address,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub is_active: bool,
        pub access_token: ::std::string::String,
    }
    ///Container type for all return fields from the `streamingCounter` function with signature `streamingCounter()` and selector `0xdc3309ad`
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
    pub struct StreamingCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `streamingPayments` function with signature `streamingPayments(uint256)` and selector `0xcd56f0ac`
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
    pub struct StreamingPaymentsReturn {
        pub product_id: ::ethers::core::types::U256,
        pub buyer: ::ethers::core::types::Address,
        pub rate_per_second: ::ethers::core::types::U256,
        pub balance: ::ethers::core::types::U256,
        pub last_withdrawal: ::ethers::core::types::U256,
        pub start_time: ::ethers::core::types::U256,
        pub is_active: bool,
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
    ///`DataProduct(uint256,string,uint256,uint256,uint256,bool,string,string,uint256)`
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
    pub struct DataProduct {
        pub vehicle_id: ::ethers::core::types::U256,
        pub data_type: ::std::string::String,
        pub price_per_hour: ::ethers::core::types::U256,
        pub min_duration: ::ethers::core::types::U256,
        pub max_duration: ::ethers::core::types::U256,
        pub is_active: bool,
        pub description: ::std::string::String,
        pub api_endpoint: ::std::string::String,
        pub created_at: ::ethers::core::types::U256,
    }
    ///`Purchase(uint256,address,uint256,uint256,uint256,bool,string)`
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
    pub struct Purchase {
        pub product_id: ::ethers::core::types::U256,
        pub buyer: ::ethers::core::types::Address,
        pub start_time: ::ethers::core::types::U256,
        pub end_time: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub is_active: bool,
        pub access_token: ::std::string::String,
    }
    ///`StreamingPayment(uint256,address,uint256,uint256,uint256,uint256,bool)`
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
    pub struct StreamingPayment {
        pub product_id: ::ethers::core::types::U256,
        pub buyer: ::ethers::core::types::Address,
        pub rate_per_second: ::ethers::core::types::U256,
        pub balance: ::ethers::core::types::U256,
        pub last_withdrawal: ::ethers::core::types::U256,
        pub start_time: ::ethers::core::types::U256,
        pub is_active: bool,
    }
}
