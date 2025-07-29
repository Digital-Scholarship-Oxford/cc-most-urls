#import "@preview/diatypst:0.6.0": *
#set text(font: "Atkinson Hyperlegible")
#show: slides.with(
  title: "How long are most urls?",
  subtitle: "An exercise in using the Common Crawl dataset",
  date: datetime.today().display("[year]-[month]-[day]"),
  authors: "Pierre Marshall",
  ratio: 16 / 9,
  layout: "small",
  title-color: rgb("#002147"),
  footer: false,
  toc: false,
)

= Introduction

== Why

URIs are printed
URIs are in browser windows (screenshot of browser window showing https://theofficialabsolutelongestdomainnameregisteredontheworldwideweb.international/)

Writing code where you want to know how long the url is in order to pre-allocate memory.


== Prior work

Kelvin Tan did this in 2010 with 6,627,999 unique urls

https://web.archive.org/web/20240114201737/https://www.supermind.org/blog/740/average-length-of-a-url-part-2


== What are the limits?

// "Maximum URL length is 2,083 characters in Internet Explorer"

// how long is the longest url in this dataset?

RFC 7231

The 414 (URI Too Long) status code indicates that the server is refusing to service the request because the request-target (Section 5.3 of [RFC7230]) is longer than the server is willing to interpret.

== Method

Common crawl provides a crawl index, which you can download.

```bash
#!/bin/bash

while read p; do
 wget "https://data.commoncrawl.org/${p}"
done < cc-index.paths
```

In jsonl format

walk through each line in the file

// There are for sure better ways of doing this!
// Talk about parquet, crude text processing.

== Filtering

invalid 100
duplicated 200

Total urls involved in this analysis

== Methodology

Internationalized domain names - IDNA

https://haurakicollective.maori.nz

https://haurakicollective.māori.nz

URL encoding

https://www.example.com/%e8%ae%af

https://www.example.com/讯

= Results

== Frequency graph 1

Stacked bar chart of normal and i18n count, and a total line showing accumulated trend.

== Frequency graph 2

Stacked bar chart of i18n count broken down by status code.

== Average tables

#show figure.where(
  kind: table,
): set figure.caption(position: top)

#figure(
  table(
    columns: 7,
    table.header([averages], [100], [200], [300], [400], [500], [total]),
    [mean], [104], [5], [104], [5], [104], [104],
    [median], [108], [4], [104], [5], [104], [5],
  ),
  caption: [Normal average],
) <norm_avg_table>


#figure(
  table(
    columns: 7,
    table.header([averages], [100], [200], [300], [400], [500], [total]),
    [mean], [104], [5], [104], [5], [104], [104],
    [median], [108], [4], [104], [5], [104], [5],
  ),
  caption: [Internationalised average],
) <i18n_avg_table>

// point here is that there is no one unique answer

== Get my data!

Download link to CSVbase

// Some other things you could do with this data
