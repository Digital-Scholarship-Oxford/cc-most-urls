---
title: Measuring URLs in the Common Crawl Index
author: Pierre Marshall
abstract: words words
---

## Introduction

## What is Common Crawl

A free, open repository of web crawl data that can be used by anyone

  - An archive?
  - An index?

References on this?

### How are sites indexed?

CDXJ index, link to the spec


## Motivation

Writing code where you want to know how long the url is in order to pre-allocate memory.

Useful for archives to know just how long a URL should be.

Doing this sort of thing helps you to find edge cases in the wild
https://catchjs.com/Blog/PerformanceInTheWild

### Practical limits



RFC 7231 Section 6.5.12.

The 414 (URI Too Long) status code indicates that the server is refusing to service the request because the request-target is longer than the server is willing to interpret.

## Standards


## Confounding issues

### Internationalisation

URLs can contain newlines
https://onlinelibrary.wiley.com/doi/10.1002/spe.3296

## Method

 Start with cc-index-paths, download and decompress each url in the file.

  We then have a 5GB chunk of the index.

  For each line in this chunk, truncate the first few bytes to get the json, then deserialise it using
  #strike[serde] nanoserde to a struct which looks like this:

  ```rust
  struct IndexRecordParsed {
      url: String,
      url_length: u16,
      i18_url_length: u16,
      status: u8,
  }```

  #pagebreak()

  // `url_length` is simply `index.url.len()`, and i18_url_length is a little more complicated.

  Then push that to a Vec, and deduplicate at the end with ```rust dedup_by(|a, b| a.url == b.url)```.

  // this whole thing is pretty straightforward, 165 lines of code.

  Iterate over the newly created Vec and map all these values to a comma-separated list.
  Write it out to file.
  Start again.

## Results

Tables!



## Discussion

Where this fits with other URI schemes. For example, DOIs.
