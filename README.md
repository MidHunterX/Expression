# Expression

## Requirements

- [ ] 1. Change wallpapers with any backends. Like swww, feh etc.
- [ ] 2. 24 hour wallpaper cycling. Dedicated wallpaper for each hour.
- [ ] 3. Higher priority special wallpapers which overrides normal cycle.
- [ ] 4. Collections
- [ ] 5. Collection Flow control

- [ ] Debug Logging
- [ ] Detailed Testing

### Though process

Requirement 3 & 4: High priority wallpapers and collections

- Currently this feature is being implemented via setting env variables in format `T_HHMM`.
- Then checking if that variable is set else follow lower priority (24 hour cycling)
- A better way would be setting high priority dir (override) and low priority dir (24 hour cycle).
- Maybe even scale it to multiple low priority dirs (collections).
- Collections can be a collection of wallpapers with similar theme (e.g. nature, 24_hour, dark_mode).

Requirement 5: Collection Flow Control

- Seeing different collections, it would be nice to set different collections at different time of the day.
- Like light wallpapers during the day and dark wallpapers during the night.

## Implementation

### Switchable Backends

## Behaviors

- If backend is not set up, this script will initialize it for you and run rest of the script anyway.
