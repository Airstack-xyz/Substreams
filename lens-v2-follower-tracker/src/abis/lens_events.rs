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
        pub struct Acted {
            pub publication_action_params: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<u8>,
                Vec<u8>,
            ),
            pub action_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Acted {
            const TOPIC_ID: [u8; 32] = [
                148u8,
                68u8,
                244u8,
                110u8,
                137u8,
                218u8,
                88u8,
                0u8,
                241u8,
                250u8,
                208u8,
                125u8,
                81u8,
                27u8,
                229u8,
                79u8,
                160u8,
                145u8,
                125u8,
                31u8,
                56u8,
                140u8,
                238u8,
                11u8,
                44u8,
                201u8,
                25u8,
                2u8,
                22u8,
                192u8,
                233u8,
                115u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Address, ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    publication_action_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
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
                            tuple_elements[4usize]
                                .clone()
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
                            tuple_elements[5usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[6usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    action_module_return_data: values
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
        impl substreams_ethereum::Event for Acted {
            const NAME: &'static str = "Acted";
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
        pub struct ActionModuleWhitelisted {
            pub action_module: Vec<u8>,
            pub id: substreams::scalar::BigInt,
            pub whitelisted: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ActionModuleWhitelisted {
            const TOPIC_ID: [u8; 32] = [
                192u8,
                115u8,
                125u8,
                170u8,
                241u8,
                193u8,
                140u8,
                250u8,
                223u8,
                206u8,
                155u8,
                88u8,
                236u8,
                147u8,
                222u8,
                204u8,
                73u8,
                133u8,
                56u8,
                200u8,
                110u8,
                28u8,
                61u8,
                235u8,
                109u8,
                58u8,
                59u8,
                62u8,
                147u8,
                136u8,
                53u8,
                96u8,
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
                    action_module: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'action_module' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'id' from topic of type 'uint256': {:?}",
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
        impl substreams_ethereum::Event for ActionModuleWhitelisted {
            const NAME: &'static str = "ActionModuleWhitelisted";
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
        pub struct Blocked {
            pub by_profile_id: substreams::scalar::BigInt,
            pub id_of_profile_blocked: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Blocked {
            const TOPIC_ID: [u8; 32] = [
                248u8,
                5u8,
                209u8,
                234u8,
                157u8,
                73u8,
                12u8,
                224u8,
                122u8,
                109u8,
                146u8,
                69u8,
                169u8,
                149u8,
                205u8,
                177u8,
                212u8,
                209u8,
                212u8,
                39u8,
                40u8,
                168u8,
                163u8,
                212u8,
                92u8,
                107u8,
                235u8,
                33u8,
                159u8,
                222u8,
                178u8,
                118u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    by_profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'by_profile_id' from topic of type 'uint256': {:?}",
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
                    id_of_profile_blocked: {
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
        impl substreams_ethereum::Event for Blocked {
            const NAME: &'static str = "Blocked";
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
            pub collect_action_params: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<u8>,
                Vec<u8>,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<u8>,
            ),
            pub collect_module: Vec<u8>,
            pub collect_nft: Vec<u8>,
            pub token_id: substreams::scalar::BigInt,
            pub collect_action_result: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Collected {
            const TOPIC_ID: [u8; 32] = [
                251u8,
                46u8,
                46u8,
                147u8,
                57u8,
                236u8,
                7u8,
                229u8,
                188u8,
                193u8,
                5u8,
                45u8,
                254u8,
                225u8,
                235u8,
                88u8,
                144u8,
                119u8,
                78u8,
                175u8,
                64u8,
                200u8,
                213u8,
                135u8,
                228u8,
                77u8,
                169u8,
                124u8,
                223u8,
                204u8,
                212u8,
                56u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 608usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address, ethabi::ParamType::Address,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(8usize))),
                                    ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    collect_action_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[5usize]
                                .clone()
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
                            tuple_elements[6usize]
                                .clone()
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
                            tuple_elements[7usize]
                                .clone()
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
                            tuple_elements[8usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    collect_module: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    collect_nft: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    collect_action_result: values
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
            pub comment_params: (
                substreams::scalar::BigInt,
                String,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<u8>,
                Vec<Vec<u8>>,
                Vec<Vec<u8>>,
                Vec<u8>,
                Vec<u8>,
            ),
            pub pub_id: substreams::scalar::BigInt,
            pub reference_module_return_data: Vec<u8>,
            pub action_modules_init_return_datas: Vec<Vec<u8>>,
            pub reference_module_init_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl CommentCreated {
            const TOPIC_ID: [u8; 32] = [
                215u8,
                106u8,
                242u8,
                131u8,
                90u8,
                83u8,
                13u8,
                159u8,
                221u8,
                45u8,
                8u8,
                138u8,
                152u8,
                17u8,
                79u8,
                112u8,
                6u8,
                213u8,
                189u8,
                230u8,
                226u8,
                45u8,
                124u8,
                234u8,
                208u8,
                113u8,
                81u8,
                21u8,
                53u8,
                139u8,
                133u8,
                217u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 800usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Bytes,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                                    ethabi::ParamType::Address, ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
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
                    comment_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_string()
                                .expect(INTERNAL_ERR),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[3usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[4usize]
                                .clone()
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
                            tuple_elements[5usize]
                                .clone()
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
                            tuple_elements[6usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[7usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    inner
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec()
                                })
                                .collect(),
                            tuple_elements[8usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                .collect(),
                            tuple_elements[9usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[10usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    reference_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    action_modules_init_return_datas: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                        .collect(),
                    reference_module_init_return_data: values
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
        pub struct DelegatedExecutorsConfigChanged {
            pub delegator_profile_id: substreams::scalar::BigInt,
            pub config_number: substreams::scalar::BigInt,
            pub delegated_executors: Vec<Vec<u8>>,
            pub approvals: Vec<bool>,
            pub config_switched: bool,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl DelegatedExecutorsConfigChanged {
            const TOPIC_ID: [u8; 32] = [
                205u8,
                207u8,
                239u8,
                68u8,
                132u8,
                219u8,
                36u8,
                80u8,
                29u8,
                69u8,
                180u8,
                102u8,
                109u8,
                77u8,
                85u8,
                190u8,
                171u8,
                35u8,
                141u8,
                7u8,
                48u8,
                209u8,
                114u8,
                52u8,
                56u8,
                203u8,
                124u8,
                128u8,
                13u8,
                186u8,
                208u8,
                251u8,
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
                    delegator_profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'delegator_profile_id' from topic of type 'uint256': {:?}",
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
                    config_number: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'config_number' from topic of type 'uint256': {:?}",
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
                    config_switched: ethabi::decode(
                            &[ethabi::ParamType::Bool],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'config_switched' from topic of type 'bool': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                    delegated_executors: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    approvals: values
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
        impl substreams_ethereum::Event for DelegatedExecutorsConfigChanged {
            const NAME: &'static str = "DelegatedExecutorsConfigChanged";
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
        pub struct Followed {
            pub follower_profile_id: substreams::scalar::BigInt,
            pub id_of_profile_followed: substreams::scalar::BigInt,
            pub follow_token_id_assigned: substreams::scalar::BigInt,
            pub follow_module_data: Vec<u8>,
            pub process_follow_module_return_data: Vec<u8>,
            pub transaction_executor: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Followed {
            const TOPIC_ID: [u8; 32] = [
                129u8,
                125u8,
                44u8,
                113u8,
                163u8,
                236u8,
                53u8,
                220u8,
                80u8,
                242u8,
                228u8,
                176u8,
                216u8,
                144u8,
                148u8,
                60u8,
                137u8,
                242u8,
                167u8,
                171u8,
                157u8,
                150u8,
                239u8,
                242u8,
                51u8,
                237u8,
                164u8,
                147u8,
                43u8,
                80u8,
                109u8,
                11u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    follower_profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'follower_profile_id' from topic of type 'uint256': {:?}",
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
                    id_of_profile_followed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_token_id_assigned: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    follow_module_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    process_follow_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    transaction_executor: values
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
            pub mirror_params: (
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<u8>,
            ),
            pub pub_id: substreams::scalar::BigInt,
            pub reference_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl MirrorCreated {
            const TOPIC_ID: [u8; 32] = [
                15u8,
                117u8,
                191u8,
                37u8,
                191u8,
                35u8,
                76u8,
                180u8,
                172u8,
                250u8,
                136u8,
                2u8,
                209u8,
                120u8,
                149u8,
                238u8,
                87u8,
                136u8,
                12u8,
                236u8,
                253u8,
                241u8,
                56u8,
                123u8,
                71u8,
                162u8,
                132u8,
                82u8,
                49u8,
                27u8,
                37u8,
                53u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 384usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
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
                    mirror_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
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
                            tuple_elements[4usize]
                                .clone()
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
                            tuple_elements[5usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
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
            pub post_params: (
                substreams::scalar::BigInt,
                String,
                Vec<Vec<u8>>,
                Vec<Vec<u8>>,
                Vec<u8>,
                Vec<u8>,
            ),
            pub pub_id: substreams::scalar::BigInt,
            pub action_modules_init_return_datas: Vec<Vec<u8>>,
            pub reference_module_init_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl PostCreated {
            const TOPIC_ID: [u8; 32] = [
                203u8,
                64u8,
                140u8,
                173u8,
                196u8,
                117u8,
                134u8,
                194u8,
                85u8,
                167u8,
                167u8,
                34u8,
                195u8,
                27u8,
                218u8,
                20u8,
                217u8,
                168u8,
                101u8,
                14u8,
                2u8,
                12u8,
                23u8,
                249u8,
                153u8,
                12u8,
                237u8,
                37u8,
                125u8,
                251u8,
                127u8,
                130u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 480usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::String,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                                    ethabi::ParamType::Address, ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
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
                    post_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_string()
                                .expect(INTERNAL_ERR),
                            tuple_elements[2usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    inner
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec()
                                })
                                .collect(),
                            tuple_elements[3usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                .collect(),
                            tuple_elements[4usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[5usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    action_modules_init_return_datas: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                        .collect(),
                    reference_module_init_return_data: values
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
            pub image_uri: String,
            pub follow_module: Vec<u8>,
            pub follow_module_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl ProfileCreated {
            const TOPIC_ID: [u8; 32] = [
                77u8,
                27u8,
                140u8,
                252u8,
                96u8,
                232u8,
                112u8,
                186u8,
                117u8,
                24u8,
                131u8,
                17u8,
                203u8,
                89u8,
                27u8,
                229u8,
                170u8,
                198u8,
                81u8,
                215u8,
                229u8,
                140u8,
                142u8,
                177u8,
                137u8,
                166u8,
                73u8,
                76u8,
                7u8,
                148u8,
                115u8,
                34u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() < 192usize {
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
        pub struct QuoteCreated {
            pub quote_params: (
                substreams::scalar::BigInt,
                String,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<substreams::scalar::BigInt>,
                Vec<substreams::scalar::BigInt>,
                Vec<u8>,
                Vec<Vec<u8>>,
                Vec<Vec<u8>>,
                Vec<u8>,
                Vec<u8>,
            ),
            pub pub_id: substreams::scalar::BigInt,
            pub reference_module_return_data: Vec<u8>,
            pub action_modules_init_return_datas: Vec<Vec<u8>>,
            pub reference_module_init_return_data: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl QuoteCreated {
            const TOPIC_ID: [u8; 32] = [
                186u8,
                186u8,
                141u8,
                134u8,
                189u8,
                184u8,
                195u8,
                167u8,
                68u8,
                64u8,
                1u8,
                192u8,
                1u8,
                20u8,
                235u8,
                99u8,
                75u8,
                242u8,
                177u8,
                120u8,
                69u8,
                227u8,
                109u8,
                100u8,
                31u8,
                138u8,
                6u8,
                105u8,
                17u8,
                160u8,
                170u8,
                237u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 800usize {
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
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::String,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                                    ethabi::ParamType::Bytes,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Address)),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                                    ethabi::ParamType::Address, ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
                            ethabi::ParamType::Bytes,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    pub_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
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
                    quote_params: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_string()
                                .expect(INTERNAL_ERR),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[3usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[4usize]
                                .clone()
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
                            tuple_elements[5usize]
                                .clone()
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
                            tuple_elements[6usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[7usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    inner
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec()
                                })
                                .collect(),
                            tuple_elements[8usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                                .collect(),
                            tuple_elements[9usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[10usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    reference_module_return_data: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                    action_modules_init_return_datas: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                        .collect(),
                    reference_module_init_return_data: values
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
        impl substreams_ethereum::Event for QuoteCreated {
            const NAME: &'static str = "QuoteCreated";
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
        #[derive(Debug, Clone, PartialEq)]
        pub struct Unblocked {
            pub by_profile_id: substreams::scalar::BigInt,
            pub id_of_profile_unblocked: substreams::scalar::BigInt,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Unblocked {
            const TOPIC_ID: [u8; 32] = [
                3u8,
                71u8,
                202u8,
                166u8,
                16u8,
                103u8,
                131u8,
                151u8,
                250u8,
                32u8,
                131u8,
                2u8,
                82u8,
                56u8,
                102u8,
                137u8,
                182u8,
                60u8,
                246u8,
                219u8,
                190u8,
                37u8,
                197u8,
                188u8,
                121u8,
                179u8,
                137u8,
                104u8,
                184u8,
                3u8,
                192u8,
                1u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    by_profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'by_profile_id' from topic of type 'uint256': {:?}",
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
                    id_of_profile_unblocked: {
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
        impl substreams_ethereum::Event for Unblocked {
            const NAME: &'static str = "Unblocked";
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
        pub struct Unfollowed {
            pub unfollower_profile_id: substreams::scalar::BigInt,
            pub id_of_profile_unfollowed: substreams::scalar::BigInt,
            pub transaction_executor: Vec<u8>,
            pub timestamp: substreams::scalar::BigInt,
        }
        impl Unfollowed {
            const TOPIC_ID: [u8; 32] = [
                155u8,
                186u8,
                220u8,
                77u8,
                41u8,
                248u8,
                65u8,
                107u8,
                59u8,
                30u8,
                214u8,
                254u8,
                123u8,
                66u8,
                204u8,
                53u8,
                136u8,
                170u8,
                202u8,
                116u8,
                42u8,
                200u8,
                193u8,
                102u8,
                27u8,
                59u8,
                176u8,
                164u8,
                197u8,
                171u8,
                22u8,
                115u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
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
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    unfollower_profile_id: {
                        let mut v = [0 as u8; 32];
                        ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'unfollower_profile_id' from topic of type 'uint256': {:?}",
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
                    id_of_profile_unfollowed: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    transaction_executor: values
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
        impl substreams_ethereum::Event for Unfollowed {
            const NAME: &'static str = "Unfollowed";
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