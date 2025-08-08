pub use vehicle_registry::*;
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
pub mod vehicle_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deactivateVehicle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deactivateVehicle"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalVehicles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalVehicles"),
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
                    ::std::borrow::ToOwned::to_owned("getVehicle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVehicle"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VehicleRegistry.Vehicle",
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
                    ::std::borrow::ToOwned::to_owned("getVehicleIdByVin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVehicleIdByVin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getVehicleIdByWallet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVehicleIdByWallet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getVehicleMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVehicleMetadata"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct VehicleRegistry.VehicleMetadata",
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
                    ::std::borrow::ToOwned::to_owned("isVehicleActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isVehicleActive"),
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("registerVehicle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerVehicle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleWallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("manufacturer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("model"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("year"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataTypes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("registrationFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registrationFee"),
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
                    ::std::borrow::ToOwned::to_owned("setRegistrationFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRegistrationFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
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
                    ::std::borrow::ToOwned::to_owned("updateVehicleMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateVehicleMetadata",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("dataTypes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
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
                    ::std::borrow::ToOwned::to_owned("vehicleCounter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vehicleCounter"),
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
                    ::std::borrow::ToOwned::to_owned("vehicleMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vehicleMetadata"),
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
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastUpdate"),
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
                    ::std::borrow::ToOwned::to_owned("vehicles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vehicles"),
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
                                    name: ::std::borrow::ToOwned::to_owned("vin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("manufacturer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("model"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("year"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registrationTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("vinToVehicleId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vinToVehicleId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("walletToVehicleId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("walletToVehicleId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("withdrawFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("RegistrationFeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RegistrationFeeUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
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
                    ::std::borrow::ToOwned::to_owned("VehicleDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VehicleDeactivated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VehicleRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VehicleRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wallet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VehicleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VehicleUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vehicleId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                    ::std::borrow::ToOwned::to_owned("InsufficientFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientFee"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidVehicleData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidVehicleData"),
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
                    ::std::borrow::ToOwned::to_owned("UnauthorizedAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnauthorizedAccess"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VehicleAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VehicleAlreadyExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VehicleNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("VehicleNotFound"),
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
    pub static VEHICLEREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x80W3\x15`mW_\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3`\x01\x80Uf#\x86\xF2o\xC1\0\0`\x07Ua\x17\xBD\x90\x81a\0\x85\x829\xF3[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x14\xC4N\t\x14a\x14\xD1WP\x80c\x17\x82\x06\xF6\x14a\n\xE8W\x80c\x17\xE4C&\x14a\x14HW\x80c>\xC9\xD8M\x14a\x0B\xE7W\x80cGcC\xEE\x14a\x0BLW\x80cLY\x07\xE6\x14a\x0B\x05W\x80c^0\x81\x96\x14a\n\x15W\x80ci\x05k'\x14a\n\xE8W\x80cqP\x18\xA6\x14a\n\x91W\x80cu\x8C\xB5C\x14a\x07\xB4W\x80c\x8D\xA5\xCB[\x14a\njW\x80c\x90?|\x12\x14a\n\x15W\x80c\xB8\xBA\x95\xFA\x14a\tFW\x80c\xBA\xDF\x89\xBC\x14a\x07\xECW\x80c\xBC\x10\xC7\xE2\x14a\x07\xB4W\x80c\xC3 \xC7'\x14a\x07hW\x80c\xDF~\xBB{\x14a\x05\xDBW\x80c\xEBM\xC9\x8E\x14a\x01\xD2W\x80c\xF2\xFD\xE3\x8B\x14a\x01MWc\xFDz\xE4\x98\x14a\0\xF8W_\x80\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_R`\x05` Ra\x01?`@_ `\x02a\x01(`\x01\x83\x01a\x16\x87V[\x91\x01T`@Q\x92\x83\x92`@\x84R`@\x84\x01\x90a\x17'V[\x90` \x83\x01R\x03\x90\xF3[_\x80\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IWa\x01fa\x15\xA4V[a\x01na\x17aV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01\xBFW_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[4a\x01IW``6`\x03\x19\x01\x12a\x01IW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x02\x05\x906\x90`\x04\x01a\x15\xD1V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x02%\x906\x90`\x04\x01a\x15^V[\x91\x81\x15\x80\x15a\x05\xD0W[a\x05\xC1W_\x82\x81R`\x02` R`@\x90 `\x07\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xB2W\x81_R`\x05` R`@_ \x90\x80Q\x90`\x01`@\x1B\x82\x11a\x04\x16W\x82T\x82\x84U\x80\x83\x10a\x05;W[P` \x01\x91_R` _ \x91_\x90[\x82\x82\x10a\x04*W\x85\x85\x80_R`\x05` R`\x01`@_ \x01\x91\x80Q\x92`\x01`\x01`@\x1B\x03\x84\x11a\x04\x16Wa\x02\xBE\x81Ta\x16OV[`\x1F\x81\x11a\x03\xDBW[P` \x93`\x1F\x81\x11`\x01\x14a\x03UW\x90a\x03\x1D\x82a\x03E\x94\x93\x7F\xCA\xA4\xF1m-\xB9 R\xE3z\xF3\xB0\xBA\xD9\xF0\xB5\x89E\xC30\xE1tO\x928xd\x1F\xBB\x0E\x07\x14\x96\x97_\x91a\x03JW[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x83_R`\x05` RB`\x02`@_ \x01U`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17'V[\x03\x90\xA2\0[\x90P\x84\x01Q\x88a\x03\nV[`\x1F\x19\x81\x16\x82_R\x85_ \x90_[\x81\x81\x10a\x03\xC3WP\x91`\x01\x91\x7F\xCA\xA4\xF1m-\xB9 R\xE3z\xF3\xB0\xBA\xD9\xF0\xB5\x89E\xC30\xE1tO\x928xd\x1F\xBB\x0E\x07\x14\x96\x97\x82a\x03E\x97\x96\x95\x10a\x03\xABW[PP\x81\x1B\x01\x90Ua\x03 V[\x85\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80a\x03\x9FV[\x85\x88\x01Q\x83U` \x97\x88\x01\x97`\x01\x90\x93\x01\x92\x01a\x03cV[a\x04\x06\x90\x82_R` _ `\x1F\x87\x01`\x05\x1C\x81\x01\x91` \x88\x10a\x04\x0CW[`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x84a\x02\xC7V[\x90\x91P\x81\x90a\x03\xF9V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x04G\x86Ta\x16OV[`\x1F\x81\x11a\x05\x0BW[P` \x90`\x1F\x83\x11`\x01\x14a\x04\xA2W\x92a\x04\x88\x83`\x01\x95\x94` \x94\x87\x96_\x92a\x04\x97W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x87U[\x01\x94\x01\x91\x01\x90\x92a\x02\x8AV[\x01Q\x90P\x8C\x80a\x04tV[\x90`\x1F\x19\x83\x16\x91\x87_R\x81_ \x92_[\x81\x81\x10a\x04\xF3WP\x93` \x93`\x01\x96\x93\x87\x96\x93\x83\x88\x95\x10a\x04\xDBW[PPP\x81\x1B\x01\x87Ua\x04\x8BV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x04\xCEV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x04\xB2V[a\x055\x90\x87_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x88a\x04PV[\x83_R\x82` _ \x91\x82\x01\x91\x01[\x81\x81\x10a\x05VWPa\x02{V[\x80a\x05c`\x01\x92Ta\x16OV[\x80a\x05pW[P\x01a\x05IV[`\x1F\x81\x11\x83\x14a\x05\x85WP_\x81U[\x88a\x05iV[a\x05\xA1\x90\x82_R\x83`\x1F` _ \x92\x01`\x05\x1C\x82\x01\x91\x01a\x17KV[\x80_R_` \x81 \x81\x83UUa\x05\x7FV[c\x1A'\xEA\xC3`\xE1\x1B_R`\x04_\xFD[ch\xF2IQ`\xE0\x1B_R`\x04_\xFD[P`\x06T\x82\x11a\x02/V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_`\xE0`@Qa\x05\xFE\x81a\x14\xEBV[``\x81R\x82` \x82\x01R```@\x82\x01R``\x80\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x01R\x80\x15\x80\x15a\x07]W[a\x05\xC1W_R`\x02` Ra\x06\xFF`@_ `@Q\x90a\x06Q\x82a\x14\xEBV[a\x06Z\x81a\x16\x87V[\x82R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01\x90\x81R\x90a\x06~`\x02\x82\x01a\x16\x87V[\x90`@\x84\x01\x91\x82Ra\x06\x92`\x03\x82\x01a\x16\x87V[\x92``\x85\x01\x93\x84Ra\x074`\x04\x83\x01T\x94`\x80\x87\x01\x95\x86Ra\x07!`\xFF`\x05\x86\x01T\x16\x93`\xA0\x89\x01\x94\x15\x15\x85R`\x06\x86\x01T\x95`\xC0\x8A\x01\x96\x87R`\x07`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x96`\xE0\x8A\x01\x97\x88R`@Q\x9A\x8B\x9A` \x8CRQa\x01\0` \x8D\x01Ra\x01 \x8C\x01\x90a\x17'V[\x91Q`\x01`\x01`\xA0\x1B\x03\x16`@\x8B\x01RQ\x89\x82\x03`\x1F\x19\x01``\x8B\x01Ra\x17'V[\x90Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01Ra\x17'V[\x93Q`\xA0\x86\x01RQ\x15\x15`\xC0\x85\x01RQ`\xE0\x84\x01RQ`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x03\x90\xF3[P`\x06T\x81\x11a\x062V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW\x7F\xB1 \xD2o\xAD\x0E{\xD1(\x06\x07\x11\xA4\x84i?\x12\x83\xB8\x14;\xF9K\xDE\x9D\xF9\xB12\x17\xAAH6` `\x045a\x07\xA7a\x17aV[\x80`\x07U`@Q\x90\x81R\xA1\0[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x01`\x01`\xA0\x1B\x03a\x07\xD5a\x15\xA4V[\x16_R`\x03` R` `@_ T`@Q\x90\x81R\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_`@\x80Qa\x08\x0E\x81a\x15\x07V[``\x81R``` \x82\x01R\x01R\x80\x15\x80\x15a\t;W[a\x05\xC1W_R`\x05` R`@_ `@Q\x90a\x08@\x82a\x15\x07V[\x80Ta\x08K\x81a\x15\xBAV[\x90a\x08Y`@Q\x92\x83a\x15\"V[\x80\x82R` \x82\x01\x83_R` _ _\x91[\x83\x83\x10a\t\x1EWPPPP\x82R`\x02a\x08\x85`\x01\x83\x01a\x16\x87V[\x91` \x84\x01\x92\x83R\x01T`@\x83\x01\x90\x81R`@Q\x91` \x83R`\x80\x83\x01\x93Q\x93``` \x85\x01R\x84Q\x80\x91R`\xA0\x84\x01\x90` `\xA0\x82`\x05\x1B\x87\x01\x01\x96\x01\x91_\x90[\x82\x82\x10a\x08\xF3W\x86\x80\x87a\x08\xE8\x8B\x89Q`\x1F\x19\x85\x83\x03\x01`@\x86\x01Ra\x17'V[\x90Q``\x83\x01R\x03\x90\xF3[\x90\x91\x92\x96` \x80a\t\x10`\x01\x93`\x9F\x19\x8B\x82\x03\x01\x86R\x8BQa\x17'V[\x99\x01\x92\x01\x92\x01\x90\x92\x91a\x08\xC7V[`\x01` \x81\x92a\t-\x85a\x16\x87V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08jV[P`\x06T\x81\x11a\x08$V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_R`\x02` Ra\t\xD9`@_ a\tq\x81a\x16\x87V[\x90`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16\x90a\t\x8C`\x02\x82\x01a\x16\x87V[\x90a\t\x99`\x03\x82\x01a\x16\x87V[\x92a\t\xFA`\x04\x83\x01T\x94a\t\xEC`\xFF`\x05\x86\x01T\x16\x93`\x06\x86\x01T\x95`\x07`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x96`@Q\x9A\x8B\x9Aa\x01\0\x8CRa\x01\0\x8C\x01\x90a\x17'V[\x91` \x8B\x01R\x89\x82\x03`@\x8B\x01Ra\x17'V[\x90\x87\x82\x03``\x89\x01Ra\x17'V[\x93`\x80\x86\x01R\x15\x15`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01R\x03\x90\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01IW` \x80a\nI\x81\x936\x90`\x04\x01a\x15^V[`@Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\x04\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01IW_6`\x03\x19\x01\x12a\x01IW_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01IW_6`\x03\x19\x01\x12a\x01IWa\n\xA9a\x17aV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x82U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01IW_6`\x03\x19\x01\x12a\x01IW` `\x06T`@Q\x90\x81R\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045`\x06T\x81\x11\x15\x80a\x0B2W[` \x90`@Q\x90\x15\x15\x81R\xF3[P_R`\x02` R` `\xFF`\x05`@_ \x01T\x16a\x0B%V[4a\x01IW_6`\x03\x19\x01\x12a\x01IWa\x0Bda\x17aV[_\x80T\x81\x90\x81\x90\x81\x90G\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1=\x15a\x0B\xE2W=a\x0B\x8A\x81a\x15CV[\x90a\x0B\x98`@Q\x92\x83a\x15\"V[\x81R_` =\x92\x01>[\x15a\x0B\xA9W\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\xDA]\x1A\x19\x1C\x98]\xD8[\x08\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x90\xFD[a\x0B\xA2V[`\xE06`\x03\x19\x01\x12a\x01IW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\x12\x906\x90`\x04\x01a\x15^V[`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x01IW`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0CG\x906\x90`\x04\x01a\x15^V[\x91`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0Cg\x906\x90`\x04\x01a\x15^V[`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\x86\x906\x90`\x04\x01a\x15\xD1V[\x93`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\xA6\x906\x90`\x04\x01a\x15^V[\x94`\x02`\x01T\x14a\x149W`\x02`\x01U`\x07T4\x10a\x14+W\x84Q\x15\x80\x15a\x14#W[a\x14\x14W`@Q\x91\x85Q\x92` \x81\x81\x89\x01\x95\x80\x87\x83^\x81\x01`\x04\x81R\x03\x01\x90 Ta\x14\x05W\x84_R`\x03` R`@_ Ta\x14\x05W`\x06T_\x19\x81\x14a\x13\xF1W`\x01\x01\x93\x84`\x06U`@Q\x91a\r\x1F\x83a\x14\xEBV[\x87\x83R` \x83\x01\x91\x87\x83R`@\x84\x01\x91\x82R``\x84\x01\x90\x81R`\x80\x84\x01`\x845\x81R`\xA0\x85\x01\x91`\x01\x83R`\xC0\x86\x01\x93B\x85R`\xE0\x87\x01\x953\x87R\x8A_R`\x02` R`@_ \x97Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\r\x83\x8ATa\x16OV[`\x1F\x81\x11a\x13\xC1W[P` \x90`\x1F\x83\x11`\x01\x14a\x13^Wa\r\xBB\x92\x91_\x91\x83a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x88U[Q`\x01\x88\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UQ\x80Q`\x02\x88\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x0E\x01\x83Ta\x16OV[`\x1F\x81\x11a\x13.W[P` \x90`\x1F\x83\x11`\x01\x14a\x12\xCBWa\x0E9\x92\x91_\x91\x83a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x80Q`\x03\x87\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x0E\\\x83Ta\x16OV[`\x1F\x81\x11a\x12\x9BW[P` \x90`\x1F\x83\x11`\x01\x14a\x121W\x91\x80a\x0E\x9C\x92`\x07\x99\x98\x97\x96\x95\x94_\x92a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q`\x04\x86\x01UQ`\x05\x85\x01\x80T`\xFF\x19\x16\x91\x15\x15`\xFF\x16\x91\x90\x91\x17\x90UQ`\x06\x84\x01UQ\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@Q\x90a\x0E\xF1\x82a\x15\x07V[\x81R` \x81\x01\x95\x86R`@\x81\x01B\x81R\x83_R`\x05` R`@_ \x91Q\x80Q\x90`\x01`@\x1B\x82\x11a\x04\x16W\x83T\x82\x85U\x80\x83\x10a\x11\xBAW[P_\x84\x81R` \x80\x82 \x93\x92\x01[\x82\x82\x10a\x10\xAAWPPPP`\x01\x82\x01\x96Q\x96\x87Q`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16Wa\x0Fd\x82Ta\x16OV[`\x1F\x81\x11a\x10zW[P` `\x1F\x82\x11`\x01\x14a\x10\x11W\x81`\x02\x94\x93\x92a\x0F\xA3\x92\x89\x9A\x9B\x9C_\x92a\x04\x97WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x91\x01U` `@Q\x80\x92\x87Q\x80\x91\x83^\x81\x01`\x04\x81R\x03\x01\x90 U\x81_R`\x03` R\x80`@_ U\x7F{\x9A\x18g\x89\xF0dG#[K\xCF\x04\xF9\x918\xFC'\xF1/\xE5\x8D\xF6\x14\x9B\x99f\xFB\xD4\xD8\x01=`@Q` \x81R\x80a\x10\x083\x96` \x83\x01\x90a\x17'V[\x03\x90\xA4`\x01\x80U\0[`\x1F\x19\x82\x16\x99\x83_R\x81_ \x9A_[\x81\x81\x10a\x10bWP\x9A`\x01\x92\x84\x92`\x02\x97\x96\x95\x8B\x9C\x9D\x9E\x10a\x10JW[PPP\x81\x1B\x01\x90Ua\x0F\xA6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x10=V[\x83\x83\x01Q\x8DU`\x01\x90\x9C\x01\x9B` \x93\x84\x01\x93\x01a\x10 V[a\x10\xA4\x90\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x89a\x0FmV[\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x10\xC7\x86Ta\x16OV[`\x1F\x81\x11a\x11\x8AW[P` \x90`\x1F\x83\x11`\x01\x14a\x11!W\x92a\x11\x07\x83`\x01\x95\x94` \x94\x87\x96_\x92a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x87U[\x01\x94\x01\x91\x01\x90\x92a\x0F8V[\x01Q\x90P_\x80a\x04tV[\x90`\x1F\x19\x83\x16\x91\x87_R\x81_ \x92_[\x81\x81\x10a\x11rWP\x93` \x93`\x01\x96\x93\x87\x96\x93\x83\x88\x95\x10a\x11ZW[PPP\x81\x1B\x01\x87Ua\x11\nV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x11MV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x111V[a\x11\xB4\x90\x87_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Da\x10\xD0V[\x84_R\x82` _ \x91\x82\x01\x91\x01[\x81\x81\x10a\x11\xD5WPa\x0F*V[\x80a\x11\xE2`\x01\x92Ta\x16OV[\x80a\x11\xEFW[P\x01a\x11\xC8V[`\x1F\x81\x11\x83\x14a\x12\x04WP_\x81U[\x8Ca\x11\xE8V[a\x12 \x90\x82_R\x83`\x1F` _ \x92\x01`\x05\x1C\x82\x01\x91\x01a\x17KV[\x80_R_` \x81 \x81\x83UUa\x11\xFEV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x12\x83WP\x91`\x01\x93\x91\x85`\x07\x9B\x9A\x99\x98\x97\x96\x94\x10a\x12kW[PPP\x81\x1B\x01\x90Ua\x0E\x9FV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8F\x80\x80a\x12^V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x12AV[a\x12\xC5\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Ea\x0EeV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x13\x16WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x12\xFEW[PPP\x81\x1B\x01\x90Ua\x0E<V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8F\x80\x80a\x12\xF1V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x12\xDBV[a\x13X\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Fa\x0E\nV[\x90`\x1F\x19\x83\x16\x91\x8B_R\x81_ \x92_[\x81\x81\x10a\x13\xA9WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x13\x91W[PPP\x81\x1B\x01\x88Ua\r\xBEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x13\x84V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x13nV[a\x13\xEB\x90\x8B_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[_a\r\x8CV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xAB\xEC\xBC\xA9`\xE0\x1B_R`\x04_\xFD[cc\xBEme`\xE1\x1B_R`\x04_\xFD[P\x83\x15a\x0C\xC9V[b\x97ou`\xE2\x1B_R`\x04_\xFD[c>\xE5\xAE\xB5`\xE0\x1B_R`\x04_\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045\x80\x15\x80\x15a\x14\xC6W[a\x05\xC1W_\x81\x81R`\x02` R`@\x90 `\x07\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xB2W\x80_R`\x02` R`\x05`@_ \x01`\xFF\x19\x81T\x16\x90U\x7F\x95\xE3,a\x1D\xD3\x8E\x85\xB4\xC70\xBF\xAB\x17tp \\\xB2\xDA\xE2\x9E\xECQA\xDE;\xF20\xBC\xD6(_\x80\xA2\0[P`\x06T\x81\x11a\x14eV[4a\x01IW_6`\x03\x19\x01\x12a\x01IW` \x90`\x07T\x81R\xF3[a\x01\0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01IW\x805\x90a\x15u\x82a\x15CV[\x92a\x15\x83`@Q\x94\x85a\x15\"V[\x82\x84R` \x83\x83\x01\x01\x11a\x01IW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01IWV[`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01IW\x815a\x15\xE8\x81a\x15\xBAV[\x92a\x15\xF6`@Q\x94\x85a\x15\"V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x01IW` \x82\x01\x90[\x83\x82\x10a\x16\"WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01IW` \x91a\x16D\x87\x84\x80\x94\x88\x01\x01a\x15^V[\x81R\x01\x91\x01\x90a\x16\x13V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x16}W[` \x83\x10\x14a\x16iWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x16^V[\x90`@Q\x91\x82_\x82T\x92a\x16\x9A\x84a\x16OV[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x17\x05WP`\x01\x14a\x16\xC1W[Pa\x16\xBF\x92P\x03\x83a\x15\"V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x16\xE9WPP\x90` a\x16\xBF\x92\x82\x01\x01_a\x16\xB2V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x16\xD0V[\x90P` \x92Pa\x16\xBF\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x16\xB2V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x81\x81\x10a\x17VWPPV[_\x81U`\x01\x01a\x17KV[_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17tWV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x06.r.\xF9\xD1\xA5\xD2(]\xFA\r\x16><,\xEA\x0B\x89\xA0)\xAE\xADmQv\0\xE5=inidsolcC\0\x08\x1E\x003";
    /// The bytecode of the contract.
    pub static VEHICLEREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x14\xC4N\t\x14a\x14\xD1WP\x80c\x17\x82\x06\xF6\x14a\n\xE8W\x80c\x17\xE4C&\x14a\x14HW\x80c>\xC9\xD8M\x14a\x0B\xE7W\x80cGcC\xEE\x14a\x0BLW\x80cLY\x07\xE6\x14a\x0B\x05W\x80c^0\x81\x96\x14a\n\x15W\x80ci\x05k'\x14a\n\xE8W\x80cqP\x18\xA6\x14a\n\x91W\x80cu\x8C\xB5C\x14a\x07\xB4W\x80c\x8D\xA5\xCB[\x14a\njW\x80c\x90?|\x12\x14a\n\x15W\x80c\xB8\xBA\x95\xFA\x14a\tFW\x80c\xBA\xDF\x89\xBC\x14a\x07\xECW\x80c\xBC\x10\xC7\xE2\x14a\x07\xB4W\x80c\xC3 \xC7'\x14a\x07hW\x80c\xDF~\xBB{\x14a\x05\xDBW\x80c\xEBM\xC9\x8E\x14a\x01\xD2W\x80c\xF2\xFD\xE3\x8B\x14a\x01MWc\xFDz\xE4\x98\x14a\0\xF8W_\x80\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_R`\x05` Ra\x01?`@_ `\x02a\x01(`\x01\x83\x01a\x16\x87V[\x91\x01T`@Q\x92\x83\x92`@\x84R`@\x84\x01\x90a\x17'V[\x90` \x83\x01R\x03\x90\xF3[_\x80\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IWa\x01fa\x15\xA4V[a\x01na\x17aV[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x01\xBFW_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x83\x17\x82U`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\0[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[4a\x01IW``6`\x03\x19\x01\x12a\x01IW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x02\x05\x906\x90`\x04\x01a\x15\xD1V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x02%\x906\x90`\x04\x01a\x15^V[\x91\x81\x15\x80\x15a\x05\xD0W[a\x05\xC1W_\x82\x81R`\x02` R`@\x90 `\x07\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xB2W\x81_R`\x05` R`@_ \x90\x80Q\x90`\x01`@\x1B\x82\x11a\x04\x16W\x82T\x82\x84U\x80\x83\x10a\x05;W[P` \x01\x91_R` _ \x91_\x90[\x82\x82\x10a\x04*W\x85\x85\x80_R`\x05` R`\x01`@_ \x01\x91\x80Q\x92`\x01`\x01`@\x1B\x03\x84\x11a\x04\x16Wa\x02\xBE\x81Ta\x16OV[`\x1F\x81\x11a\x03\xDBW[P` \x93`\x1F\x81\x11`\x01\x14a\x03UW\x90a\x03\x1D\x82a\x03E\x94\x93\x7F\xCA\xA4\xF1m-\xB9 R\xE3z\xF3\xB0\xBA\xD9\xF0\xB5\x89E\xC30\xE1tO\x928xd\x1F\xBB\x0E\x07\x14\x96\x97_\x91a\x03JW[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x83_R`\x05` RB`\x02`@_ \x01U`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x17'V[\x03\x90\xA2\0[\x90P\x84\x01Q\x88a\x03\nV[`\x1F\x19\x81\x16\x82_R\x85_ \x90_[\x81\x81\x10a\x03\xC3WP\x91`\x01\x91\x7F\xCA\xA4\xF1m-\xB9 R\xE3z\xF3\xB0\xBA\xD9\xF0\xB5\x89E\xC30\xE1tO\x928xd\x1F\xBB\x0E\x07\x14\x96\x97\x82a\x03E\x97\x96\x95\x10a\x03\xABW[PP\x81\x1B\x01\x90Ua\x03 V[\x85\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80a\x03\x9FV[\x85\x88\x01Q\x83U` \x97\x88\x01\x97`\x01\x90\x93\x01\x92\x01a\x03cV[a\x04\x06\x90\x82_R` _ `\x1F\x87\x01`\x05\x1C\x81\x01\x91` \x88\x10a\x04\x0CW[`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x84a\x02\xC7V[\x90\x91P\x81\x90a\x03\xF9V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x04G\x86Ta\x16OV[`\x1F\x81\x11a\x05\x0BW[P` \x90`\x1F\x83\x11`\x01\x14a\x04\xA2W\x92a\x04\x88\x83`\x01\x95\x94` \x94\x87\x96_\x92a\x04\x97W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x87U[\x01\x94\x01\x91\x01\x90\x92a\x02\x8AV[\x01Q\x90P\x8C\x80a\x04tV[\x90`\x1F\x19\x83\x16\x91\x87_R\x81_ \x92_[\x81\x81\x10a\x04\xF3WP\x93` \x93`\x01\x96\x93\x87\x96\x93\x83\x88\x95\x10a\x04\xDBW[PPP\x81\x1B\x01\x87Ua\x04\x8BV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x04\xCEV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x04\xB2V[a\x055\x90\x87_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x88a\x04PV[\x83_R\x82` _ \x91\x82\x01\x91\x01[\x81\x81\x10a\x05VWPa\x02{V[\x80a\x05c`\x01\x92Ta\x16OV[\x80a\x05pW[P\x01a\x05IV[`\x1F\x81\x11\x83\x14a\x05\x85WP_\x81U[\x88a\x05iV[a\x05\xA1\x90\x82_R\x83`\x1F` _ \x92\x01`\x05\x1C\x82\x01\x91\x01a\x17KV[\x80_R_` \x81 \x81\x83UUa\x05\x7FV[c\x1A'\xEA\xC3`\xE1\x1B_R`\x04_\xFD[ch\xF2IQ`\xE0\x1B_R`\x04_\xFD[P`\x06T\x82\x11a\x02/V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_`\xE0`@Qa\x05\xFE\x81a\x14\xEBV[``\x81R\x82` \x82\x01R```@\x82\x01R``\x80\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x01R\x80\x15\x80\x15a\x07]W[a\x05\xC1W_R`\x02` Ra\x06\xFF`@_ `@Q\x90a\x06Q\x82a\x14\xEBV[a\x06Z\x81a\x16\x87V[\x82R`\x01\x81\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01\x90\x81R\x90a\x06~`\x02\x82\x01a\x16\x87V[\x90`@\x84\x01\x91\x82Ra\x06\x92`\x03\x82\x01a\x16\x87V[\x92``\x85\x01\x93\x84Ra\x074`\x04\x83\x01T\x94`\x80\x87\x01\x95\x86Ra\x07!`\xFF`\x05\x86\x01T\x16\x93`\xA0\x89\x01\x94\x15\x15\x85R`\x06\x86\x01T\x95`\xC0\x8A\x01\x96\x87R`\x07`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x96`\xE0\x8A\x01\x97\x88R`@Q\x9A\x8B\x9A` \x8CRQa\x01\0` \x8D\x01Ra\x01 \x8C\x01\x90a\x17'V[\x91Q`\x01`\x01`\xA0\x1B\x03\x16`@\x8B\x01RQ\x89\x82\x03`\x1F\x19\x01``\x8B\x01Ra\x17'V[\x90Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01Ra\x17'V[\x93Q`\xA0\x86\x01RQ\x15\x15`\xC0\x85\x01RQ`\xE0\x84\x01RQ`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x03\x90\xF3[P`\x06T\x81\x11a\x062V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW\x7F\xB1 \xD2o\xAD\x0E{\xD1(\x06\x07\x11\xA4\x84i?\x12\x83\xB8\x14;\xF9K\xDE\x9D\xF9\xB12\x17\xAAH6` `\x045a\x07\xA7a\x17aV[\x80`\x07U`@Q\x90\x81R\xA1\0[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x01`\x01`\xA0\x1B\x03a\x07\xD5a\x15\xA4V[\x16_R`\x03` R` `@_ T`@Q\x90\x81R\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_`@\x80Qa\x08\x0E\x81a\x15\x07V[``\x81R``` \x82\x01R\x01R\x80\x15\x80\x15a\t;W[a\x05\xC1W_R`\x05` R`@_ `@Q\x90a\x08@\x82a\x15\x07V[\x80Ta\x08K\x81a\x15\xBAV[\x90a\x08Y`@Q\x92\x83a\x15\"V[\x80\x82R` \x82\x01\x83_R` _ _\x91[\x83\x83\x10a\t\x1EWPPPP\x82R`\x02a\x08\x85`\x01\x83\x01a\x16\x87V[\x91` \x84\x01\x92\x83R\x01T`@\x83\x01\x90\x81R`@Q\x91` \x83R`\x80\x83\x01\x93Q\x93``` \x85\x01R\x84Q\x80\x91R`\xA0\x84\x01\x90` `\xA0\x82`\x05\x1B\x87\x01\x01\x96\x01\x91_\x90[\x82\x82\x10a\x08\xF3W\x86\x80\x87a\x08\xE8\x8B\x89Q`\x1F\x19\x85\x83\x03\x01`@\x86\x01Ra\x17'V[\x90Q``\x83\x01R\x03\x90\xF3[\x90\x91\x92\x96` \x80a\t\x10`\x01\x93`\x9F\x19\x8B\x82\x03\x01\x86R\x8BQa\x17'V[\x99\x01\x92\x01\x92\x01\x90\x92\x91a\x08\xC7V[`\x01` \x81\x92a\t-\x85a\x16\x87V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x08jV[P`\x06T\x81\x11a\x08$V[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045_R`\x02` Ra\t\xD9`@_ a\tq\x81a\x16\x87V[\x90`\x01\x80`\xA0\x1B\x03`\x01\x82\x01T\x16\x90a\t\x8C`\x02\x82\x01a\x16\x87V[\x90a\t\x99`\x03\x82\x01a\x16\x87V[\x92a\t\xFA`\x04\x83\x01T\x94a\t\xEC`\xFF`\x05\x86\x01T\x16\x93`\x06\x86\x01T\x95`\x07`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x96`@Q\x9A\x8B\x9Aa\x01\0\x8CRa\x01\0\x8C\x01\x90a\x17'V[\x91` \x8B\x01R\x89\x82\x03`@\x8B\x01Ra\x17'V[\x90\x87\x82\x03``\x89\x01Ra\x17'V[\x93`\x80\x86\x01R\x15\x15`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01R\x03\x90\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01IW` \x80a\nI\x81\x936\x90`\x04\x01a\x15^V[`@Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\x04\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x01IW_6`\x03\x19\x01\x12a\x01IW_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01IW_6`\x03\x19\x01\x12a\x01IWa\n\xA9a\x17aV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x82U`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01IW_6`\x03\x19\x01\x12a\x01IW` `\x06T`@Q\x90\x81R\xF3[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045`\x06T\x81\x11\x15\x80a\x0B2W[` \x90`@Q\x90\x15\x15\x81R\xF3[P_R`\x02` R` `\xFF`\x05`@_ \x01T\x16a\x0B%V[4a\x01IW_6`\x03\x19\x01\x12a\x01IWa\x0Bda\x17aV[_\x80T\x81\x90\x81\x90\x81\x90G\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1=\x15a\x0B\xE2W=a\x0B\x8A\x81a\x15CV[\x90a\x0B\x98`@Q\x92\x83a\x15\"V[\x81R_` =\x92\x01>[\x15a\x0B\xA9W\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x15\xDA]\x1A\x19\x1C\x98]\xD8[\x08\x19\x98Z[\x19Y`z\x1B`D\x82\x01R`d\x90\xFD[a\x0B\xA2V[`\xE06`\x03\x19\x01\x12a\x01IW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\x12\x906\x90`\x04\x01a\x15^V[`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x01IW`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0CG\x906\x90`\x04\x01a\x15^V[\x91`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0Cg\x906\x90`\x04\x01a\x15^V[`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\x86\x906\x90`\x04\x01a\x15\xD1V[\x93`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\x01IWa\x0C\xA6\x906\x90`\x04\x01a\x15^V[\x94`\x02`\x01T\x14a\x149W`\x02`\x01U`\x07T4\x10a\x14+W\x84Q\x15\x80\x15a\x14#W[a\x14\x14W`@Q\x91\x85Q\x92` \x81\x81\x89\x01\x95\x80\x87\x83^\x81\x01`\x04\x81R\x03\x01\x90 Ta\x14\x05W\x84_R`\x03` R`@_ Ta\x14\x05W`\x06T_\x19\x81\x14a\x13\xF1W`\x01\x01\x93\x84`\x06U`@Q\x91a\r\x1F\x83a\x14\xEBV[\x87\x83R` \x83\x01\x91\x87\x83R`@\x84\x01\x91\x82R``\x84\x01\x90\x81R`\x80\x84\x01`\x845\x81R`\xA0\x85\x01\x91`\x01\x83R`\xC0\x86\x01\x93B\x85R`\xE0\x87\x01\x953\x87R\x8A_R`\x02` R`@_ \x97Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\r\x83\x8ATa\x16OV[`\x1F\x81\x11a\x13\xC1W[P` \x90`\x1F\x83\x11`\x01\x14a\x13^Wa\r\xBB\x92\x91_\x91\x83a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x88U[Q`\x01\x88\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UQ\x80Q`\x02\x88\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x0E\x01\x83Ta\x16OV[`\x1F\x81\x11a\x13.W[P` \x90`\x1F\x83\x11`\x01\x14a\x12\xCBWa\x0E9\x92\x91_\x91\x83a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x80Q`\x03\x87\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x0E\\\x83Ta\x16OV[`\x1F\x81\x11a\x12\x9BW[P` \x90`\x1F\x83\x11`\x01\x14a\x121W\x91\x80a\x0E\x9C\x92`\x07\x99\x98\x97\x96\x95\x94_\x92a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q`\x04\x86\x01UQ`\x05\x85\x01\x80T`\xFF\x19\x16\x91\x15\x15`\xFF\x16\x91\x90\x91\x17\x90UQ`\x06\x84\x01UQ\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@Q\x90a\x0E\xF1\x82a\x15\x07V[\x81R` \x81\x01\x95\x86R`@\x81\x01B\x81R\x83_R`\x05` R`@_ \x91Q\x80Q\x90`\x01`@\x1B\x82\x11a\x04\x16W\x83T\x82\x85U\x80\x83\x10a\x11\xBAW[P_\x84\x81R` \x80\x82 \x93\x92\x01[\x82\x82\x10a\x10\xAAWPPPP`\x01\x82\x01\x96Q\x96\x87Q`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16Wa\x0Fd\x82Ta\x16OV[`\x1F\x81\x11a\x10zW[P` `\x1F\x82\x11`\x01\x14a\x10\x11W\x81`\x02\x94\x93\x92a\x0F\xA3\x92\x89\x9A\x9B\x9C_\x92a\x04\x97WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x91\x01U` `@Q\x80\x92\x87Q\x80\x91\x83^\x81\x01`\x04\x81R\x03\x01\x90 U\x81_R`\x03` R\x80`@_ U\x7F{\x9A\x18g\x89\xF0dG#[K\xCF\x04\xF9\x918\xFC'\xF1/\xE5\x8D\xF6\x14\x9B\x99f\xFB\xD4\xD8\x01=`@Q` \x81R\x80a\x10\x083\x96` \x83\x01\x90a\x17'V[\x03\x90\xA4`\x01\x80U\0[`\x1F\x19\x82\x16\x99\x83_R\x81_ \x9A_[\x81\x81\x10a\x10bWP\x9A`\x01\x92\x84\x92`\x02\x97\x96\x95\x8B\x9C\x9D\x9E\x10a\x10JW[PPP\x81\x1B\x01\x90Ua\x0F\xA6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8B\x80\x80a\x10=V[\x83\x83\x01Q\x8DU`\x01\x90\x9C\x01\x9B` \x93\x84\x01\x93\x01a\x10 V[a\x10\xA4\x90\x83_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01\x91` \x85\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x89a\x0FmV[\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x04\x16Wa\x10\xC7\x86Ta\x16OV[`\x1F\x81\x11a\x11\x8AW[P` \x90`\x1F\x83\x11`\x01\x14a\x11!W\x92a\x11\x07\x83`\x01\x95\x94` \x94\x87\x96_\x92a\x11\x16WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x87U[\x01\x94\x01\x91\x01\x90\x92a\x0F8V[\x01Q\x90P_\x80a\x04tV[\x90`\x1F\x19\x83\x16\x91\x87_R\x81_ \x92_[\x81\x81\x10a\x11rWP\x93` \x93`\x01\x96\x93\x87\x96\x93\x83\x88\x95\x10a\x11ZW[PPP\x81\x1B\x01\x87Ua\x11\nV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x11MV[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x111V[a\x11\xB4\x90\x87_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Da\x10\xD0V[\x84_R\x82` _ \x91\x82\x01\x91\x01[\x81\x81\x10a\x11\xD5WPa\x0F*V[\x80a\x11\xE2`\x01\x92Ta\x16OV[\x80a\x11\xEFW[P\x01a\x11\xC8V[`\x1F\x81\x11\x83\x14a\x12\x04WP_\x81U[\x8Ca\x11\xE8V[a\x12 \x90\x82_R\x83`\x1F` _ \x92\x01`\x05\x1C\x82\x01\x91\x01a\x17KV[\x80_R_` \x81 \x81\x83UUa\x11\xFEV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x12\x83WP\x91`\x01\x93\x91\x85`\x07\x9B\x9A\x99\x98\x97\x96\x94\x10a\x12kW[PPP\x81\x1B\x01\x90Ua\x0E\x9FV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8F\x80\x80a\x12^V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x12AV[a\x12\xC5\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Ea\x0EeV[\x90`\x1F\x19\x83\x16\x91\x84_R\x81_ \x92_[\x81\x81\x10a\x13\x16WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x12\xFEW[PPP\x81\x1B\x01\x90Ua\x0E<V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8F\x80\x80a\x12\xF1V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x12\xDBV[a\x13X\x90\x84_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[\x8Fa\x0E\nV[\x90`\x1F\x19\x83\x16\x91\x8B_R\x81_ \x92_[\x81\x81\x10a\x13\xA9WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x13\x91W[PPP\x81\x1B\x01\x88Ua\r\xBEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x13\x84V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x13nV[a\x13\xEB\x90\x8B_R` _ `\x1F\x85\x01`\x05\x1C\x81\x01\x91` \x86\x10a\x04\x0CW`\x1F\x01`\x05\x1C\x01\x90a\x17KV[_a\r\x8CV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xAB\xEC\xBC\xA9`\xE0\x1B_R`\x04_\xFD[cc\xBEme`\xE1\x1B_R`\x04_\xFD[P\x83\x15a\x0C\xC9V[b\x97ou`\xE2\x1B_R`\x04_\xFD[c>\xE5\xAE\xB5`\xE0\x1B_R`\x04_\xFD[4a\x01IW` 6`\x03\x19\x01\x12a\x01IW`\x045\x80\x15\x80\x15a\x14\xC6W[a\x05\xC1W_\x81\x81R`\x02` R`@\x90 `\x07\x01T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xB2W\x80_R`\x02` R`\x05`@_ \x01`\xFF\x19\x81T\x16\x90U\x7F\x95\xE3,a\x1D\xD3\x8E\x85\xB4\xC70\xBF\xAB\x17tp \\\xB2\xDA\xE2\x9E\xECQA\xDE;\xF20\xBC\xD6(_\x80\xA2\0[P`\x06T\x81\x11a\x14eV[4a\x01IW_6`\x03\x19\x01\x12a\x01IW` \x90`\x07T\x81R\xF3[a\x01\0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x04\x16W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01IW\x805\x90a\x15u\x82a\x15CV[\x92a\x15\x83`@Q\x94\x85a\x15\"V[\x82\x84R` \x83\x83\x01\x01\x11a\x01IW\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01IWV[`\x01`\x01`@\x1B\x03\x81\x11a\x04\x16W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01IW\x815a\x15\xE8\x81a\x15\xBAV[\x92a\x15\xF6`@Q\x94\x85a\x15\"V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x01IW` \x82\x01\x90[\x83\x82\x10a\x16\"WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x01IW` \x91a\x16D\x87\x84\x80\x94\x88\x01\x01a\x15^V[\x81R\x01\x91\x01\x90a\x16\x13V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x16}W[` \x83\x10\x14a\x16iWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x16^V[\x90`@Q\x91\x82_\x82T\x92a\x16\x9A\x84a\x16OV[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x17\x05WP`\x01\x14a\x16\xC1W[Pa\x16\xBF\x92P\x03\x83a\x15\"V[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x16\xE9WPP\x90` a\x16\xBF\x92\x82\x01\x01_a\x16\xB2V[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x16\xD0V[\x90P` \x92Pa\x16\xBF\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x16\xB2V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x81\x81\x10a\x17VWPPV[_\x81U`\x01\x01a\x17KV[_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17tWV[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x06.r.\xF9\xD1\xA5\xD2(]\xFA\r\x16><,\xEA\x0B\x89\xA0)\xAE\xADmQv\0\xE5=inidsolcC\0\x08\x1E\x003";
    /// The deployed bytecode of the contract.
    pub static VEHICLEREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct VehicleRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VehicleRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VehicleRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VehicleRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VehicleRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VehicleRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VehicleRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VEHICLEREGISTRY_ABI.clone(),
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
                VEHICLEREGISTRY_ABI.clone(),
                VEHICLEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deactivateVehicle` (0x17e44326) function
        pub fn deactivate_vehicle(
            &self,
            vehicle_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 228, 67, 38], vehicle_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalVehicles` (0x69056b27) function
        pub fn get_total_vehicles(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 5, 107, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVehicle` (0xdf7ebb7b) function
        pub fn get_vehicle(
            &self,
            vehicle_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Vehicle> {
            self.0
                .method_hash([223, 126, 187, 123], vehicle_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVehicleIdByVin` (0x5e308196) function
        pub fn get_vehicle_id_by_vin(
            &self,
            vin: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 48, 129, 150], vin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVehicleIdByWallet` (0x758cb543) function
        pub fn get_vehicle_id_by_wallet(
            &self,
            wallet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 140, 181, 67], wallet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVehicleMetadata` (0xbadf89bc) function
        pub fn get_vehicle_metadata(
            &self,
            vehicle_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, VehicleMetadata> {
            self.0
                .method_hash([186, 223, 137, 188], vehicle_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isVehicleActive` (0x4c5907e6) function
        pub fn is_vehicle_active(
            &self,
            vehicle_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 89, 7, 230], vehicle_id)
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
        ///Calls the contract's `registerVehicle` (0x3ec9d84d) function
        pub fn register_vehicle(
            &self,
            vin: ::std::string::String,
            vehicle_wallet: ::ethers::core::types::Address,
            manufacturer: ::std::string::String,
            model: ::std::string::String,
            year: ::ethers::core::types::U256,
            data_types: ::std::vec::Vec<::std::string::String>,
            ipfs_hash: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [62, 201, 216, 77],
                    (
                        vin,
                        vehicle_wallet,
                        manufacturer,
                        model,
                        year,
                        data_types,
                        ipfs_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registrationFee` (0x14c44e09) function
        pub fn registration_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 196, 78, 9], ())
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
        ///Calls the contract's `setRegistrationFee` (0xc320c727) function
        pub fn set_registration_fee(
            &self,
            new_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 32, 199, 39], new_fee)
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
        ///Calls the contract's `updateVehicleMetadata` (0xeb4dc98e) function
        pub fn update_vehicle_metadata(
            &self,
            vehicle_id: ::ethers::core::types::U256,
            data_types: ::std::vec::Vec<::std::string::String>,
            ipfs_hash: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 77, 201, 142], (vehicle_id, data_types, ipfs_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vehicleCounter` (0x178206f6) function
        pub fn vehicle_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([23, 130, 6, 246], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vehicleMetadata` (0xfd7ae498) function
        pub fn vehicle_metadata(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::string::String, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([253, 122, 228, 152], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vehicles` (0xb8ba95fa) function
        pub fn vehicles(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::string::String,
                ::ethers::core::types::Address,
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([184, 186, 149, 250], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vinToVehicleId` (0x903f7c12) function
        pub fn vin_to_vehicle_id(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 63, 124, 18], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `walletToVehicleId` (0xbc10c7e2) function
        pub fn wallet_to_vehicle_id(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 16, 199, 226], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFees` (0x476343ee) function
        pub fn withdraw_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 99, 67, 238], ())
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `RegistrationFeeUpdated` event
        pub fn registration_fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegistrationFeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VehicleDeactivated` event
        pub fn vehicle_deactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VehicleDeactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VehicleRegistered` event
        pub fn vehicle_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VehicleRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VehicleUpdated` event
        pub fn vehicle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VehicleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VehicleRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for VehicleRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientFee` with signature `InsufficientFee()` and selector `0x025dbdd4`
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
    #[etherror(name = "InsufficientFee", abi = "InsufficientFee()")]
    pub struct InsufficientFee;
    ///Custom Error type `InvalidVehicleData` with signature `InvalidVehicleData()` and selector `0xc77cdaca`
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
    #[etherror(name = "InvalidVehicleData", abi = "InvalidVehicleData()")]
    pub struct InvalidVehicleData;
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
    ///Custom Error type `VehicleAlreadyExists` with signature `VehicleAlreadyExists()` and selector `0xabecbca9`
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
    #[etherror(name = "VehicleAlreadyExists", abi = "VehicleAlreadyExists()")]
    pub struct VehicleAlreadyExists;
    ///Custom Error type `VehicleNotFound` with signature `VehicleNotFound()` and selector `0x68f24951`
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
    #[etherror(name = "VehicleNotFound", abi = "VehicleNotFound()")]
    pub struct VehicleNotFound;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VehicleRegistryErrors {
        InsufficientFee(InsufficientFee),
        InvalidVehicleData(InvalidVehicleData),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        UnauthorizedAccess(UnauthorizedAccess),
        VehicleAlreadyExists(VehicleAlreadyExists),
        VehicleNotFound(VehicleNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for VehicleRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InsufficientFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientFee(decoded));
            }
            if let Ok(decoded) = <InvalidVehicleData as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidVehicleData(decoded));
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
            if let Ok(decoded) = <UnauthorizedAccess as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnauthorizedAccess(decoded));
            }
            if let Ok(decoded) = <VehicleAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleAlreadyExists(decoded));
            }
            if let Ok(decoded) = <VehicleNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleNotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VehicleRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidVehicleData(element) => {
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
                Self::UnauthorizedAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for VehicleRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidVehicleData as ::ethers::contract::EthError>::selector() => {
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
                    == <UnauthorizedAccess as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <VehicleAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <VehicleNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for VehicleRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidVehicleData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReentrancyGuardReentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnauthorizedAccess(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for VehicleRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientFee> for VehicleRegistryErrors {
        fn from(value: InsufficientFee) -> Self {
            Self::InsufficientFee(value)
        }
    }
    impl ::core::convert::From<InvalidVehicleData> for VehicleRegistryErrors {
        fn from(value: InvalidVehicleData) -> Self {
            Self::InvalidVehicleData(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for VehicleRegistryErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for VehicleRegistryErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardReentrantCall> for VehicleRegistryErrors {
        fn from(value: ReentrancyGuardReentrantCall) -> Self {
            Self::ReentrancyGuardReentrantCall(value)
        }
    }
    impl ::core::convert::From<UnauthorizedAccess> for VehicleRegistryErrors {
        fn from(value: UnauthorizedAccess) -> Self {
            Self::UnauthorizedAccess(value)
        }
    }
    impl ::core::convert::From<VehicleAlreadyExists> for VehicleRegistryErrors {
        fn from(value: VehicleAlreadyExists) -> Self {
            Self::VehicleAlreadyExists(value)
        }
    }
    impl ::core::convert::From<VehicleNotFound> for VehicleRegistryErrors {
        fn from(value: VehicleNotFound) -> Self {
            Self::VehicleNotFound(value)
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
    #[ethevent(name = "RegistrationFeeUpdated", abi = "RegistrationFeeUpdated(uint256)")]
    pub struct RegistrationFeeUpdatedFilter {
        pub new_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "VehicleDeactivated", abi = "VehicleDeactivated(uint256)")]
    pub struct VehicleDeactivatedFilter {
        #[ethevent(indexed)]
        pub vehicle_id: ::ethers::core::types::U256,
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
        name = "VehicleRegistered",
        abi = "VehicleRegistered(uint256,string,address,address)"
    )]
    pub struct VehicleRegisteredFilter {
        #[ethevent(indexed)]
        pub vehicle_id: ::ethers::core::types::U256,
        pub vin: ::std::string::String,
        #[ethevent(indexed)]
        pub wallet: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "VehicleUpdated", abi = "VehicleUpdated(uint256,string)")]
    pub struct VehicleUpdatedFilter {
        #[ethevent(indexed)]
        pub vehicle_id: ::ethers::core::types::U256,
        pub ipfs_hash: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VehicleRegistryEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RegistrationFeeUpdatedFilter(RegistrationFeeUpdatedFilter),
        VehicleDeactivatedFilter(VehicleDeactivatedFilter),
        VehicleRegisteredFilter(VehicleRegisteredFilter),
        VehicleUpdatedFilter(VehicleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for VehicleRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VehicleRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RegistrationFeeUpdatedFilter::decode_log(log) {
                return Ok(VehicleRegistryEvents::RegistrationFeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = VehicleDeactivatedFilter::decode_log(log) {
                return Ok(VehicleRegistryEvents::VehicleDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = VehicleRegisteredFilter::decode_log(log) {
                return Ok(VehicleRegistryEvents::VehicleRegisteredFilter(decoded));
            }
            if let Ok(decoded) = VehicleUpdatedFilter::decode_log(log) {
                return Ok(VehicleRegistryEvents::VehicleUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VehicleRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegistrationFeeUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleDeactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for VehicleRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RegistrationFeeUpdatedFilter> for VehicleRegistryEvents {
        fn from(value: RegistrationFeeUpdatedFilter) -> Self {
            Self::RegistrationFeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<VehicleDeactivatedFilter> for VehicleRegistryEvents {
        fn from(value: VehicleDeactivatedFilter) -> Self {
            Self::VehicleDeactivatedFilter(value)
        }
    }
    impl ::core::convert::From<VehicleRegisteredFilter> for VehicleRegistryEvents {
        fn from(value: VehicleRegisteredFilter) -> Self {
            Self::VehicleRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<VehicleUpdatedFilter> for VehicleRegistryEvents {
        fn from(value: VehicleUpdatedFilter) -> Self {
            Self::VehicleUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `deactivateVehicle` function with signature `deactivateVehicle(uint256)` and selector `0x17e44326`
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
    #[ethcall(name = "deactivateVehicle", abi = "deactivateVehicle(uint256)")]
    pub struct DeactivateVehicleCall {
        pub vehicle_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalVehicles` function with signature `getTotalVehicles()` and selector `0x69056b27`
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
    #[ethcall(name = "getTotalVehicles", abi = "getTotalVehicles()")]
    pub struct GetTotalVehiclesCall;
    ///Container type for all input parameters for the `getVehicle` function with signature `getVehicle(uint256)` and selector `0xdf7ebb7b`
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
    #[ethcall(name = "getVehicle", abi = "getVehicle(uint256)")]
    pub struct GetVehicleCall {
        pub vehicle_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVehicleIdByVin` function with signature `getVehicleIdByVin(string)` and selector `0x5e308196`
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
    #[ethcall(name = "getVehicleIdByVin", abi = "getVehicleIdByVin(string)")]
    pub struct GetVehicleIdByVinCall {
        pub vin: ::std::string::String,
    }
    ///Container type for all input parameters for the `getVehicleIdByWallet` function with signature `getVehicleIdByWallet(address)` and selector `0x758cb543`
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
    #[ethcall(name = "getVehicleIdByWallet", abi = "getVehicleIdByWallet(address)")]
    pub struct GetVehicleIdByWalletCall {
        pub wallet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getVehicleMetadata` function with signature `getVehicleMetadata(uint256)` and selector `0xbadf89bc`
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
    #[ethcall(name = "getVehicleMetadata", abi = "getVehicleMetadata(uint256)")]
    pub struct GetVehicleMetadataCall {
        pub vehicle_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isVehicleActive` function with signature `isVehicleActive(uint256)` and selector `0x4c5907e6`
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
    #[ethcall(name = "isVehicleActive", abi = "isVehicleActive(uint256)")]
    pub struct IsVehicleActiveCall {
        pub vehicle_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `registerVehicle` function with signature `registerVehicle(string,address,string,string,uint256,string[],string)` and selector `0x3ec9d84d`
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
        name = "registerVehicle",
        abi = "registerVehicle(string,address,string,string,uint256,string[],string)"
    )]
    pub struct RegisterVehicleCall {
        pub vin: ::std::string::String,
        pub vehicle_wallet: ::ethers::core::types::Address,
        pub manufacturer: ::std::string::String,
        pub model: ::std::string::String,
        pub year: ::ethers::core::types::U256,
        pub data_types: ::std::vec::Vec<::std::string::String>,
        pub ipfs_hash: ::std::string::String,
    }
    ///Container type for all input parameters for the `registrationFee` function with signature `registrationFee()` and selector `0x14c44e09`
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
    #[ethcall(name = "registrationFee", abi = "registrationFee()")]
    pub struct RegistrationFeeCall;
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
    ///Container type for all input parameters for the `setRegistrationFee` function with signature `setRegistrationFee(uint256)` and selector `0xc320c727`
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
    #[ethcall(name = "setRegistrationFee", abi = "setRegistrationFee(uint256)")]
    pub struct SetRegistrationFeeCall {
        pub new_fee: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `updateVehicleMetadata` function with signature `updateVehicleMetadata(uint256,string[],string)` and selector `0xeb4dc98e`
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
        name = "updateVehicleMetadata",
        abi = "updateVehicleMetadata(uint256,string[],string)"
    )]
    pub struct UpdateVehicleMetadataCall {
        pub vehicle_id: ::ethers::core::types::U256,
        pub data_types: ::std::vec::Vec<::std::string::String>,
        pub ipfs_hash: ::std::string::String,
    }
    ///Container type for all input parameters for the `vehicleCounter` function with signature `vehicleCounter()` and selector `0x178206f6`
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
    #[ethcall(name = "vehicleCounter", abi = "vehicleCounter()")]
    pub struct VehicleCounterCall;
    ///Container type for all input parameters for the `vehicleMetadata` function with signature `vehicleMetadata(uint256)` and selector `0xfd7ae498`
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
    #[ethcall(name = "vehicleMetadata", abi = "vehicleMetadata(uint256)")]
    pub struct VehicleMetadataCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `vehicles` function with signature `vehicles(uint256)` and selector `0xb8ba95fa`
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
    #[ethcall(name = "vehicles", abi = "vehicles(uint256)")]
    pub struct VehiclesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `vinToVehicleId` function with signature `vinToVehicleId(string)` and selector `0x903f7c12`
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
    #[ethcall(name = "vinToVehicleId", abi = "vinToVehicleId(string)")]
    pub struct VinToVehicleIdCall(pub ::std::string::String);
    ///Container type for all input parameters for the `walletToVehicleId` function with signature `walletToVehicleId(address)` and selector `0xbc10c7e2`
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
    #[ethcall(name = "walletToVehicleId", abi = "walletToVehicleId(address)")]
    pub struct WalletToVehicleIdCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdrawFees` function with signature `withdrawFees()` and selector `0x476343ee`
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
    #[ethcall(name = "withdrawFees", abi = "withdrawFees()")]
    pub struct WithdrawFeesCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VehicleRegistryCalls {
        DeactivateVehicle(DeactivateVehicleCall),
        GetTotalVehicles(GetTotalVehiclesCall),
        GetVehicle(GetVehicleCall),
        GetVehicleIdByVin(GetVehicleIdByVinCall),
        GetVehicleIdByWallet(GetVehicleIdByWalletCall),
        GetVehicleMetadata(GetVehicleMetadataCall),
        IsVehicleActive(IsVehicleActiveCall),
        Owner(OwnerCall),
        RegisterVehicle(RegisterVehicleCall),
        RegistrationFee(RegistrationFeeCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetRegistrationFee(SetRegistrationFeeCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateVehicleMetadata(UpdateVehicleMetadataCall),
        VehicleCounter(VehicleCounterCall),
        VehicleMetadata(VehicleMetadataCall),
        Vehicles(VehiclesCall),
        VinToVehicleId(VinToVehicleIdCall),
        WalletToVehicleId(WalletToVehicleIdCall),
        WithdrawFees(WithdrawFeesCall),
    }
    impl ::ethers::core::abi::AbiDecode for VehicleRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DeactivateVehicleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateVehicle(decoded));
            }
            if let Ok(decoded) = <GetTotalVehiclesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalVehicles(decoded));
            }
            if let Ok(decoded) = <GetVehicleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVehicle(decoded));
            }
            if let Ok(decoded) = <GetVehicleIdByVinCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVehicleIdByVin(decoded));
            }
            if let Ok(decoded) = <GetVehicleIdByWalletCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVehicleIdByWallet(decoded));
            }
            if let Ok(decoded) = <GetVehicleMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVehicleMetadata(decoded));
            }
            if let Ok(decoded) = <IsVehicleActiveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsVehicleActive(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RegisterVehicleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterVehicle(decoded));
            }
            if let Ok(decoded) = <RegistrationFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistrationFee(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetRegistrationFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRegistrationFee(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateVehicleMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateVehicleMetadata(decoded));
            }
            if let Ok(decoded) = <VehicleCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleCounter(decoded));
            }
            if let Ok(decoded) = <VehicleMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VehicleMetadata(decoded));
            }
            if let Ok(decoded) = <VehiclesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vehicles(decoded));
            }
            if let Ok(decoded) = <VinToVehicleIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VinToVehicleId(decoded));
            }
            if let Ok(decoded) = <WalletToVehicleIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WalletToVehicleId(decoded));
            }
            if let Ok(decoded) = <WithdrawFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawFees(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VehicleRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeactivateVehicle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalVehicles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVehicle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVehicleIdByVin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVehicleIdByWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVehicleMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsVehicleActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterVehicle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistrationFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRegistrationFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateVehicleMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VehicleMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vehicles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VinToVehicleId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WalletToVehicleId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VehicleRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeactivateVehicle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalVehicles(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVehicle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVehicleIdByVin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVehicleIdByWallet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVehicleMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsVehicleActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterVehicle(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistrationFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRegistrationFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateVehicleMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VehicleCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VehicleMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vehicles(element) => ::core::fmt::Display::fmt(element, f),
                Self::VinToVehicleId(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletToVehicleId(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFees(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeactivateVehicleCall> for VehicleRegistryCalls {
        fn from(value: DeactivateVehicleCall) -> Self {
            Self::DeactivateVehicle(value)
        }
    }
    impl ::core::convert::From<GetTotalVehiclesCall> for VehicleRegistryCalls {
        fn from(value: GetTotalVehiclesCall) -> Self {
            Self::GetTotalVehicles(value)
        }
    }
    impl ::core::convert::From<GetVehicleCall> for VehicleRegistryCalls {
        fn from(value: GetVehicleCall) -> Self {
            Self::GetVehicle(value)
        }
    }
    impl ::core::convert::From<GetVehicleIdByVinCall> for VehicleRegistryCalls {
        fn from(value: GetVehicleIdByVinCall) -> Self {
            Self::GetVehicleIdByVin(value)
        }
    }
    impl ::core::convert::From<GetVehicleIdByWalletCall> for VehicleRegistryCalls {
        fn from(value: GetVehicleIdByWalletCall) -> Self {
            Self::GetVehicleIdByWallet(value)
        }
    }
    impl ::core::convert::From<GetVehicleMetadataCall> for VehicleRegistryCalls {
        fn from(value: GetVehicleMetadataCall) -> Self {
            Self::GetVehicleMetadata(value)
        }
    }
    impl ::core::convert::From<IsVehicleActiveCall> for VehicleRegistryCalls {
        fn from(value: IsVehicleActiveCall) -> Self {
            Self::IsVehicleActive(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VehicleRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterVehicleCall> for VehicleRegistryCalls {
        fn from(value: RegisterVehicleCall) -> Self {
            Self::RegisterVehicle(value)
        }
    }
    impl ::core::convert::From<RegistrationFeeCall> for VehicleRegistryCalls {
        fn from(value: RegistrationFeeCall) -> Self {
            Self::RegistrationFee(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VehicleRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetRegistrationFeeCall> for VehicleRegistryCalls {
        fn from(value: SetRegistrationFeeCall) -> Self {
            Self::SetRegistrationFee(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VehicleRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateVehicleMetadataCall> for VehicleRegistryCalls {
        fn from(value: UpdateVehicleMetadataCall) -> Self {
            Self::UpdateVehicleMetadata(value)
        }
    }
    impl ::core::convert::From<VehicleCounterCall> for VehicleRegistryCalls {
        fn from(value: VehicleCounterCall) -> Self {
            Self::VehicleCounter(value)
        }
    }
    impl ::core::convert::From<VehicleMetadataCall> for VehicleRegistryCalls {
        fn from(value: VehicleMetadataCall) -> Self {
            Self::VehicleMetadata(value)
        }
    }
    impl ::core::convert::From<VehiclesCall> for VehicleRegistryCalls {
        fn from(value: VehiclesCall) -> Self {
            Self::Vehicles(value)
        }
    }
    impl ::core::convert::From<VinToVehicleIdCall> for VehicleRegistryCalls {
        fn from(value: VinToVehicleIdCall) -> Self {
            Self::VinToVehicleId(value)
        }
    }
    impl ::core::convert::From<WalletToVehicleIdCall> for VehicleRegistryCalls {
        fn from(value: WalletToVehicleIdCall) -> Self {
            Self::WalletToVehicleId(value)
        }
    }
    impl ::core::convert::From<WithdrawFeesCall> for VehicleRegistryCalls {
        fn from(value: WithdrawFeesCall) -> Self {
            Self::WithdrawFees(value)
        }
    }
    ///Container type for all return fields from the `getTotalVehicles` function with signature `getTotalVehicles()` and selector `0x69056b27`
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
    pub struct GetTotalVehiclesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVehicle` function with signature `getVehicle(uint256)` and selector `0xdf7ebb7b`
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
    pub struct GetVehicleReturn(pub Vehicle);
    ///Container type for all return fields from the `getVehicleIdByVin` function with signature `getVehicleIdByVin(string)` and selector `0x5e308196`
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
    pub struct GetVehicleIdByVinReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVehicleIdByWallet` function with signature `getVehicleIdByWallet(address)` and selector `0x758cb543`
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
    pub struct GetVehicleIdByWalletReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVehicleMetadata` function with signature `getVehicleMetadata(uint256)` and selector `0xbadf89bc`
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
    pub struct GetVehicleMetadataReturn(pub VehicleMetadata);
    ///Container type for all return fields from the `isVehicleActive` function with signature `isVehicleActive(uint256)` and selector `0x4c5907e6`
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
    pub struct IsVehicleActiveReturn(pub bool);
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
    ///Container type for all return fields from the `registrationFee` function with signature `registrationFee()` and selector `0x14c44e09`
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
    pub struct RegistrationFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vehicleCounter` function with signature `vehicleCounter()` and selector `0x178206f6`
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
    pub struct VehicleCounterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vehicleMetadata` function with signature `vehicleMetadata(uint256)` and selector `0xfd7ae498`
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
    pub struct VehicleMetadataReturn {
        pub ipfs_hash: ::std::string::String,
        pub last_update: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `vehicles` function with signature `vehicles(uint256)` and selector `0xb8ba95fa`
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
    pub struct VehiclesReturn {
        pub vin: ::std::string::String,
        pub wallet: ::ethers::core::types::Address,
        pub manufacturer: ::std::string::String,
        pub model: ::std::string::String,
        pub year: ::ethers::core::types::U256,
        pub is_active: bool,
        pub registration_timestamp: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `vinToVehicleId` function with signature `vinToVehicleId(string)` and selector `0x903f7c12`
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
    pub struct VinToVehicleIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `walletToVehicleId` function with signature `walletToVehicleId(address)` and selector `0xbc10c7e2`
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
    pub struct WalletToVehicleIdReturn(pub ::ethers::core::types::U256);
    ///`Vehicle(string,address,string,string,uint256,bool,uint256,address)`
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
    pub struct Vehicle {
        pub vin: ::std::string::String,
        pub wallet: ::ethers::core::types::Address,
        pub manufacturer: ::std::string::String,
        pub model: ::std::string::String,
        pub year: ::ethers::core::types::U256,
        pub is_active: bool,
        pub registration_timestamp: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
    }
    ///`VehicleMetadata(string[],string,uint256)`
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
    pub struct VehicleMetadata {
        pub data_types: ::std::vec::Vec<::std::string::String>,
        pub ipfs_hash: ::std::string::String,
        pub last_update: ::ethers::core::types::U256,
    }
}
