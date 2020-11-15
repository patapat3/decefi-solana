#![no_main]
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
struct DepositRequest {
    instruction: Deposit,
    balance: u64,
    correct_payer_account: bool,
}

fuzz_target!(|data: &DepositRequest| {
    // fuzzed code goes here
        let place_order_result = process_instruction(
        market_accounts.market.owner,
        &[
            market_accounts.market.clone(),
            orders_account.clone(),
            market_accounts.req_q.clone(),
            if data.correct_payer_account == (data.instruction.side == Side::Bid) {
                pc_account.clone()
            } else {
                coin_account.clone()
            },
            owner.clone(),
            market_accounts.coin_vault.clone(),
            market_accounts.pc_vault.clone(),
            market_accounts.spl_token_program.clone(),
        ],
        &MarketInstruction::NewOrder(data.instruction.clone()).pack(),
    );
    if !data.correct_payer_account {
        assert!(place_order_result.is_err());
    }
});
