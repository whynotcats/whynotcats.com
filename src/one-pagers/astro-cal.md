###### Generate calendar events for predictable photo events such as moonrise/sunrise, golden hour, etc.

This idea came from accidently witnessing a super moon rising behind Mt. Baker while on a walk and realizing that I couldn't be bothered to consistently remember when the moon was going to rise in the day and what phase it was in. 

#### Definition of Done

Generate a `.ics` file of calendar events that can be imported into gcal. This file type can be used by most calendar software and generating it doesn't require any OAuth or interaction with external APIs.

#### Dependencies and Constraints

The heavy lifting for this is going to be the astro library used to generate astrological event times. For convience we'll start with something that has the exact convience features we need (get moonrise/set times from a location.) Eventually we should move to a move flexible library once I brush up on my astronomy and can generate the calculations myself.

#### Definition of Awesome

Google Calendar integration
Filtering by position in the sky (such as when the moon rises behind a mountain)
List phase of moon and allow filtering on phase of moon
