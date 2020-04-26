# WikiGraph

A CLI program that may be used to map Wikipedia articles to a graph and find interesting relations.

If you're actually looking for this kind of functionality you may want to check out [WikiGraph by erabug](https://github.com/erabug/wikigraph) which is far better than my program. I did not look at her code or her program and did not get the idea from her. The main difference is that she makes use of a dataset while my version uses live data from `wikipedia.org`.

## Installation

There is no cargo crate for this program as it is experimental and in my opinion not worth putting on crates.io.

You can install it as follows:
```
git clone https://github.com/MiltFra/wikigraph.git
cd wikigraph
cargo build --release
```

The release flag is not required but highly recommended as it increases the performance significantly.

## Usage

At the moment the CLI only supports a single operation: To find all the possible paths between the URLs in a given input file.

An input file may look as follows:

```
/wiki/Tree
https://en.wikipedia.org/wiki/Astronomical_symbols
YouTube
```

All of these ways of describing a Wikipedia article are valid and may be interchanged as desired.

Assuming this text is stored in a file called `input-file` a command to find the paths between "Tree" and "Astronomical symbols", "Tree" and "YouTube" and "Astronomical symbols" and "YouTube" would look as follows:

```
$ target/release/wikigraph input-file
```

Note that for this to work you need to run the steps described in the installation section.

## Purpose and Experience

If you run the program you will notice that it works but is not really usable. Due to the insane connectedness of Wikipedia and the exponential scaling of the graph size, searching for distant relations between articles is bacially impossible using this. The HTTP requests are just too slow to keep up.

This program was written as an experiment and as a project to explore Rust and especially concurrency in the language. It makes heavy use of the async/await syntax and has taught me more about the usage of this concurrency.

This code is public to allow people, like me, that are new to the language and/or programming to read some source code that is not too complex or abstract and maybe get some inspirations. Furthermore I think it will be fun to look at in the future to assess my competence at the time of writing this and maybe (hopefully, lol) observe some change in the way I write code.

I feel like if you've read this far and you're still reading, you may need a hug. So here you go: *\*hug\**. Have a lovely day!

## License

GPL3. TL;DR: Do whatever you want with the code as long as it stays FOSS under the same license and the source is included. More information can be found in `LICENSE`.
