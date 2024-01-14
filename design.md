# Application Design

## API

Should I do normal REST, gRPC, , or perhaps more than one?
REST is tried and true.
gRPC is fast and type-safe.
GraphQL lets the client specify exactly what data is desired.

Answer: Use gRPC or REST to start. Add/migrate to graphql later if desired.

## User Interface

wasm app with API calls. Use leptos for UI.

Need an area for users to submit photos or change requests for an item. Link these to the item record id.
Build this with the very basics, to start. Create a basic form to offer suggested changes, similar to
github issues. Just a box to type in, maybe a couple metadata fields.

Photo uploads should get sent to a temporary storage area on the S3 protocol with Cloudflare R2 and await
approval, where they will be converted to appropriate resolution and file size, then stored in the images
library.

Images library will be free if R2, but handles resolutions automatically if using Cloudflare Images ($5/month).

One image per item.

Images will be named with a prefix of the item's db record id, then then resolution.

## Data Storage

Database will hold individual items.

Searches will need to search both items table and common names table, and if found in either location
look up the item in items table and display it in results.

### Items Table

* id (pk)
* lin
* nsn
* ets_transferrable
* nomenclature
* size
* cic
* ui
* unit_price

### Common Names Table

* item_id (pk, fk)
* common_name (pk)

## Admin Area

Regular HTML-based site, nothing fancy. Allow admins to read suggestions and approve photos.

## Plan of Work

1. Learn Leptos
2. Build back end API
