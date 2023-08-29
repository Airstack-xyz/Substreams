    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct BaseInitialized {
            pub name: String,
            pub symbol: String,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl BaseInitialized {
            const TOPIC_ID: [u8; 32] = [
                65u8,
                76u8,
                208u8,
                179u8,
                70u8,
                118u8,
                152u8,
                79u8,
                9u8,
                165u8,
                247u8,
                108u8,
                233u8,
                113u8,
                141u8,
                64u8,
                98u8,
                229u8,
                2u8,
                131u8,
                171u8,
                224u8,
                231u8,
                226u8,
                116u8,
                169u8,
                165u8,
                180u8,
                224u8,
                201u8,
                156u8,
                48u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    name: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    symbol: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for BaseInitialized {
            const NAME: &'static str = "BaseInitialized";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectModuleWhitelisted {
            pub collect_module: Vec<u8>,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CollectModuleWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                108u8,
                193u8,
                154u8,
                121u8,
                77u8,
                106u8,
                67u8,
                144u8,
                35u8,
                21u8,
                12u8,
                213u8,
                135u8,
                72u8,
                238u8,
                212u8,
                53u8,
                49u8,
                144u8,
                192u8,
                187u8,
                6u8,
                13u8,
                46u8,
                98u8,
                80u8,
                226u8,
                223u8,
                74u8,
                104u8,
                182u8,
                115u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    collect_module: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'collect_module' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CollectModuleWhitelisted {
            const NAME: &'static str = "CollectModuleWhitelisted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectNftDeployed {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub collect_nft: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CollectNftDeployed {
            const TOPIC_ID: [u8; 32] = [
                11u8,
                34u8,
                123u8,
                85u8,
                15u8,
                254u8,
                212u8,
                138u8,
                248u8,
                19u8,
                179u8,
                46u8,
                36u8,
                111u8,
                120u8,
                126u8,
                153u8,
                88u8,
                30u8,
                225u8,
                50u8,
                6u8,
                186u8,
                143u8,
                157u8,
                144u8,
                214u8,
                54u8,
                21u8,
                38u8,
                155u8,
                63u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collect_nft: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'collect_nft' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CollectNftDeployed {
            const NAME: &'static str = "CollectNFTDeployed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectNftInitialized {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CollectNftInitialized {
            const TOPIC_ID: [u8; 32] = [
                137u8,
                138u8,
                45u8,
                236u8,
                149u8,
                133u8,
                98u8,
                85u8,
                151u8,
                122u8,
                15u8,
                180u8,
                140u8,
                235u8,
                195u8,
                0u8,
                81u8,
                213u8,
                12u8,
                13u8,
                141u8,
                51u8,
                249u8,
                61u8,
                234u8,
                30u8,
                61u8,
                219u8,
                46u8,
                52u8,
                36u8,
                66u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CollectNftInitialized {
            const NAME: &'static str = "CollectNFTInitialized";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectNftTransferred {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub collect_nft_id: substreams::scalar::BigInt,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CollectNftTransferred {
            const TOPIC_ID: [u8; 32] = [
                104u8,
                237u8,
                183u8,
                236u8,
                44u8,
                55u8,
                210u8,
                27u8,
                59u8,
                114u8,
                35u8,
                57u8,
                96u8,
                180u8,
                135u8,
                242u8,
                150u8,
                111u8,
                74u8,
                200u8,
                43u8,
                116u8,
                48u8,
                211u8,
                159u8,
                36u8,
                209u8,
                248u8,
                214u8,
                249u8,
                145u8,
                6u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collect_nft_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'collect_nft_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CollectNftTransferred {
            const NAME: &'static str = "CollectNFTTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Collected {
            pub collector: Vec<u8>,
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub root_profile_id: substreams::scalar::BigInt,
            pub root_pub_id: substreams::scalar::BigInt,
            pub collect_module_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Collected {
            const TOPIC_ID: [u8; 32] = [
                237u8,
                57u8,
                191u8,
                13u8,
                154u8,
                250u8,
                132u8,
                150u8,
                16u8,
                185u8,
                1u8,
                201u8,
                215u8,
                244u8,
                208u8,
                7u8,
                81u8,
                186u8,
                32u8,
                222u8,
                45u8,
                176u8,
                35u8,
                66u8,
                128u8,
                101u8,
                190u8,
                193u8,
                83u8,
                131u8,
                50u8,
                24u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    collector: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'collector' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    root_profile_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    root_pub_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collect_module_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Collected {
            const NAME: &'static str = "Collected";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CommentCreated {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub content_uri: String,
            pub profile_id_pointed: substreams::scalar::BigInt,
            pub pub_id_pointed: substreams::scalar::BigInt,
            pub reference_module_data: Vec<u8>,
            pub collect_module: Vec<u8>,
            pub collect_module_return_data: Vec<u8>,
            pub reference_module: Vec<u8>,
            pub reference_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CommentCreated {
            const TOPIC_ID: [u8; 32] = [
                123u8,
                77u8,
                26u8,
                163u8,
                55u8,
                115u8,
                22u8,
                23u8,
                153u8,
                132u8,
                116u8,
                41u8,
                228u8,
                251u8,
                242u8,
                159u8,
                86u8,
                219u8,
                241u8,
                163u8,
                254u8,
                129u8,
                95u8,
                80u8,
                112u8,
                35u8,
                28u8,
                191u8,
                186u8,
                64u8,
                44u8,
                55u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 416usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    content_uri: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    profile_id_pointed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id_pointed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    reference_module_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    collect_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    collect_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    reference_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    reference_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CommentCreated {
            const NAME: &'static str = "CommentCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DefaultProfileSet {
            pub wallet: Vec<u8>,
            pub profile_id: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl DefaultProfileSet {
            const TOPIC_ID: [u8; 32] = [
                10u8,
                253u8,
                124u8,
                71u8,
                158u8,
                139u8,
                199u8,
                220u8,
                219u8,
                133u8,
                107u8,
                60u8,
                194u8,
                125u8,
                35u8,
                50u8,
                223u8,
                225u8,
                240u8,
                24u8,
                253u8,
                229u8,
                116u8,
                234u8,
                18u8,
                73u8,
                25u8,
                221u8,
                202u8,
                232u8,
                169u8,
                51u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wallet: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'wallet' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for DefaultProfileSet {
            const NAME: &'static str = "DefaultProfileSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DispatcherSet {
            pub profile_id: substreams::scalar::BigInt,
            pub dispatcher: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl DispatcherSet {
            const TOPIC_ID: [u8; 32] = [
                34u8,
                186u8,
                174u8,
                196u8,
                149u8,
                47u8,
                53u8,
                245u8,
                158u8,
                69u8,
                189u8,
                45u8,
                219u8,
                40u8,
                126u8,
                28u8,
                204u8,
                109u8,
                49u8,
                147u8,
                117u8,
                119u8,
                12u8,
                9u8,
                66u8,
                142u8,
                184u8,
                248u8,
                214u8,
                4u8,
                224u8,
                101u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    dispatcher: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'dispatcher' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for DispatcherSet {
            const NAME: &'static str = "DispatcherSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct EmergencyAdminSet {
            pub caller: Vec<u8>,
            pub old_emergency_admin: Vec<u8>,
            pub new_emergency_admin: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl EmergencyAdminSet {
            const TOPIC_ID: [u8; 32] = [
                103u8,
                108u8,
                8u8,
                1u8,
                176u8,
                244u8,
                0u8,
                118u8,
                46u8,
                149u8,
                142u8,
                227u8,
                28u8,
                251u8,
                177u8,
                8u8,
                112u8,
                231u8,
                7u8,
                134u8,
                246u8,
                118u8,
                31u8,
                87u8,
                200u8,
                100u8,
                126u8,
                118u8,
                107u8,
                13u8,
                179u8,
                217u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    caller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'caller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    old_emergency_admin: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'old_emergency_admin' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_emergency_admin: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_emergency_admin' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for EmergencyAdminSet {
            const NAME: &'static str = "EmergencyAdminSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FeeModuleBaseConstructed {
            pub module_globals: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FeeModuleBaseConstructed {
            const TOPIC_ID: [u8; 32] = [
                78u8,
                132u8,
                165u8,
                41u8,
                244u8,
                198u8,
                39u8,
                181u8,
                231u8,
                135u8,
                3u8,
                125u8,
                17u8,
                120u8,
                115u8,
                175u8,
                16u8,
                24u8,
                118u8,
                136u8,
                4u8,
                204u8,
                163u8,
                199u8,
                240u8,
                212u8,
                112u8,
                65u8,
                254u8,
                44u8,
                137u8,
                237u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    module_globals: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'module_globals' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FeeModuleBaseConstructed {
            const NAME: &'static str = "FeeModuleBaseConstructed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowModuleSet {
            pub profile_id: substreams::scalar::BigInt,
            pub follow_module: Vec<u8>,
            pub follow_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowModuleSet {
            const TOPIC_ID: [u8; 32] = [
                146u8,
                217u8,
                94u8,
                64u8,
                9u8,
                50u8,
                209u8,
                41u8,
                136u8,
                94u8,
                98u8,
                123u8,
                56u8,
                177u8,
                105u8,
                203u8,
                178u8,
                132u8,
                67u8,
                255u8,
                170u8,
                162u8,
                130u8,
                208u8,
                251u8,
                160u8,
                207u8,
                135u8,
                151u8,
                114u8,
                19u8,
                89u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    follow_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowModuleSet {
            const NAME: &'static str = "FollowModuleSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowModuleWhitelisted {
            pub follow_module: Vec<u8>,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowModuleWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                82u8,
                197u8,
                183u8,
                136u8,
                157u8,
                249u8,
                241u8,
                47u8,
                132u8,
                236u8,
                61u8,
                160u8,
                81u8,
                232u8,
                84u8,
                229u8,
                135u8,
                102u8,
                120u8,
                55u8,
                13u8,
                131u8,
                87u8,
                149u8,
                156u8,
                35u8,
                239u8,
                89u8,
                221u8,
                100u8,
                134u8,
                223u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    follow_module: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'follow_module' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowModuleWhitelisted {
            const NAME: &'static str = "FollowModuleWhitelisted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowNftDelegatedPowerChanged {
            pub delegate: Vec<u8>,
            pub new_power: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowNftDelegatedPowerChanged {
            const TOPIC_ID: [u8; 32] = [
                217u8,
                166u8,
                7u8,
                1u8,
                116u8,
                244u8,
                204u8,
                202u8,
                118u8,
                237u8,
                72u8,
                150u8,
                67u8,
                46u8,
                154u8,
                9u8,
                11u8,
                22u8,
                224u8,
                126u8,
                143u8,
                226u8,
                127u8,
                39u8,
                95u8,
                80u8,
                179u8,
                53u8,
                0u8,
                185u8,
                142u8,
                82u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    delegate: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'delegate' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_power: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'new_power' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowNftDelegatedPowerChanged {
            const NAME: &'static str = "FollowNFTDelegatedPowerChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowNftDeployed {
            pub profile_id: substreams::scalar::BigInt,
            pub follow_nft: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowNftDeployed {
            const TOPIC_ID: [u8; 32] = [
                68u8,
                64u8,
                62u8,
                56u8,
                186u8,
                237u8,
                94u8,
                64u8,
                223u8,
                127u8,
                100u8,
                255u8,
                135u8,
                8u8,
                176u8,
                118u8,
                199u8,
                90u8,
                13u8,
                253u8,
                168u8,
                56u8,
                14u8,
                117u8,
                223u8,
                92u8,
                54u8,
                241u8,
                26u8,
                71u8,
                103u8,
                67u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_nft: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'follow_nft' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowNftDeployed {
            const NAME: &'static str = "FollowNFTDeployed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowNftInitialized {
            pub profile_id: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowNftInitialized {
            const TOPIC_ID: [u8; 32] = [
                174u8,
                193u8,
                81u8,
                39u8,
                223u8,
                17u8,
                166u8,
                181u8,
                98u8,
                200u8,
                125u8,
                49u8,
                188u8,
                184u8,
                244u8,
                205u8,
                47u8,
                12u8,
                245u8,
                127u8,
                185u8,
                182u8,
                99u8,
                214u8,
                51u8,
                74u8,
                191u8,
                65u8,
                254u8,
                169u8,
                77u8,
                149u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowNftInitialized {
            const NAME: &'static str = "FollowNFTInitialized";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowNftTransferred {
            pub profile_id: substreams::scalar::BigInt,
            pub follow_nft_id: substreams::scalar::BigInt,
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowNftTransferred {
            const TOPIC_ID: [u8; 32] = [
                73u8,
                150u8,
                173u8,
                34u8,
                87u8,
                231u8,
                219u8,
                68u8,
                144u8,
                129u8,
                54u8,
                196u8,
                49u8,
                40u8,
                204u8,
                16u8,
                202u8,
                152u8,
                128u8,
                150u8,
                246u8,
                125u8,
                198u8,
                187u8,
                11u8,
                206u8,
                225u8,
                29u8,
                21u8,
                19u8,
                104u8,
                251u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_nft_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'follow_nft_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    from: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowNftTransferred {
            const NAME: &'static str = "FollowNFTTransferred";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowNfturiSet {
            pub profile_id: substreams::scalar::BigInt,
            pub follow_nfturi: String,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowNfturiSet {
            const TOPIC_ID: [u8; 32] = [
                232u8,
                40u8,
                134u8,
                225u8,
                175u8,
                111u8,
                202u8,
                181u8,
                202u8,
                239u8,
                19u8,
                129u8,
                91u8,
                34u8,
                245u8,
                19u8,
                132u8,
                233u8,
                112u8,
                195u8,
                103u8,
                167u8,
                133u8,
                242u8,
                101u8,
                209u8,
                56u8,
                96u8,
                167u8,
                214u8,
                150u8,
                109u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_nfturi: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowNfturiSet {
            const NAME: &'static str = "FollowNFTURISet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Followed {
            pub follower: Vec<u8>,
            pub profile_ids: Vec<substreams::scalar::BigInt>,
            pub follow_module_datas: Vec<Vec<u8>>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Followed {
            const TOPIC_ID: [u8; 32] = [
                64u8,
                72u8,
                112u8,
                114u8,
                220u8,
                86u8,
                243u8,
                132u8,
                40u8,
                125u8,
                38u8,
                251u8,
                224u8,
                144u8,
                244u8,
                4u8,
                20u8,
                60u8,
                39u8,
                55u8,
                213u8,
                70u8,
                50u8,
                23u8,
                116u8,
                81u8,
                209u8,
                247u8,
                75u8,
                216u8,
                44u8,
                118u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    follower: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'follower' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    profile_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let mut v = [0 as u8; 32];
                            inner
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        })
                        .collect(),
                    follow_module_datas: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                        .collect(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for Followed {
            const NAME: &'static str = "Followed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowsApproved {
            pub owner: Vec<u8>,
            pub profile_id: substreams::scalar::BigInt,
            pub addresses: Vec<Vec<u8>>,
            pub approved: Vec<bool>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowsApproved {
            const TOPIC_ID: [u8; 32] = [
                198u8,
                127u8,
                195u8,
                151u8,
                45u8,
                165u8,
                214u8,
                67u8,
                74u8,
                183u8,
                183u8,
                150u8,
                186u8,
                19u8,
                60u8,
                36u8,
                13u8,
                64u8,
                238u8,
                78u8,
                105u8,
                18u8,
                153u8,
                99u8,
                197u8,
                170u8,
                15u8,
                42u8,
                111u8,
                124u8,
                58u8,
                214u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool)),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    addresses: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                        .collect(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowsApproved {
            const NAME: &'static str = "FollowsApproved";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FollowsToggled {
            pub owner: Vec<u8>,
            pub profile_ids: Vec<substreams::scalar::BigInt>,
            pub enabled: Vec<bool>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl FollowsToggled {
            const TOPIC_ID: [u8; 32] = [
                85u8,
                56u8,
                200u8,
                12u8,
                141u8,
                59u8,
                238u8,
                57u8,
                125u8,
                135u8,
                167u8,
                209u8,
                83u8,
                247u8,
                240u8,
                133u8,
                187u8,
                18u8,
                173u8,
                242u8,
                254u8,
                37u8,
                160u8,
                38u8,
                199u8,
                204u8,
                78u8,
                131u8,
                216u8,
                197u8,
                241u8,
                215u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool)),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'owner' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    profile_ids: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let mut v = [0 as u8; 32];
                            inner
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        })
                        .collect(),
                    enabled: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                        .collect(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for FollowsToggled {
            const NAME: &'static str = "FollowsToggled";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GovernanceSet {
            pub caller: Vec<u8>,
            pub prev_governance: Vec<u8>,
            pub new_governance: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl GovernanceSet {
            const TOPIC_ID: [u8; 32] = [
                229u8,
                82u8,
                165u8,
                84u8,
                85u8,
                183u8,
                64u8,
                132u8,
                90u8,
                92u8,
                7u8,
                237u8,
                68u8,
                93u8,
                23u8,
                36u8,
                20u8,
                47u8,
                201u8,
                151u8,
                179u8,
                137u8,
                131u8,
                84u8,
                149u8,
                162u8,
                155u8,
                48u8,
                205u8,
                220u8,
                28u8,
                205u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    caller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'caller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    prev_governance: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'prev_governance' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_governance: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_governance' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for GovernanceSet {
            const NAME: &'static str = "GovernanceSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MirrorCreated {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub profile_id_pointed: substreams::scalar::BigInt,
            pub pub_id_pointed: substreams::scalar::BigInt,
            pub reference_module_data: Vec<u8>,
            pub reference_module: Vec<u8>,
            pub reference_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl MirrorCreated {
            const TOPIC_ID: [u8; 32] = [
                158u8,
                165u8,
                222u8,
                219u8,
                133u8,
                189u8,
                157u8,
                164u8,
                226u8,
                100u8,
                238u8,
                90u8,
                57u8,
                183u8,
                186u8,
                9u8,
                130u8,
                229u8,
                212u8,
                208u8,
                53u8,
                213u8,
                94u8,
                223u8,
                169u8,
                138u8,
                54u8,
                176u8,
                14u8,
                119u8,
                11u8,
                90u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 256usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    profile_id_pointed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id_pointed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    reference_module_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    reference_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    reference_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for MirrorCreated {
            const NAME: &'static str = "MirrorCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModuleBaseConstructed {
            pub hub: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ModuleBaseConstructed {
            const TOPIC_ID: [u8; 32] = [
                241u8,
                161u8,
                250u8,
                107u8,
                100u8,
                170u8,
                149u8,
                24u8,
                111u8,
                90u8,
                18u8,
                133u8,
                231u8,
                97u8,
                152u8,
                208u8,
                218u8,
                128u8,
                217u8,
                197u8,
                168u8,
                128u8,
                98u8,
                100u8,
                29u8,
                68u8,
                127u8,
                29u8,
                124u8,
                84u8,
                229u8,
                108u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    hub: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'hub' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ModuleBaseConstructed {
            const NAME: &'static str = "ModuleBaseConstructed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModuleGlobalsCurrencyWhitelisted {
            pub currency: Vec<u8>,
            pub prev_whitelisted: bool,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ModuleGlobalsCurrencyWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                121u8,
                195u8,
                206u8,
                252u8,
                133u8,
                31u8,
                214u8,
                4u8,
                15u8,
                6u8,
                175u8,
                32u8,
                44u8,
                84u8,
                40u8,
                24u8,
                217u8,
                251u8,
                57u8,
                188u8,
                221u8,
                203u8,
                122u8,
                126u8,
                63u8,
                86u8,
                59u8,
                21u8,
                48u8,
                10u8,
                39u8,
                67u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    currency: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'currency' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    prev_whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'prev_whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ModuleGlobalsCurrencyWhitelisted {
            const NAME: &'static str = "ModuleGlobalsCurrencyWhitelisted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModuleGlobalsGovernanceSet {
            pub prev_governance: Vec<u8>,
            pub new_governance: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ModuleGlobalsGovernanceSet {
            const TOPIC_ID: [u8; 32] = [
                191u8,
                83u8,
                138u8,
                44u8,
                13u8,
                179u8,
                212u8,
                64u8,
                144u8,
                107u8,
                129u8,
                121u8,
                221u8,
                3u8,
                148u8,
                246u8,
                138u8,
                101u8,
                176u8,
                177u8,
                72u8,
                29u8,
                167u8,
                15u8,
                254u8,
                226u8,
                78u8,
                25u8,
                220u8,
                206u8,
                232u8,
                76u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    prev_governance: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'prev_governance' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_governance: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_governance' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ModuleGlobalsGovernanceSet {
            const NAME: &'static str = "ModuleGlobalsGovernanceSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModuleGlobalsTreasuryFeeSet {
            pub prev_treasury_fee: substreams::scalar::BigInt,
            pub new_treasury_fee: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ModuleGlobalsTreasuryFeeSet {
            const TOPIC_ID: [u8; 32] = [
                236u8,
                147u8,
                104u8,
                98u8,
                230u8,
                187u8,
                137u8,
                124u8,
                215u8,
                17u8,
                165u8,
                243u8,
                24u8,
                37u8,
                5u8,
                117u8,
                131u8,
                18u8,
                140u8,
                42u8,
                72u8,
                47u8,
                10u8,
                76u8,
                154u8,
                78u8,
                108u8,
                63u8,
                215u8,
                192u8,
                35u8,
                244u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    prev_treasury_fee: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(16usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'prev_treasury_fee' from topic of type 'uint16': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    new_treasury_fee: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(16usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'new_treasury_fee' from topic of type 'uint16': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ModuleGlobalsTreasuryFeeSet {
            const NAME: &'static str = "ModuleGlobalsTreasuryFeeSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ModuleGlobalsTreasurySet {
            pub prev_treasury: Vec<u8>,
            pub new_treasury: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ModuleGlobalsTreasurySet {
            const TOPIC_ID: [u8; 32] = [
                61u8,
                252u8,
                83u8,
                214u8,
                180u8,
                155u8,
                251u8,
                201u8,
                50u8,
                178u8,
                21u8,
                186u8,
                81u8,
                95u8,
                13u8,
                10u8,
                176u8,
                225u8,
                122u8,
                172u8,
                23u8,
                114u8,
                111u8,
                186u8,
                72u8,
                7u8,
                95u8,
                12u8,
                22u8,
                199u8,
                255u8,
                227u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    prev_treasury: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'prev_treasury' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_treasury: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_treasury' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ModuleGlobalsTreasurySet {
            const NAME: &'static str = "ModuleGlobalsTreasurySet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PostCreated {
            pub profile_id: substreams::scalar::BigInt,
            pub pub_id: substreams::scalar::BigInt,
            pub content_uri: String,
            pub collect_module: Vec<u8>,
            pub collect_module_return_data: Vec<u8>,
            pub reference_module: Vec<u8>,
            pub reference_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl PostCreated {
            const TOPIC_ID: [u8; 32] = [
                198u8,
                114u8,
                195u8,
                139u8,
                77u8,
                38u8,
                195u8,
                201u8,
                120u8,
                34u8,
                142u8,
                153u8,
                22u8,
                65u8,
                5u8,
                40u8,
                4u8,
                16u8,
                177u8,
                68u8,
                175u8,
                36u8,
                221u8,
                62u8,
                216u8,
                228u8,
                249u8,
                210u8,
                17u8,
                217u8,
                106u8,
                80u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 288usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pub_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    content_uri: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    collect_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    collect_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    reference_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    reference_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for PostCreated {
            const NAME: &'static str = "PostCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProfileCreated {
            pub profile_id: substreams::scalar::BigInt,
            pub creator: Vec<u8>,
            pub to: Vec<u8>,
            pub handle: String,
            pub image_uri: String,
            pub follow_module: Vec<u8>,
            pub follow_module_return_data: Vec<u8>,
            pub follow_nfturi: String,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ProfileCreated {
            const TOPIC_ID: [u8; 32] = [
                78u8,
                20u8,
                245u8,
                124u8,
                255u8,
                121u8,
                16u8,
                65u8,
                111u8,
                46u8,
                244u8,
                60u8,
                240u8,
                80u8,
                25u8,
                181u8,
                169u8,
                122u8,
                49u8,
                61u8,
                231u8,
                31u8,
                236u8,
                147u8,
                68u8,
                190u8,
                17u8,
                185u8,
                184u8,
                143u8,
                237u8,
                18u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 320usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::String,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    creator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'creator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'to' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    handle: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    image_uri: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    follow_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    follow_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    follow_nfturi: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ProfileCreated {
            const NAME: &'static str = "ProfileCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProfileCreatorWhitelisted {
            pub profile_creator: Vec<u8>,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ProfileCreatorWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                143u8,
                97u8,
                120u8,
                67u8,
                136u8,
                155u8,
                148u8,
                137u8,
                43u8,
                212u8,
                72u8,
                82u8,
                211u8,
                108u8,
                166u8,
                167u8,
                244u8,
                158u8,
                207u8,
                67u8,
                80u8,
                160u8,
                30u8,
                123u8,
                104u8,
                226u8,
                45u8,
                128u8,
                244u8,
                237u8,
                149u8,
                188u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_creator: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'profile_creator' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ProfileCreatorWhitelisted {
            const NAME: &'static str = "ProfileCreatorWhitelisted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProfileImageUriSet {
            pub profile_id: substreams::scalar::BigInt,
            pub image_uri: String,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ProfileImageUriSet {
            const TOPIC_ID: [u8; 32] = [
                213u8,
                165u8,
                135u8,
                156u8,
                173u8,
                51u8,
                200u8,
                48u8,
                204u8,
                20u8,
                50u8,
                193u8,
                133u8,
                1u8,
                7u8,
                2u8,
                154u8,
                9u8,
                200u8,
                12u8,
                96u8,
                233u8,
                188u8,
                225u8,
                236u8,
                208u8,
                141u8,
                36u8,
                136u8,
                11u8,
                212u8,
                108u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    image_uri: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ProfileImageUriSet {
            const NAME: &'static str = "ProfileImageURISet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ProfileMetadataSet {
            pub profile_id: substreams::scalar::BigInt,
            pub metadata: String,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ProfileMetadataSet {
            const TOPIC_ID: [u8; 32] = [
                249u8,
                1u8,
                168u8,
                179u8,
                131u8,
                41u8,
                20u8,
                164u8,
                89u8,
                153u8,
                221u8,
                76u8,
                66u8,
                95u8,
                190u8,
                66u8,
                235u8,
                65u8,
                130u8,
                114u8,
                77u8,
                57u8,
                65u8,
                0u8,
                64u8,
                30u8,
                99u8,
                61u8,
                159u8,
                11u8,
                40u8,
                106u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'profile_id' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    metadata: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ProfileMetadataSet {
            const NAME: &'static str = "ProfileMetadataSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ReferenceModuleWhitelisted {
            pub reference_module: Vec<u8>,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ReferenceModuleWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                55u8,
                135u8,
                42u8,
                5u8,
                62u8,
                242u8,
                12u8,
                181u8,
                45u8,
                239u8,
                183u8,
                201u8,
                236u8,
                32u8,
                225u8,
                168u8,
                124u8,
                184u8,
                221u8,
                80u8,
                152u8,
                172u8,
                158u8,
                118u8,
                161u8,
                68u8,
                190u8,
                38u8,
                61u8,
                254u8,
                245u8,
                114u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    reference_module: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'reference_module' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    whitelisted: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'whitelisted' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for ReferenceModuleWhitelisted {
            const NAME: &'static str = "ReferenceModuleWhitelisted";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct StateSet {
            pub caller: Vec<u8>,
            pub prev_state: substreams::scalar::BigInt,
            pub new_state: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl StateSet {
            const TOPIC_ID: [u8; 32] = [
                162u8,
                249u8,
                161u8,
                73u8,
                159u8,
                193u8,
                249u8,
                183u8,
                121u8,
                109u8,
                33u8,
                254u8,
                87u8,
                97u8,
                41u8,
                12u8,
                203u8,
                126u8,
                14u8,
                246u8,
                204u8,
                243u8,
                95u8,
                165u8,
                139u8,
                102u8,
                143u8,
                48u8,
                74u8,
                98u8,
                161u8,
                202u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    caller: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'caller' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    prev_state: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(8usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'prev_state' from topic of type 'uint8': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    new_state: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(8usize)],
                                log.topics[3usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'new_state' from topic of type 'uint8': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for StateSet {
            const NAME: &'static str = "StateSet";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokenGuardianStateChanged {
            pub wallet: Vec<u8>,
            pub enabled: bool,
            pub token_guardian_disabling_timestamp: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl TokenGuardianStateChanged {
            const TOPIC_ID: [u8; 32] = [
                3u8,
                90u8,
                223u8,
                59u8,
                190u8,
                22u8,
                179u8,
                23u8,
                207u8,
                74u8,
                62u8,
                5u8,
                201u8,
                102u8,
                234u8,
                101u8,
                113u8,
                209u8,
                175u8,
                0u8,
                20u8,
                124u8,
                95u8,
                18u8,
                27u8,
                209u8,
                81u8,
                75u8,
                30u8,
                50u8,
                42u8,
                6u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    wallet: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'wallet' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    enabled: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'enabled' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    token_guardian_disabling_timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    timestamp: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for TokenGuardianStateChanged {
            const NAME: &'static str = "TokenGuardianStateChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }