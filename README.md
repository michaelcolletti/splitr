# Why Splitr? 

- While studying data-center migration and enterprise architecture and cloud, I came upon some talented folks that were extraordinary at creating a workable solution where there seemingly wasn't a way around a problem. Sometimes this was in using tools that needed approval

- Back then, I thought Golang would be perfect for this, partially because I was so into learning more about Go and had already dug enough into the conventional well worn ways to solve these problems (split, other proprietary and open utilities etc) but they all lacked the staggering performance I knew would be achievable with Rust.

### Use Case 

- Use case to split and reassemble files quickly and reliably
- Enter **splitr**, A performant rust program that will split very large files of different types of formats into manageable parts. It should have the ability to save locally (default to $DATADUMP env var for filesystem dump)

### Command Line Options

- Split To split a file: target/debug/splitr <split|reassemble> <file_path> <split_size|part_count>
- Reassemble: target/debug/splitr reassemble> <file_path> <split_size|part_count>
- 
