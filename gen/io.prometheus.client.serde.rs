// @generated
impl serde::Serialize for Bucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cumulative_count.is_some() {
            len += 1;
        }
        if self.upper_bound.is_some() {
            len += 1;
        }
        if self.exemplar.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Bucket", len)?;
        if let Some(v) = self.cumulative_count.as_ref() {
            struct_ser.serialize_field("cumulativeCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.upper_bound.as_ref() {
            struct_ser.serialize_field("upperBound", v)?;
        }
        if let Some(v) = self.exemplar.as_ref() {
            struct_ser.serialize_field("exemplar", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cumulative_count",
            "cumulativeCount",
            "upper_bound",
            "upperBound",
            "exemplar",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CumulativeCount,
            UpperBound,
            Exemplar,
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
                            "cumulativeCount" | "cumulative_count" => Ok(GeneratedField::CumulativeCount),
                            "upperBound" | "upper_bound" => Ok(GeneratedField::UpperBound),
                            "exemplar" => Ok(GeneratedField::Exemplar),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Bucket")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Bucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cumulative_count__ = None;
                let mut upper_bound__ = None;
                let mut exemplar__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CumulativeCount => {
                            if cumulative_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cumulativeCount"));
                            }
                            cumulative_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UpperBound => {
                            if upper_bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upperBound"));
                            }
                            upper_bound__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Exemplar => {
                            if exemplar__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exemplar"));
                            }
                            exemplar__ = map.next_value()?;
                        }
                    }
                }
                Ok(Bucket {
                    cumulative_count: cumulative_count__,
                    upper_bound: upper_bound__,
                    exemplar: exemplar__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Bucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Counter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        if self.exemplar.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Counter", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.exemplar.as_ref() {
            struct_ser.serialize_field("exemplar", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Counter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "exemplar",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Exemplar,
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
                            "value" => Ok(GeneratedField::Value),
                            "exemplar" => Ok(GeneratedField::Exemplar),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Counter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Counter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Counter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut exemplar__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Exemplar => {
                            if exemplar__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exemplar"));
                            }
                            exemplar__ = map.next_value()?;
                        }
                    }
                }
                Ok(Counter {
                    value: value__,
                    exemplar: exemplar__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Counter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Exemplar {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.label.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Exemplar", len)?;
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Exemplar {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "label",
            "value",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Label,
            Value,
            Timestamp,
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
                            "label" => Ok(GeneratedField::Label),
                            "value" => Ok(GeneratedField::Value),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Exemplar;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Exemplar")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Exemplar, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut label__ = None;
                let mut value__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                    }
                }
                Ok(Exemplar {
                    label: label__.unwrap_or_default(),
                    value: value__,
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Exemplar", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Gauge {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Gauge", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Gauge {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Gauge;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Gauge")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Gauge, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Gauge {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Gauge", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Histogram {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample_count.is_some() {
            len += 1;
        }
        if self.sample_sum.is_some() {
            len += 1;
        }
        if !self.bucket.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Histogram", len)?;
        if let Some(v) = self.sample_count.as_ref() {
            struct_ser.serialize_field("sampleCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.sample_sum.as_ref() {
            struct_ser.serialize_field("sampleSum", v)?;
        }
        if !self.bucket.is_empty() {
            struct_ser.serialize_field("bucket", &self.bucket)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Histogram {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample_count",
            "sampleCount",
            "sample_sum",
            "sampleSum",
            "bucket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SampleCount,
            SampleSum,
            Bucket,
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
                            "sampleCount" | "sample_count" => Ok(GeneratedField::SampleCount),
                            "sampleSum" | "sample_sum" => Ok(GeneratedField::SampleSum),
                            "bucket" => Ok(GeneratedField::Bucket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Histogram;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Histogram")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Histogram, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample_count__ = None;
                let mut sample_sum__ = None;
                let mut bucket__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SampleCount => {
                            if sample_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleCount"));
                            }
                            sample_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SampleSum => {
                            if sample_sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleSum"));
                            }
                            sample_sum__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Bucket => {
                            if bucket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucket"));
                            }
                            bucket__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Histogram {
                    sample_count: sample_count__,
                    sample_sum: sample_sum__,
                    bucket: bucket__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Histogram", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LabelPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.LabelPair", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LabelPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
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
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LabelPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.LabelPair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LabelPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(LabelPair {
                    name: name__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.LabelPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.label.is_empty() {
            len += 1;
        }
        if self.gauge.is_some() {
            len += 1;
        }
        if self.counter.is_some() {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        if self.untyped.is_some() {
            len += 1;
        }
        if self.histogram.is_some() {
            len += 1;
        }
        if self.timestamp_ms.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Metric", len)?;
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if let Some(v) = self.gauge.as_ref() {
            struct_ser.serialize_field("gauge", v)?;
        }
        if let Some(v) = self.counter.as_ref() {
            struct_ser.serialize_field("counter", v)?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        if let Some(v) = self.untyped.as_ref() {
            struct_ser.serialize_field("untyped", v)?;
        }
        if let Some(v) = self.histogram.as_ref() {
            struct_ser.serialize_field("histogram", v)?;
        }
        if let Some(v) = self.timestamp_ms.as_ref() {
            struct_ser.serialize_field("timestampMs", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "label",
            "gauge",
            "counter",
            "summary",
            "untyped",
            "histogram",
            "timestamp_ms",
            "timestampMs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Label,
            Gauge,
            Counter,
            Summary,
            Untyped,
            Histogram,
            TimestampMs,
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
                            "label" => Ok(GeneratedField::Label),
                            "gauge" => Ok(GeneratedField::Gauge),
                            "counter" => Ok(GeneratedField::Counter),
                            "summary" => Ok(GeneratedField::Summary),
                            "untyped" => Ok(GeneratedField::Untyped),
                            "histogram" => Ok(GeneratedField::Histogram),
                            "timestampMs" | "timestamp_ms" => Ok(GeneratedField::TimestampMs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Metric")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Metric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut label__ = None;
                let mut gauge__ = None;
                let mut counter__ = None;
                let mut summary__ = None;
                let mut untyped__ = None;
                let mut histogram__ = None;
                let mut timestamp_ms__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map.next_value()?);
                        }
                        GeneratedField::Gauge => {
                            if gauge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gauge"));
                            }
                            gauge__ = map.next_value()?;
                        }
                        GeneratedField::Counter => {
                            if counter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counter"));
                            }
                            counter__ = map.next_value()?;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map.next_value()?;
                        }
                        GeneratedField::Untyped => {
                            if untyped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("untyped"));
                            }
                            untyped__ = map.next_value()?;
                        }
                        GeneratedField::Histogram => {
                            if histogram__.is_some() {
                                return Err(serde::de::Error::duplicate_field("histogram"));
                            }
                            histogram__ = map.next_value()?;
                        }
                        GeneratedField::TimestampMs => {
                            if timestamp_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampMs"));
                            }
                            timestamp_ms__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Metric {
                    label: label__.unwrap_or_default(),
                    gauge: gauge__,
                    counter: counter__,
                    summary: summary__,
                    untyped: untyped__,
                    histogram: histogram__,
                    timestamp_ms: timestamp_ms__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Metric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricFamily {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.help.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        if !self.metric.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.MetricFamily", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.help.as_ref() {
            struct_ser.serialize_field("help", v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            let v = MetricType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.metric.is_empty() {
            struct_ser.serialize_field("metric", &self.metric)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricFamily {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "help",
            "type",
            "metric",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Help,
            Type,
            Metric,
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
                            "name" => Ok(GeneratedField::Name),
                            "help" => Ok(GeneratedField::Help),
                            "type" => Ok(GeneratedField::Type),
                            "metric" => Ok(GeneratedField::Metric),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricFamily;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.MetricFamily")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MetricFamily, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut help__ = None;
                let mut r#type__ = None;
                let mut metric__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Help => {
                            if help__.is_some() {
                                return Err(serde::de::Error::duplicate_field("help"));
                            }
                            help__ = map.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<MetricType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Metric => {
                            if metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metric"));
                            }
                            metric__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MetricFamily {
                    name: name__,
                    help: help__,
                    r#type: r#type__,
                    metric: metric__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.MetricFamily", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Counter => "COUNTER",
            Self::Gauge => "GAUGE",
            Self::Summary => "SUMMARY",
            Self::Untyped => "UNTYPED",
            Self::Histogram => "HISTOGRAM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MetricType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COUNTER",
            "GAUGE",
            "SUMMARY",
            "UNTYPED",
            "HISTOGRAM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(MetricType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(MetricType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COUNTER" => Ok(MetricType::Counter),
                    "GAUGE" => Ok(MetricType::Gauge),
                    "SUMMARY" => Ok(MetricType::Summary),
                    "UNTYPED" => Ok(MetricType::Untyped),
                    "HISTOGRAM" => Ok(MetricType::Histogram),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Quantile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quantile.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Quantile", len)?;
        if let Some(v) = self.quantile.as_ref() {
            struct_ser.serialize_field("quantile", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quantile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quantile",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Quantile,
            Value,
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
                            "quantile" => Ok(GeneratedField::Quantile),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quantile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Quantile")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Quantile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quantile__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Quantile => {
                            if quantile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantile"));
                            }
                            quantile__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Quantile {
                    quantile: quantile__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Quantile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Summary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample_count.is_some() {
            len += 1;
        }
        if self.sample_sum.is_some() {
            len += 1;
        }
        if !self.quantile.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Summary", len)?;
        if let Some(v) = self.sample_count.as_ref() {
            struct_ser.serialize_field("sampleCount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.sample_sum.as_ref() {
            struct_ser.serialize_field("sampleSum", v)?;
        }
        if !self.quantile.is_empty() {
            struct_ser.serialize_field("quantile", &self.quantile)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Summary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample_count",
            "sampleCount",
            "sample_sum",
            "sampleSum",
            "quantile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SampleCount,
            SampleSum,
            Quantile,
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
                            "sampleCount" | "sample_count" => Ok(GeneratedField::SampleCount),
                            "sampleSum" | "sample_sum" => Ok(GeneratedField::SampleSum),
                            "quantile" => Ok(GeneratedField::Quantile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Summary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Summary")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Summary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample_count__ = None;
                let mut sample_sum__ = None;
                let mut quantile__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SampleCount => {
                            if sample_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleCount"));
                            }
                            sample_count__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::SampleSum => {
                            if sample_sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleSum"));
                            }
                            sample_sum__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Quantile => {
                            if quantile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantile"));
                            }
                            quantile__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Summary {
                    sample_count: sample_count__,
                    sample_sum: sample_sum__,
                    quantile: quantile__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Summary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Untyped {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("io.prometheus.client.Untyped", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Untyped {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Untyped;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct io.prometheus.client.Untyped")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Untyped, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Untyped {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("io.prometheus.client.Untyped", FIELDS, GeneratedVisitor)
    }
}
