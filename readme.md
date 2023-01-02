This project manages real time data transit data for the greater Paris region.

The data comes from the official website https://prim.iledefrance-mobilites.fr/

We cache the data to allow other users to acces rapidly the data without having to login.

## Data sources

* _EstimatedTimetable_: all vehicles that are currently running in the [SIRI Lite](http://www.normes-donnees-tc.org/format-dechange/donnees-temps-reel/) format
* _Stops_ and _Lines_ referential: two custom CSV files
* [GTFS](https://gtfs.org/): theoretical timetables. We don’t use in this project, but we proxy it.

## Technical stuff

* Written in [Rust](https://www.rust-lang.org/)
* Using the [Actix](https://actix.rs/) framework (both web and actors)
* With [Tera](https://tera.netlify.app/) templates

The actors are organised as follows

```
Fetchers # Get fresh data
  - Siri # Every 90 seconds
  - GTFS # Every hour
  - LineReferential # Every hour
  - StopReferential # Every hour
States # Central repository for data
  - Templates # Html stuff
  - PublicTransitData #
Sessions # All the connexions
  - CentralDispatch # Knows every who’s watching a page
  - SessionActor # One for every browser following a page
```
