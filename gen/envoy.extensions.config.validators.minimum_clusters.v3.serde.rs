// @generated
impl serde::Serialize for MinimumClustersValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_clusters_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.config.validators.minimum_clusters.v3.MinimumClustersValidator", len)?;
        if self.min_clusters_num != 0 {
            struct_ser.serialize_field("minClustersNum", &self.min_clusters_num)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinimumClustersValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_clusters_num",
            "minClustersNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinClustersNum,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minClustersNum" | "min_clusters_num" => Ok(GeneratedField::MinClustersNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MinimumClustersValidator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.config.validators.minimum_clusters.v3.MinimumClustersValidator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MinimumClustersValidator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_clusters_num__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MinClustersNum => {
                            if min_clusters_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minClustersNum"));
                            }
                            min_clusters_num__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MinimumClustersValidator {
                    min_clusters_num: min_clusters_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.config.validators.minimum_clusters.v3.MinimumClustersValidator", FIELDS, GeneratedVisitor)
    }
}
