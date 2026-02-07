# 部署说明

## ✅ 编译成功

已成功编译套利程序：
- **文件**: `target/deploy/pinocchio_arbitrage.so`
- **大小**: 343KB
- **框架**: Pinocchio 0.5.0

## 🚀 部署步骤

### 1. 配置Solana网络

```bash
# 开发网（测试用）
solana config set --url https://api.devnet.solana.com

# 主网（生产用）
# solana config set --url https://api.mainnet-beta.solana.com
```

### 2. 检查钱包余额

```bash
# 查看当前钱包
solana address

# 查看余额
solana balance

# 如果是devnet，可以空投SOL
solana airdrop 2
```

### 3. 部署程序

```bash
# 部署到当前配置的网络
solana program deploy target/deploy/pinocchio_arbitrage.so

# 部署成功后会返回程序ID，例如：
# Program Id: 7xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### 4. 记录程序ID

部署成功后，将程序ID更新到 `src/client.rs` 中：

```rust
const ARBITRAGE_PROGRAM_ID: &str = "你的程序ID";
```

## ⚙️ 准备代币账户

在调用程序前，需要为钱包创建所需的代币账户：

```bash
# 创建USDC代币账户
spl-token create-account EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v

# 创建cbBTC代币账户（替换为实际mint地址）
spl-token create-account <cbBTC_MINT_ADDRESS>

# 创建WBTC代币账户（替换为实际mint地址）
spl-token create-account <WBTC_MINT_ADDRESS>
```

## 🔍 验证部署

```bash
# 查看程序信息
solana program show <PROGRAM_ID>

# 查看程序账户
solana account <PROGRAM_ID>
```

## 📝 下一步

1. **获取池子地址**
   - Jupiter v6: USDC-cbBTC池
   - Manifest: cbBTC-WBTC池  
   - Whirlpool: WBTC-USDC池

2. **更新client.rs**
   - 填入程序ID
   - 填入池子地址
   - 填入代币账户地址

3. **测试交易**
   - 使用小额测试
   - 验证套利逻辑
   - 监控日志输出

## ⚠️ 注意事项

- **devnet测试**: 先在devnet充分测试
- **资金安全**: 主网部署前确保代码安全
- **权限设置**: 程序部署后默认upgrade authority是部署者
- **Gas费用**: 部署需要~2-5 SOL的gas费（根据程序大小）

## 🛠️ 常见问题

### 部署失败：余额不足
```bash
# 检查余额
solana balance

# devnet空投
solana airdrop 5
```

### 部署失败：程序太大
程序已经优化到343KB，应该可以正常部署。如果还是太大，可以：
```bash
# 进一步优化
cargo build --release --features="no-entrypoint"
```

### 升级已部署的程序
```bash
# 升级程序（需要是upgrade authority）
solana program deploy --program-id <PROGRAM_ID> target/deploy/pinocchio_arbitrage.so
```

## 📊 预期结果

部署成功后，你应该看到类似输出：

```
Program Id: 7xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Deployment successful. Signature: 5yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
```

保存这个Program Id，后续调用时需要使用。
