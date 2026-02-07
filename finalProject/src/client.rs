// 客户端调用示例 (需要在实际项目中使用 solana-sdk)
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

// 程序ID (部署后获得)
const ARBITRAGE_PROGRAM_ID: &str = "YourProgramIdHere";

// Jupiter Aggregator v6 程序ID
const JUPITER_V6_PROGRAM_ID: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

// Manifest 程序ID
const MANIFEST_PROGRAM_ID: &str = "MNFSTqtC93rEfYHB6hF82sKdZpUDFWkViLByLd1k1Ms";

// Whirlpool 程序ID
const WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

// Token Program
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

// System Program
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

pub fn create_arbitrage_instruction(
    user: &Pubkey,
    user_usdc_account: &Pubkey,
    user_cbbtc_account: &Pubkey,
    user_wbtc_account: &Pubkey,
    amount: u64, // USDC金额 (6 decimals, 所以75.63 = 75_630_000)
) -> Instruction {
    let program_id = ARBITRAGE_PROGRAM_ID.parse::<Pubkey>().unwrap();
    
    // 池子账户 (需要根据实际情况填写)
    let jupiter_pool = "YourJupiterPoolAddress".parse::<Pubkey>().unwrap();
    let manifest_pool = "YourManifestPoolAddress".parse::<Pubkey>().unwrap();
    let whirlpool_pool = "YourWhirlpoolPoolAddress".parse::<Pubkey>().unwrap();
    
    let accounts = vec![
        AccountMeta::new(*user, true),                      // 0. 用户 (signer)
        AccountMeta::new(*user_usdc_account, false),        // 1. 用户USDC账户
        AccountMeta::new(*user_cbbtc_account, false),       // 2. 用户cbBTC账户
        AccountMeta::new(*user_wbtc_account, false),        // 3. 用户WBTC账户
        AccountMeta::new_readonly(JUPITER_V6_PROGRAM_ID.parse().unwrap(), false), // 4. Jupiter程序
        AccountMeta::new_readonly(MANIFEST_PROGRAM_ID.parse().unwrap(), false),   // 5. Manifest程序
        AccountMeta::new_readonly(WHIRLPOOL_PROGRAM_ID.parse().unwrap(), false),  // 6. Whirlpool程序
        AccountMeta::new(jupiter_pool, false),              // 7. Jupiter池子
        AccountMeta::new(manifest_pool, false),             // 8. Manifest池子
        AccountMeta::new(whirlpool_pool, false),            // 9. Whirlpool池子
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID.parse().unwrap(), false),      // 10. Token程序
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID.parse().unwrap(), false),     // 11. System程序
    ];
    
    // 构建指令数据: [instruction_tag (1 byte), amount (8 bytes)]
    let mut data = vec![0u8]; // 0 = ExecuteArbitrage
    data.extend_from_slice(&amount.to_le_bytes());
    
    Instruction {
        program_id,
        accounts,
        data,
    }
}

// 使用示例
pub fn example_usage() {
    // 初始化用户密钥对
    let user = Keypair::new();
    
    // 用户的代币账户 (需要预先创建)
    let user_usdc = "YourUSDCAccountAddress".parse::<Pubkey>().unwrap();
    let user_cbbtc = "YourCbBTCAccountAddress".parse::<Pubkey>().unwrap();
    let user_wbtc = "YourWBTCAccountAddress".parse::<Pubkey>().unwrap();
    
    // 套利金额: 75.63 USDC (USDC有6位小数)
    let amount = 75_630_000u64;
    
    // 创建套利指令
    let arbitrage_ix = create_arbitrage_instruction(
        &user.pubkey(),
        &user_usdc,
        &user_cbbtc,
        &user_wbtc,
        amount,
    );
    
    // 创建交易
    // let recent_blockhash = client.get_latest_blockhash().unwrap();
    // let transaction = Transaction::new_signed_with_payer(
    //     &[arbitrage_ix],
    //     Some(&user.pubkey()),
    //     &[&user],
    //     recent_blockhash,
    // );
    
    // 发送交易
    // let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    // println!("套利交易成功: {}", signature);
}
