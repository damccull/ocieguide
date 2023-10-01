# Design Doc

## Events DB Representation
All events will contain the following metadata as database fields. Context about the type of event will be provided by its name in rust and the event_type field in the database.

* event_id
* ocieitem_id
* event_type
* created_timestamp
* created_by
* change_vector

All events will contain json that contains arbitrary data fields. The deserialization will pick up only the fields it cares about, ignoring the rest.
