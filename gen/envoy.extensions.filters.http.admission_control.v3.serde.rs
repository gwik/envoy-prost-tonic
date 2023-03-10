// @generated
impl serde::Serialize for AdmissionControl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled.is_some() {
            len += 1;
        }
        if self.sampling_window.is_some() {
            len += 1;
        }
        if self.aggression.is_some() {
            len += 1;
        }
        if self.sr_threshold.is_some() {
            len += 1;
        }
        if self.rps_threshold.is_some() {
            len += 1;
        }
        if self.max_rejection_probability.is_some() {
            len += 1;
        }
        if self.evaluation_criteria.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl", len)?;
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.sampling_window.as_ref() {
            struct_ser.serialize_field("samplingWindow", v)?;
        }
        if let Some(v) = self.aggression.as_ref() {
            struct_ser.serialize_field("aggression", v)?;
        }
        if let Some(v) = self.sr_threshold.as_ref() {
            struct_ser.serialize_field("srThreshold", v)?;
        }
        if let Some(v) = self.rps_threshold.as_ref() {
            struct_ser.serialize_field("rpsThreshold", v)?;
        }
        if let Some(v) = self.max_rejection_probability.as_ref() {
            struct_ser.serialize_field("maxRejectionProbability", v)?;
        }
        if let Some(v) = self.evaluation_criteria.as_ref() {
            match v {
                admission_control::EvaluationCriteria::SuccessCriteria(v) => {
                    struct_ser.serialize_field("successCriteria", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdmissionControl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "sampling_window",
            "samplingWindow",
            "aggression",
            "sr_threshold",
            "srThreshold",
            "rps_threshold",
            "rpsThreshold",
            "max_rejection_probability",
            "maxRejectionProbability",
            "success_criteria",
            "successCriteria",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            SamplingWindow,
            Aggression,
            SrThreshold,
            RpsThreshold,
            MaxRejectionProbability,
            SuccessCriteria,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "samplingWindow" | "sampling_window" => Ok(GeneratedField::SamplingWindow),
                            "aggression" => Ok(GeneratedField::Aggression),
                            "srThreshold" | "sr_threshold" => Ok(GeneratedField::SrThreshold),
                            "rpsThreshold" | "rps_threshold" => Ok(GeneratedField::RpsThreshold),
                            "maxRejectionProbability" | "max_rejection_probability" => Ok(GeneratedField::MaxRejectionProbability),
                            "successCriteria" | "success_criteria" => Ok(GeneratedField::SuccessCriteria),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdmissionControl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.admission_control.v3.AdmissionControl")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AdmissionControl, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut sampling_window__ = None;
                let mut aggression__ = None;
                let mut sr_threshold__ = None;
                let mut rps_threshold__ = None;
                let mut max_rejection_probability__ = None;
                let mut evaluation_criteria__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                        GeneratedField::SamplingWindow => {
                            if sampling_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingWindow"));
                            }
                            sampling_window__ = map.next_value()?;
                        }
                        GeneratedField::Aggression => {
                            if aggression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggression"));
                            }
                            aggression__ = map.next_value()?;
                        }
                        GeneratedField::SrThreshold => {
                            if sr_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srThreshold"));
                            }
                            sr_threshold__ = map.next_value()?;
                        }
                        GeneratedField::RpsThreshold => {
                            if rps_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpsThreshold"));
                            }
                            rps_threshold__ = map.next_value()?;
                        }
                        GeneratedField::MaxRejectionProbability => {
                            if max_rejection_probability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRejectionProbability"));
                            }
                            max_rejection_probability__ = map.next_value()?;
                        }
                        GeneratedField::SuccessCriteria => {
                            if evaluation_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successCriteria"));
                            }
                            evaluation_criteria__ = map.next_value::<::std::option::Option<_>>()?.map(admission_control::EvaluationCriteria::SuccessCriteria)
;
                        }
                    }
                }
                Ok(AdmissionControl {
                    enabled: enabled__,
                    sampling_window: sampling_window__,
                    aggression: aggression__,
                    sr_threshold: sr_threshold__,
                    rps_threshold: rps_threshold__,
                    max_rejection_probability: max_rejection_probability__,
                    evaluation_criteria: evaluation_criteria__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for admission_control::SuccessCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_criteria.is_some() {
            len += 1;
        }
        if self.grpc_criteria.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria", len)?;
        if let Some(v) = self.http_criteria.as_ref() {
            struct_ser.serialize_field("httpCriteria", v)?;
        }
        if let Some(v) = self.grpc_criteria.as_ref() {
            struct_ser.serialize_field("grpcCriteria", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for admission_control::SuccessCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_criteria",
            "httpCriteria",
            "grpc_criteria",
            "grpcCriteria",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpCriteria,
            GrpcCriteria,
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
                            "httpCriteria" | "http_criteria" => Ok(GeneratedField::HttpCriteria),
                            "grpcCriteria" | "grpc_criteria" => Ok(GeneratedField::GrpcCriteria),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = admission_control::SuccessCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<admission_control::SuccessCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_criteria__ = None;
                let mut grpc_criteria__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpCriteria => {
                            if http_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpCriteria"));
                            }
                            http_criteria__ = map.next_value()?;
                        }
                        GeneratedField::GrpcCriteria => {
                            if grpc_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcCriteria"));
                            }
                            grpc_criteria__ = map.next_value()?;
                        }
                    }
                }
                Ok(admission_control::SuccessCriteria {
                    http_criteria: http_criteria__,
                    grpc_criteria: grpc_criteria__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for admission_control::success_criteria::GrpcCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grpc_success_status.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.GrpcCriteria", len)?;
        if !self.grpc_success_status.is_empty() {
            struct_ser.serialize_field("grpcSuccessStatus", &self.grpc_success_status)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for admission_control::success_criteria::GrpcCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_success_status",
            "grpcSuccessStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcSuccessStatus,
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
                            "grpcSuccessStatus" | "grpc_success_status" => Ok(GeneratedField::GrpcSuccessStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = admission_control::success_criteria::GrpcCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.GrpcCriteria")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<admission_control::success_criteria::GrpcCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_success_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcSuccessStatus => {
                            if grpc_success_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcSuccessStatus"));
                            }
                            grpc_success_status__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(admission_control::success_criteria::GrpcCriteria {
                    grpc_success_status: grpc_success_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.GrpcCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for admission_control::success_criteria::HttpCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.http_success_status.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.HttpCriteria", len)?;
        if !self.http_success_status.is_empty() {
            struct_ser.serialize_field("httpSuccessStatus", &self.http_success_status)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for admission_control::success_criteria::HttpCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_success_status",
            "httpSuccessStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpSuccessStatus,
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
                            "httpSuccessStatus" | "http_success_status" => Ok(GeneratedField::HttpSuccessStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = admission_control::success_criteria::HttpCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.HttpCriteria")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<admission_control::success_criteria::HttpCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_success_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpSuccessStatus => {
                            if http_success_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpSuccessStatus"));
                            }
                            http_success_status__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(admission_control::success_criteria::HttpCriteria {
                    http_success_status: http_success_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.admission_control.v3.AdmissionControl.SuccessCriteria.HttpCriteria", FIELDS, GeneratedVisitor)
    }
}
