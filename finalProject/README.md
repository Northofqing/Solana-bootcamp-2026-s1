# Pinocchio 套利程序

基于图片中的套利路径实现的Solana链上套利程序，使用Pinocchio框架开发。

## 套利路径

根据提供的交易截图，套利路径如下：

1. **75.63 USDC → 0.00096698 cbBTC** (Jupiter Aggregator v6, 手续费0.01%)
2. **0.00096698 cbBTC → 0.00096999 WBTC** (Manifest)
3. **0.00096999 WBTC → 75.631329 USDC** (Whirlpool)

**净利润**: 0.001329 USDC (约0.0018%)

## 项目结构

```
pinocchio_arbitrage/
├── Cargo.toml           # 项目配置，包含pinocchio依赖
├── src/
│   ├── main.rs          # 链上程序主逻辑
│   └── client.rs        # 客户端调用示例
└── README.md
```

## 核心功能

### 链上程序 (main.rs)

- **execute_arbitrage**: 执行完整的三跳套利
  - 账户验证和签名检查
  - 调用Jupiter v6进行USDC→cbBTC交换
  - 调用Manifest进行cbBTC→WBTC交换
  - 调用Whirlpool进行WBTC→USDC交换
  - 验证套利盈利性

### 指令格式

```
ExecuteArbitrage {
    amount: u64,  // 初始USDC金额 (6位小数)
}
```

## 所需账户

1. **[signer]** 用户账户
2. **[writable]** 用户USDC代币账户
3. **[writable]** 用户cbBTC代币账户
4. **[writable]** 用户WBTC代币账户
5. **[]** Jupiter Aggregator v6程序
6. **[]** Manifest程序
7. **[]** Whirlpool程序
8. **[writable]** Jupiter池子账户 (USDC-cbBTC)
9. **[writable]** Manifest池子账户 (cbBTC-WBTC)
10. **[writable]** Whirlpool池子账户 (WBTC-USDC)
11. **[]** Token程序
12. **[]** System程序

## 编译部署

### 前提条件

安装Solana工具链：
```bash
# 安装Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# 验证安装
solana --version
```

### 1. 编译程序

使用标准的Cargo编译（生成普通库）：
```bash
cargo build --release
```

**或者**使用Solana BPF工具链编译（生成可部署的.so文件）：
```bash
# 如果使用cargo-build-sbf（推荐）
cargo build-sbf

# 或使用旧版cargo-build-bpf
cargo build-bpf
```

### 2. 部署到Solana

```bash
# 设置网络（devnet用于测试）
solana config set --url https://api.devnet.solana.com

# 创建密钥对（如果还没有）
solana-keygen new

# 空投测试SOL（仅devnet/testnet）
solana airdrop 2

# 部署程序
solana program deploy target/deploy/pinocchio_arbitrage.so
```

### 3. 记录程序ID

部署成功后会返回程序ID，需要更新到client.rs中的`ARBITRAGE_PROGRAM_ID`

## 使用方法

### 准备工作

1. 确保钱包有足够的USDC、SOL (手续费)
2. 创建cbBTC和WBTC的代币账户
3. 找到对应的池子地址：
   - Jupiter v6: USDC-cbBTC池
   - Manifest: cbBTC-WBTC池
   - Whirlpool: WBTC-USDC池

### 调用示例

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

let client = RpcClient::new("https://api.mainnet-beta.solana.com");
let user = Keypair::from_bytes(&your_keypair_bytes).unwrap();

// 创建套利指令 (75.63 USDC)
let ix = create_arbitrage_instruction(
    &user.pubkey(),
    &usdc_account,
    &cbbtc_account,
    &wbtc_account,
    75_630_000,  // 75.63 USDC
);

// 发送交易
let tx = Transaction::new_signed_with_payer(
    &[ix],
    Some(&user.pubkey()),
    &[&user],
    recent_blockhash,
);

let sig = client.send_and_confirm_transaction(&tx)?;
println!("套利完成: {}", sig);
```

## 风险提示

1. **价格波动**: 交易执行期间价格可能变化，导致亏损
2. **滑点控制**: 建议设置合理的最小输出金额
3. **手续费**: 包括交易手续费、DEX手续费和优先费用
4. **MEV**: 可能被三明治攻击，建议使用Jito等MEV保护
5. **流动性**: 大额交易可能导致高滑点

## 优化建议

1. **实时监控**: 监控各DEX价格，发现套利机会自动执行
2. **动态路径**: 根据价格自动选择最优套利路径
3. **批量执行**: 一次交易执行多个套利机会
4. **闪电贷**: 使用闪电贷放大收益
5. **Gas优化**: 使用compute unit优化减少手续费

## 依赖的协议

- **Jupiter Aggregator v6**: `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4`
- **Manifest**: `MNFSTqtC93rEfYHB6hF82sKdZpUDFWkViLByLd1k1Ms`
- **Whirlpool**: `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc`

## License

MIT
