---
id: us-ca
type: state-broadband-market
scale: regional
market: California
tier: T2
score.DIM-01: 9.2
score.DIM-02: 9
quantity: 94.2 | percent-households-with-broadband | cited | acs-s2801-2023
quantity: 94.0 | percent-us-locations-served-100-20-context | cited | fcc-nbm-2024
---

# California broadband market (regional scale)

DIM-01 (access/adoption adequacy) is derived from the cited ACS 2023 household
broadband subscription rate (94.2%) via a stated transform
adequacy = clamp(adoption_percent - 85, 0, 10). DIM-02 (availability adequacy at
100/20 Mbps) is an **estimate** anchored to the FCC National Broadband Map
(~94% of US locations served as of Dec 2024) and its documented rural-coverage
deficits; no precise per-state served-percentage is asserted here.
