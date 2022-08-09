# CLI for interacting with the API

You can visit the [Roblox Documentation](https://create.roblox.com/docs/open-cloud/data-store-api) for more information about the API

## List Data Stores

List all Datastores in the specified "universe" or game

```
USAGE:
    rbx-ds-cloud list-data-stores [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --limit <LIMIT>

OPTIONS:
    -a, --api-key <API_KEY>            API key of Roblox Open Cloud
    -c, --cursor <CURSOR>              (Optional) Cursor for the next set of data
    -l, --limit <LIMIT>                (INTEGER) Maximum number of items to return
    -p, --prefix <PREFIX>              (Optional) Return only data stores with this prefix
    -u, --universe-id <UNIVERSE_ID>    The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
```

## List Entries

Returns a list of entry keys within a data store

```
USAGE:
    rbx-ds-cloud list-entries [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --limit <LIMIT>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -c, --cursor <CURSOR> (Optional) Provide to request the next set of data
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -l, --limit <LIMIT> (INTEGER) Maximum number of items to return
    -o, --all-scopes <ALL_SCOPES> If true, return keys from all scopes [possible values: true, false]
    -p, --prefix <PREFIX> (Optional) Return only keys with this prefix
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure
    page
```

## Get Entry

Returns the value associated with an entry

```
USAGE:
    rbx-ds-cloud get-entry [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -k, --key <KEY> The key which identifies the entry
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
```

## Set Entry

Sets the value, metadata and user IDs associated with an entry

```
USAGE:
    rbx-ds-cloud set-entry [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY> --data <DATA>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -D, --data <DATA> JSON-stringified or stringify-able data (Limit: 4MB)
    -e, --exclusive-create <EXCLUSIVE_CREATE> (Optional) Only create the entry if it does not exist [possible values: true ,false]
    -i, --match-version <MATCH_VERSION> (Optional) Only update if current version matches this
    -k, --key <KEY>The key which identifies the entry
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -t, --attributes <ATTRIBUTES> (Optional) JSON-stringified attributes data
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
    -U, --user-ids <USER_IDS> (Optional) Associated UserID (can be multiple)
```

## Increment Entry

Increments the value for an entry by a given amount, or create a new entry with that amount

```
USAGE:
    rbx-ds-cloud increment-entry [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY> --increment-by <INCREMENT_BY>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -i, --increment-by <INCREMENT_BY> The amount by which the entry should be incremented
    -k, --key <KEY> The key which identifies the entry
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -t, --attributes <ATTRIBUTES> (Optional) JSON-stringified attributes data
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure
    page
    -U, --user-ids <USER_IDS> (Optional) Comma-separated list of Roblox user IDs
```

## Delete Entry

Marks the entry as deleted by creating a 'tombstone' version. Entries are deleted permanently after 30 days

```
USAGE:
    rbx-ds-cloud delete-entry [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -k, --key <KEY> The key which identifies the entry
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
```

## List Entry Versions

Returns the versions and metadata of an Entry of a datastore

```
USAGE:
    rbx-ds-cloud list-entry-versions [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY> --sort-order <SORT_ORDER> --limit <LIMIT>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -c, --cursor <CURSOR> (Optional) Cursor for the next set of data
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -e, --end-time <END_TIME> (Optional) End time constraint (ISO UTC Datetime)
    -k, --key <KEY> The key which identifies the entry
    -l, --limit <LIMIT> (INTEGER) Maximum number of items to return
    -o, --sort-order <SORT_ORDER> Sort order [possible values: ascending, descending]
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -t, --start-time <START_TIME> (Optional) Start time constraint (ISO UTC Datetime)
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
```

## Get Entry Version

Returns the metadata of a specific version of an entry

```
USAGE:
    rbx-ds-cloud get-entry-version [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY> --datastore-name <DATASTORE_NAME> --key <KEY> --version-id <VERSION_ID>

OPTIONS:
    -a, --api-key <API_KEY> API key of Roblox Open Cloud
    -d, --datastore-name <DATASTORE_NAME> Name of the data store
    -k, --key <KEY> The key which identifies the entry
    -s, --scope <SCOPE> (Optional) If "None", defaults to global, similar to Lua API
    -u, --universe-id <UNIVERSE_ID> The value of DataModel.GameId, which is visible in the URL on the universe's Configure page
    -v, --version-id <VERSION_ID> The version of the key
```
