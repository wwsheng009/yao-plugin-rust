https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

https://www.swiftdiaries.com/rust/tonic/

```sh

rustup.exe update

```

创建项目

```sh
cargo new rust_grpc

cd rust_grpc
mkdir proto
# 把proto定义文件放在proto目录
```

生成模型定义对应的 rust 代码

https://github.com/hyperium/tonic/blob/master/tonic-build/README.md

更新 cargo.yaml 配置文件，增加`build-dependencies`

```yaml
[package]
name = "rust_grpc"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[build-dependencies]
tonic-build = "0.10"
```

```sh
cargo update
cargo build
```

在根目录创建文件`build.rs`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/model.proto")?;
    tonic_build::compile_protos("proto/grpc_controller.proto")?;
    Ok(())
}
```

默认会生成在这个目录下：`target/debug/build/{project-name}-{hash}/out`

`\rust_grpc\target\debug\build\rust_grpc-6a050400d6a6cfde\out`

```rust
fn main() {

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src")  // you can change the generated code's location
        .compile(
            &["proto/model.proto","proto/grpc_controller.proto"],
            &[""], // specify the root location to search proto dependencies
        ).unwrap();
}
```

比如以上的配置：

- 生成 server 定义
- 生成 client 定义
- 目标目录是 src

生成的文件名会是 proto 文件中的定义的 package 名称。比如定义的 package 是`plugin`，会生成`plugin.rs`

```sh
rm ..\myplugin.dll && cargo build && move .\target\debug\myplugin.exe ../myplugin.dll
```

```sh
yao run plugins.myplugin.hell
```
