# Changelog

## Since v1.0.0 (unreleased)

### Breaking changes

- Replaced --color with a positional Vec. Example usage: `tint_and_shade '#808080' '#123456' -p 20`
- Renamed binary from tint_and_shade to tint-and-shade

### Other changes

- Colors are now validated and skipped if invalid
- The percentage must now be between 0-100 (inclusive)
- Improved error and warning styling
- rgb colors are now accepted in CLI arguments in rgb(r, g, b) format
- Added descriptions to CLI arguments
- Added basic output mode
