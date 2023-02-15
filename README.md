# Circadian Tools &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] 

<!-- [![Code Coverage[codecov]]] -->

[Build Status]: https://github.com/BWStearns/circadian_tools/actions/workflows/circadian-tools-build.yml/badge.svg
[actions]: https://github.com/BWStearns/circadian_tools/actions/workflows/circadian-tools-build.yml?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/circadian_tools.svg
[crates.io]: https://crates.io/crates/circadian\_tools

Circadian data is data which is cyclical, like time of day, or day of week, etc. One could also imagine data which is cyclical in other ways, like the phases of the moon, seasons, or a sales cycle etc.

## Averages

One issue that can arise is averaging. Imagine a nightly data delivery. You want to know when the delivery usually occurs and your times look like `[23:00, 01:00, 23:30, 00:30]`. If you simply average the times of the delivery you will get noon, which is clearly and intutively wrong. Instead we first project the times onto a unit circle giving us cartesian X/Y coordinates. We then average the points generating a new point inside the circle. If we then draw a line from the origin through the point to the edge of the circle we get a new time which is average time of the delivery.

Additionally, we get a confidence measurement, which is the distance of the average point from the origin. A distance of 1 means that all the points are the same. A distance of 0 means that the points are equally distributed, so there's really no meaningful average. This can be useful if you want to know how predictive your average might be in the future. A use case here would be dynamically adjusting an alert threshold. It uses `f64` so it has some precision issues, but it's good enough for most purposes with some cleaning up of the output.

```rust 
use circadian_tools;

fn main() {
    let data = vec![23.0, 1.0];
    let (avg, conf) = circadian_tools::circadian_average(24.0, data.into_iter());
    println!("Average of 23 and 1 on a 24 hour cycle is {}, with {} confidence", avg, conf);
    // prints "Average of 23 and 1 on a 24 hour cycle is 0.0000..., with 0.9659... confidence"
}
```

There's also a function for averaging a vector of `Timelike` objects. This is useful if you have a vector of times and you want to know the average time of day.

```rust
use chrono::NaiveTime;
use circadian_tools;

fn main() {
    let data = vec![
        NaiveTime::from_hms_opt(1, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
    ];
    let avg_time = circadian_tools::avg_time_of_day(data.into_iter());
    println!("Average of 1100 and 0100 is {}", avg_time);
    // prints "Average of 1100 and 0100 is 0000"
}
```

## Let me know if you have suggestions!

This is a small library, starting with averaging, and is a work in progress. I will be adding more tools as I go. If you have any suggestions, please let me know.
