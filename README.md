<img src="img/splitr1.jpg" width="600" />

[![Build, Test, Tag, Release and Deploy](https://github.com/michaelcolletti/splitr/actions/workflows/rust-build.yml/badge.svg)](https://github.com/michaelcolletti/splitr/actions/workflows/rust-build.yml)
[![Rust Docker Deploy](https://github.com/michaelcolletti/splitr/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/michaelcolletti/splitr/actions/workflows/docker-publish.yml)
# Splitr? 

- While studying data-center migration and enterprise architecture and cloud, I came upon some talented folks that were extraordinary at creating a workable solution where there seemingly wasn't a way around a problem. Sometimes this was in using tools that needed approval (e.g. Authority to Operate, ATO).

- Back then, I thought Golang might be perfect for this, partially because I was so into learning more about Golang, wanted to have a singular binary and had already dug enough into the conventional well worn ways to solve these problems (split, other proprietary and open utilities etc). These all lacked the staggering performance I knew would be achievable with Rust. 

- Also, in certain migration cases I saw that being resourceful and imaginative had limits. Sometimes, 'that dog won't hunt' and constraints win some days. However, if one can run in a container, or execute the rustc compiler or Cargo, this program, as simple as it is, could be really helpful. Often this wont be permitted as tools, scripts etc. need to be approved and could go through quite a lengthy manual process of validation and vetting by customers. It is this reason that non-compiled languages, besides pliability, are effectively used for highly regulated environments. Here, I'm talking about Python and PERL (no joke, it still works and is used more than I imagined). Deep respect the PERL OG [Meryln](https://en.wikipedia.org/wiki/Randal_L._Schwartz).

### Use Cases

- There are many use-cases with a need to split and reassemble files quickly and reliably. It should be performant and auditable. Maybe you need to move a huge system image or you're in log-hell needing to move a important log, or see the middle of a file that you can't even open.

### Enter **Splitr**

A performant Rust program that will split very large files of different formats into manageable parts. It should have the ability to save locally (default to $DATADUMP environment variable for filesystem dump). 

### Ways to Run Splitr  

- Running from the CLI

### Command Line Options Running Locally

- To split a file: target/debug/splitr <split|reassemble> <file_path> <split_size|part_count>
- Reassemble: target/debug/splitr reassemble> <file_path> <split_size|part_count>
- Detail to split: **cargo run split BinaryFileName 1MB 10** _Note the multiple files broken out_
- Detail to reassemble: **cargo run BinaryFileName reassemble 10  _The file is back in one piece_

### Examples 

- `<split|reassemble>`: The command to either split or reassemble the file.
- `<file_path>`: The path to the file to be split or reassembled.
- `<split_size|part_count>`: The size to split the file into (e.g., `10MB`, `1GB`, `1TB`) and the number of parts to reassemble.


### Running with Docker (this section also experimental)

- Running from a Podman/Docker container 
- (TBA) Web Frontend with Object Storage 

- A prereq is having the repo cloned or just download the multi-stage build of a rust env using the Dockerfile.

```sh
git clone https://github.com/michaelcolletti/splitr.git`
```

### To build and run the Docker container, 

- Follow these steps:

1. **Build the Docker image:**

    ```sh
    docker build -t splitr .
    ```
    
*- Note: be sure the dot is there. It specifies where the Dockerfile is. Alternatively you can use the -f /path/to/Dockerfile*

2. **Run the Docker container:**

    ```sh
    docker run --rm -v $(pwd):/data splitr <split|reassemble> <file_path> <split_size|part_count>
    ```

    - `<split|reassemble>`: The command to either split or reassemble the file.
    - `<file_path>`: The path to the file to be split or reassembled.
    - `<split_size|part_count>`: The size to split the file into (e.g., `10MB`, `1GB`, `1TB`) or the number of parts to reassemble.

For example, to split a file:

```sh
docker run --rm -v $(pwd):/data splitr split /data/largefile.txt 100MB
```

