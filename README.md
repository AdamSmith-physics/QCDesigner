# QCDesigner
Multiplatform app for designing and exporting quantum circuit diagrams


## Resources 

- [Rust "The Book"](https://doc.rust-lang.org/book/)
- [Zed Repo](https://github.com/zed-industries/zed/tree/main/crates/gpui)
- [GPUI Book](https://matinaniss.github.io/gpui-book/introduction.html)
- [0xshadow blog](https://blog.0xshadow.dev/posts/learning-gpui/gpui-todo-app/)
- [GPUI-Component](https://github.com/longbridge/gpui-component/tree/main)
- [GPUI Examples](https://github.com/zed-industries/awesome-gpui)

## AI use

Artificial intelligence tools were used in this project. Specifically, I made use of Large Language Models (LLMs) and LLM powered coding agents. These were use for three main reasons:
1. I am new to the Rust programming language, and these tools are have aided in speeding up the learning process. 
2. I used the GPUI crate for the app GUI. This is in beta and embedded in the codebase for the zed code editor. As such, it is currently very sparsely documented. I include **[add above]** above the existing references that I have used. Otherwise, the use of LLM powered agents has been extremely powerful in learning how GPUI works in absence of detailed documentation!
3. For repetitive and tedius coding tasks, as well as debugging GPUI implementation issues. This also includes tidying code, and adding consistent commenting and formating. This has helped speed up development. 

I have made use of the following resources:
- Gemini Pro --- mainly for the learning how to use Rust and GPUI properly.
- Zed agent and OpenCode powered by `qwen3.6:27b-coding-mxfp8` via Ollama locally --- main coding agent for tedius coding tasks and debugging assistance.
- Zed agent powered by Claude Sonnet 5 via GitHub copilot --- for more challenging debugging of code.