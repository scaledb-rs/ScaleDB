# ScaleDB&trade;
ScaleDB is the flexible open-source data store.

# Overall goal
ScaleDB is designed to provide a consistent yet flexible framework for data storage in server, client, mobile and edge applications.

# Status
- ScaleDB is in active development (pre-production)
- Please watch this project to be notified of milestone releases

# Feature goals
- Store structured, semi-structured and unstructured data
- Store data in-memory, with reliable persistence to permanent storage
- Named collections (including the default empty-name collection) store key/value pairs
  - Keys are UTF-8 encoded strings (or uint8 vectors) by default
  - Values can be any of the following types:
    - Primitive types (e.g. boolean, string, float64)
    - Structured data (e.g. strongly-typed structs) 
    - Semi-structured data (e.g. dynamically-typed objects)
    - Unstructured data (e.g. text files)
  - Strongly-typed data can contain dynamically-typed fields (see "object" type)
  - Structured data also supports optionals (nullable types)
- Expire specified values at a certain timestamp or after a defined duration
- Multiple runtime scenarios
  - Run ScaleDB out of process (to share data among applications)
  - Embed the ScaleDB engine directly into your custom application
  - Support planned for edge scenarios in the future
- Client libraries for Rust and C#
  - Software clients can define/manipulate collections and serialize/deserialize objects using language-native models
  - Support planned for additional programming languages in the future
- Supports Linux, macOS and Windows
  - Support planned for additional platforms in the future

# Supported data types
- int8, int16, int32, int64, int128
- uint8, uint16, uint32, uint64, uint128
- float32, float64
- boolean
- object (accepts any type)
- vector

# Convenience data types
- uint8 vector (i.e. byte array)
- string (a UTF-8 encoded representation of a uint8 vector)
- timestamp (NOTE: if generated using system time, not guaranteed to be monotonic)

# Potential applications
- Queues
- Rate limiting
- In-memory cache
- Session data store
- Real-time analytics

# Sample structured data (pseudocode)
```
/* NOTE: for convenience, the key (i.e. unique ID) may also be mirrored as a struct field (not shown here);
   that behavior is consistent with many NoSQL databases (which represent ID as a field in each record instance) */

struct automobile {
    number_of_wheels: uint8,
    color: string,
    mileage: uint32,
    license_tag: Option<string>
}
```

# Sample semi-structured data (pseudocode)
```
// NOTE: dynamic data may contain extra fields, fewer fields, or even fields using inconsistent types
var festival = object {
    name: "Storage Fest",
    max_attendees: 3000,
    amenities: [
        "snacks",
        "entertainment"
    ]
}
```

# Legal notice
ScaleDB is a trademark of ScaleDB LLC, the organization that maintains the ScaleDB open source project.

The source code is licensed under a permissive open-source license (MIT). This license deals primarily with copyright and does not grant license to trademark(s). 

You may use the project's trademark(s) as reasonably required for customary use in describing the origin of the original work. Thank you for respecting the project's trademark(s).
