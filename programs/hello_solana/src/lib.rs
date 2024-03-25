// 导入Solana的模块和类型
use solana_program::{
    account_info::AccountInfo, 
    entrypoint, // 宏定义，用来指定智能合约的执行入口
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};

// 指定智能合约的执行函数是 process_instruction
entrypoint!(process_instruction);

// 智能合约逻辑
fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");

    msg!("Our program's Program ID: {}", &program_id);

    Ok(())
}