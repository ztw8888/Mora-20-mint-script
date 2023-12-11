
# 批量打 Mora-20 铭文 Rust 脚本

[索引标准](https://mora.app/planet/qvsfp-6aaaa-aaaan-qdbua-cai/0C6YXTY4BP1XXQ9G8X7VJ9MTCA)

# 如何使用

1. 在 [Plug](https://plugwallet.ooo/) 创建一个账号并导出 `.PEM` 文件，放到代码根目录下，并命名为 `identity.pem`

2. 在 Mora 的星球管理页面给刚才导出的账号添加星球写作者权限

3. 在 `main.rs` 文件自定义铭文信息
```rust
    const MORA_CANISTER_ID: &str = "erpbi-cyaaa-aaaan-qdccq-cai"; // 修改为你自己的 Mora 星球 Canister Id
    const MINT_TITLE: &str = "NNSDAO"; // 修改为你要打铭文要求的文章 Title
    // 修改为你要打铭文要求的 Mint 命令
    const MINT_CONTENT: &str = "{\"p\": \"mora-20\", \"op\": \"mint\", \"tick\": \"nnsdao\", \"amt\": \"1000\"}";
    const MINT_AMOUNT: usize = 10; // 修改为你需要批量打的张数
```

4. 运行脚本 
```rust 
    cargo run --release
````

运行效果 :

![](./run_result.png)

