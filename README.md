# ocieguide readme

## Status
[![CI](https://github.com/damccull/ocieguide/actions/workflows/ci.yml/badge.svg)](https://github.com/damccull/ocieguide/actions/workflows/ci.yml) [![Security Audit](https://github.com/damccull/ocieguide/actions/workflows/dependency-security-audit.yml/badge.svg)](https://github.com/damccull/ocieguide/actions/workflows/dependency-security-audit.yml)
## Project Goal
This project aims to create and host a web application that will allow soldiers
to easily search for and identify their organizational clothing and individual
equipment (OCIE). The data should be both centrally supplied and crowd sourced,
allowing for a constantly updated database.

Soldiers should be able to search by NSN, NIIN, LIN, nomenclature, or any known
common names for equipment and see a high resolution photo along with all the
data about a piece of equipment. They should also be able to upload new photos
to existing equipment and supply data and photos for new pieces of equipment,
which should be moderated before public availability.

## Data Storage
All data should be stored as a series of events and be projected into searchable
tables.