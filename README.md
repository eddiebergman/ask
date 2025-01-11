# Ask
Just `ask` a question and get a straight forward answer.

```bash
$ ask How do I look at the assembly generated for a rust compiled program
rustc --emit=asm your_program.rs

$ ask How do I push the repo Im in up to a fresh repo on github using gh
gh repo create <repo-name> --public --source=. && git push -u origin main
```

> [!IMPORTANT]
> You must have an environment variable exposed with your `OPENAI_API_KEY` to use this program.

> [!TIP]
> If you need to use special characters in your question such as `?`, `&`, etc., you can wrap
> your question in quotes.

#### Why?
Way too many fully-customizable, full terminal chat tools out there.
All of them are cool but I just wanted to get a quick copy-paste answer,
not a full dicussion and explanation.


#### Development
Do not ask for feature requests or extra customizability, such
as different models, endpoints, prompts, alternative usage, etc.
Please just fork and/or copy-and-edit the code as you like.

A future development on this will be for personal preference and usage.

## Installation

### Grab the release binary
The only available binary is the one I could easily build on my system,
`x86_64-unknown-linux-gnu`. If you need a binary for a different platform,
please build it from source.

Release page: https://github.com/eddiebergman/ask/releases

Once downloaded, you'll likely have to allow execution of the binary:
```bash
chmod +x /path/to/download/ask
```

If you want to use the binary from anywhere, as shown in the examples above,
You'll need to add it to your path.

```bash
ask "how do I add a binary at ~/Downloads/ask so I can run it everywhere?"
mv ~/Downloads/ask /usr/local/bin/ && chmod +x /usr/local/bin/ask
```

### Build from source
```bash
git clone git@github.com:eddiebergman/ask.git
cd ask
cargo build --release
./target/release/ask "How do I make a binary file on my system usable from everywhere?"
```

The build has been optimized mainly for a smaller binary size, as the
speed is irrelavant given the web-request.

Builds on my machine at `1.5MB`, could probably be made smaller.
If you know how, please share!
