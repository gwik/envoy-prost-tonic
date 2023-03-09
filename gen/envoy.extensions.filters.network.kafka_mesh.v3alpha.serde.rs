// @generated
impl serde::Serialize for ForwardingRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_cluster.is_empty() {
            len += 1;
        }
        if self.trigger.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.ForwardingRule", len)?;
        if !self.target_cluster.is_empty() {
            struct_ser.serialize_field("targetCluster", &self.target_cluster)?;
        }
        if let Some(v) = self.trigger.as_ref() {
            match v {
                forwarding_rule::Trigger::TopicPrefix(v) => {
                    struct_ser.serialize_field("topicPrefix", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardingRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_cluster",
            "targetCluster",
            "topic_prefix",
            "topicPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetCluster,
            TopicPrefix,
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
                            "targetCluster" | "target_cluster" => Ok(GeneratedField::TargetCluster),
                            "topicPrefix" | "topic_prefix" => Ok(GeneratedField::TopicPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardingRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.kafka_mesh.v3alpha.ForwardingRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ForwardingRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_cluster__ = None;
                let mut trigger__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TargetCluster => {
                            if target_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetCluster"));
                            }
                            target_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::TopicPrefix => {
                            if trigger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topicPrefix"));
                            }
                            trigger__ = map.next_value::<::std::option::Option<_>>()?.map(forwarding_rule::Trigger::TopicPrefix);
                        }
                    }
                }
                Ok(ForwardingRule {
                    target_cluster: target_cluster__.unwrap_or_default(),
                    trigger: trigger__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.ForwardingRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KafkaClusterDefinition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.bootstrap_servers.is_empty() {
            len += 1;
        }
        if self.partition_count != 0 {
            len += 1;
        }
        if !self.producer_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaClusterDefinition", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.bootstrap_servers.is_empty() {
            struct_ser.serialize_field("bootstrapServers", &self.bootstrap_servers)?;
        }
        if self.partition_count != 0 {
            struct_ser.serialize_field("partitionCount", &self.partition_count)?;
        }
        if !self.producer_config.is_empty() {
            struct_ser.serialize_field("producerConfig", &self.producer_config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KafkaClusterDefinition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "bootstrap_servers",
            "bootstrapServers",
            "partition_count",
            "partitionCount",
            "producer_config",
            "producerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            BootstrapServers,
            PartitionCount,
            ProducerConfig,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "bootstrapServers" | "bootstrap_servers" => Ok(GeneratedField::BootstrapServers),
                            "partitionCount" | "partition_count" => Ok(GeneratedField::PartitionCount),
                            "producerConfig" | "producer_config" => Ok(GeneratedField::ProducerConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KafkaClusterDefinition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaClusterDefinition")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KafkaClusterDefinition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut bootstrap_servers__ = None;
                let mut partition_count__ = None;
                let mut producer_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::BootstrapServers => {
                            if bootstrap_servers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrapServers"));
                            }
                            bootstrap_servers__ = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionCount => {
                            if partition_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionCount"));
                            }
                            partition_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProducerConfig => {
                            if producer_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("producerConfig"));
                            }
                            producer_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(KafkaClusterDefinition {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    bootstrap_servers: bootstrap_servers__.unwrap_or_default(),
                    partition_count: partition_count__.unwrap_or_default(),
                    producer_config: producer_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaClusterDefinition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KafkaMesh {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.advertised_host.is_empty() {
            len += 1;
        }
        if self.advertised_port != 0 {
            len += 1;
        }
        if !self.upstream_clusters.is_empty() {
            len += 1;
        }
        if !self.forwarding_rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaMesh", len)?;
        if !self.advertised_host.is_empty() {
            struct_ser.serialize_field("advertisedHost", &self.advertised_host)?;
        }
        if self.advertised_port != 0 {
            struct_ser.serialize_field("advertisedPort", &self.advertised_port)?;
        }
        if !self.upstream_clusters.is_empty() {
            struct_ser.serialize_field("upstreamClusters", &self.upstream_clusters)?;
        }
        if !self.forwarding_rules.is_empty() {
            struct_ser.serialize_field("forwardingRules", &self.forwarding_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KafkaMesh {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "advertised_host",
            "advertisedHost",
            "advertised_port",
            "advertisedPort",
            "upstream_clusters",
            "upstreamClusters",
            "forwarding_rules",
            "forwardingRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AdvertisedHost,
            AdvertisedPort,
            UpstreamClusters,
            ForwardingRules,
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
                            "advertisedHost" | "advertised_host" => Ok(GeneratedField::AdvertisedHost),
                            "advertisedPort" | "advertised_port" => Ok(GeneratedField::AdvertisedPort),
                            "upstreamClusters" | "upstream_clusters" => Ok(GeneratedField::UpstreamClusters),
                            "forwardingRules" | "forwarding_rules" => Ok(GeneratedField::ForwardingRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KafkaMesh;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaMesh")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KafkaMesh, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut advertised_host__ = None;
                let mut advertised_port__ = None;
                let mut upstream_clusters__ = None;
                let mut forwarding_rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AdvertisedHost => {
                            if advertised_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("advertisedHost"));
                            }
                            advertised_host__ = Some(map.next_value()?);
                        }
                        GeneratedField::AdvertisedPort => {
                            if advertised_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("advertisedPort"));
                            }
                            advertised_port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpstreamClusters => {
                            if upstream_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamClusters"));
                            }
                            upstream_clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::ForwardingRules => {
                            if forwarding_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardingRules"));
                            }
                            forwarding_rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(KafkaMesh {
                    advertised_host: advertised_host__.unwrap_or_default(),
                    advertised_port: advertised_port__.unwrap_or_default(),
                    upstream_clusters: upstream_clusters__.unwrap_or_default(),
                    forwarding_rules: forwarding_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.kafka_mesh.v3alpha.KafkaMesh", FIELDS, GeneratedVisitor)
    }
}
