# Circadian Tools

Circadian data is data which is cyclical, like time of day, or day of week, etc. One could also imagine data which is cyclical in other ways, like the phases of the moon, seasons, or a sales cycle etc.

## Averages

One issue that can arise is averaging. Imagine a nightly data delivery. You want to know when the delivery usually occurs and your times look like `[23:00, 01:00, 23:30, 00:30]`. If you simply average the times of the delivery you will get noon, which is clearly and intutively wrong. Instead we first project the times onto a unit circle giving us cartesian X/Y coordinates. We then average the points generating a new point inside the circle. If we then draw a line from the origin through the point to the edge of the circle we get a new time which is average time of the delivery.

Additionally, we get a confidence measurement, which is the distance of the average point from the origin. A distance of 1 means that all the points are the same. A distance of 0 means that the points are equally distributed, so there's really no meaningful average. This can be useful if you want to know how predictive your average might be in the future. A use case here would be dynamically adjusting an alert threshold.

## Let me know if you have suggestions!

This is a small library, starting with averaging, and is a work in progress. I will be adding more tools as I go. If you have any suggestions, please let me know.
