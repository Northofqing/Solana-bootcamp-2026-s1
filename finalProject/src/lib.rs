use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// 使用pinocchio的ProgramResult
type ProgramResult = Result<(), ProgramError>;

entrypoint!(process_instruction);

// 程序入口点
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 解析指令类型
    let instruction = ArbitrageInstruction::try_from_slice(instruction_data)?;
    
    match instruction {
        ArbitrageInstruction::ExecuteArbitrage { amount } => {
            execute_arbitrage(accounts, amount)
        }
    }
}

// 套利指令
#[derive(Debug)]
pub enum ArbitrageInstruction {
    ExecuteArbitrage { amount: u64 },
}

impl ArbitrageInstruction {
    pub fn try_from_slice(data: &[u8]) -> Result<Self, ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        
        let (tag, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        match tag {
            0 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(ArbitrageInstruction::ExecuteArbitrage { amount })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

// 执行套利交易
// 账户顺序:
// 0. [signer] 用户账户
// 1. [writable] 用户USDC账户
// 2. [writable] 用户cbBTC账户
// 3. [writable] 用户WBTC账户
// 4. [] Jupiter Aggregator v6 程序
// 5. [] Manifest 程序
// 6. [] Whirlpool 程序
// 7. [writable] Jupiter v6池子账户 (USDC-cbBTC)
// 8. [writable] Manifest池子账户 (cbBTC-WBTC)
// 9. [writable] Whirlpool池子账户 (WBTC-USDC)
// 10. [] Token Program
// 11. [] System Program
fn execute_arbitrage(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // 账户验证
    if accounts.len() < 12 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    
    let user = &accounts[0];
    let user_usdc = &accounts[1];
    let user_cbbtc = &accounts[2];
    let user_wbtc = &accounts[3];
    let jupiter_program = &accounts[4];
    let manifest_program = &accounts[5];
    let whirlpool_program = &accounts[6];
    let jupiter_pool = &accounts[7];
    let manifest_pool = &accounts[8];
    let whirlpool_pool = &accounts[9];
    let token_program = &accounts[10];
    
    // 验证用户签名
    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // 记录初始USDC余额
    let initial_usdc_balance = get_token_balance(user_usdc)?;
    
    // 步骤1: USDC -> cbBTC (Jupiter Aggregator v6)
    // 预计: 75.63 USDC -> 0.00096698 cbBTC
    swap_jupiter_v6(
        jupiter_program,
        user_usdc,
        user_cbbtc,
        jupiter_pool,
        token_program,
        amount, // 75.63 USDC (6 decimals)
        0, // 最小输出金额，实际应设置滑点保护
    )?;
    
    let cbbtc_amount = get_token_balance(user_cbbtc)?;
    
    // 步骤2: cbBTC -> WBTC (Manifest)
    // 预计: 0.00096698 cbBTC -> 0.00096999 WBTC
    swap_manifest(
        manifest_program,
        user_cbbtc,
        user_wbtc,
        manifest_pool,
        token_program,
        cbbtc_amount,
        0,
    )?;
    
    let wbtc_amount = get_token_balance(user_wbtc)?;
    
    // 步骤3: WBTC -> USDC (Whirlpool)
    // 预计: 0.00096999 WBTC -> 75.631329 USDC
    swap_whirlpool(
        whirlpool_program,
        user_wbtc,
        user_usdc,
        whirlpool_pool,
        token_program,
        wbtc_amount,
        amount, // 至少要回收初始投入
    )?;
    
    // 验证套利成功
    let final_usdc_balance = get_token_balance(user_usdc)?;
    if final_usdc_balance <= initial_usdc_balance {
        return Err(ProgramError::Custom(1)); // 套利失败
    }
    
    let profit = final_usdc_balance - initial_usdc_balance;
    
    // 记录日志
    pinocchio::log::sol_log_64(0, 0, 0, 0, profit);
    
    Ok(())
}

// 获取代币余额
fn get_token_balance(token_account: &AccountInfo) -> Result<u64, ProgramError> {
    // 代币账户数据格式: 前32字节是mint，后8字节是amount
    let data = token_account.try_borrow_data()?;
    if data.len() < 72 {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // 偏移64字节获取amount (u64)
    Ok(u64::from_le_bytes(data[64..72].try_into().unwrap()))
}

// Jupiter Aggregator v6 交换
fn swap_jupiter_v6(
    jupiter_program: &AccountInfo,
    from_token: &AccountInfo,
    to_token: &AccountInfo,
    pool: &AccountInfo,
    token_program: &AccountInfo,
    amount_in: u64,
    min_amount_out: u64,
) -> ProgramResult {
    // 构建Jupiter v6的swap指令数据
    // 这是简化版本，实际需要根据Jupiter v6 IDL构建完整指令
    let mut instruction_data = Vec::with_capacity(17);
    instruction_data.push(1); // swap discriminator
    instruction_data.extend_from_slice(&amount_in.to_le_bytes());
    instruction_data.extend_from_slice(&min_amount_out.to_le_bytes());
    
    // 构建指令
    let instruction = pinocchio::instruction::Instruction {
        program_id: jupiter_program.key(),
        accounts: &[
            pinocchio::instruction::AccountMeta::writable(from_token.key()),
            pinocchio::instruction::AccountMeta::writable(to_token.key()),
            pinocchio::instruction::AccountMeta::writable(pool.key()),
            pinocchio::instruction::AccountMeta::readonly(token_program.key()),
        ],
        data: &instruction_data,
    };
    
    // 调用Jupiter程序
    pinocchio::program::invoke::<4>(
        &instruction,
        &[from_token, to_token, pool, token_program],
    )?;
    
    Ok(())
}

// Manifest 交换
fn swap_manifest(
    manifest_program: &AccountInfo,
    from_token: &AccountInfo,
    to_token: &AccountInfo,
    pool: &AccountInfo,
    token_program: &AccountInfo,
    amount_in: u64,
    min_amount_out: u64,
) -> ProgramResult {
    // 构建Manifest的swap指令数据
    let mut instruction_data = Vec::with_capacity(17);
    instruction_data.push(2); // swap discriminator
    instruction_data.extend_from_slice(&amount_in.to_le_bytes());
    instruction_data.extend_from_slice(&min_amount_out.to_le_bytes());
    
    // 构建指令
    let instruction = pinocchio::instruction::Instruction {
        program_id: manifest_program.key(),
        accounts: &[
            pinocchio::instruction::AccountMeta::writable(from_token.key()),
            pinocchio::instruction::AccountMeta::writable(to_token.key()),
            pinocchio::instruction::AccountMeta::writable(pool.key()),
            pinocchio::instruction::AccountMeta::readonly(token_program.key()),
        ],
        data: &instruction_data,
    };
    
    pinocchio::program::invoke::<4>(
        &instruction,
        &[from_token, to_token, pool, token_program],
    )?;
    
    Ok(())
}

// Whirlpool 交换
fn swap_whirlpool(
    whirlpool_program: &AccountInfo,
    from_token: &AccountInfo,
    to_token: &AccountInfo,
    pool: &AccountInfo,
    token_program: &AccountInfo,
    amount_in: u64,
    min_amount_out: u64,
) -> ProgramResult {
    // 构建Whirlpool的swap指令数据
    let mut instruction_data = Vec::with_capacity(17);
    instruction_data.push(3); // swap discriminator
    instruction_data.extend_from_slice(&amount_in.to_le_bytes());
    instruction_data.extend_from_slice(&min_amount_out.to_le_bytes());
    
    // 构建指令
    let instruction = pinocchio::instruction::Instruction {
        program_id: whirlpool_program.key(),
        accounts: &[
            pinocchio::instruction::AccountMeta::writable(from_token.key()),
            pinocchio::instruction::AccountMeta::writable(to_token.key()),
            pinocchio::instruction::AccountMeta::writable(pool.key()),
            pinocchio::instruction::AccountMeta::readonly(token_program.key()),
        ],
        data: &instruction_data,
    };
    
    pinocchio::program::invoke::<4>(
        &instruction,
        &[from_token, to_token, pool, token_program],
    )?;
    
    Ok(())
}
