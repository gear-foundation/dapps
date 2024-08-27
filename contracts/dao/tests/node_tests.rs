mod utils_gclient;

use gclient::GearApi;
use gstd::prelude::*;

// TODO: fix test
#[tokio::test]
#[ignore]
async fn gclient_dilution_bound() -> gclient::Result<()> {
    let mut api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let ft_program = utils_gclient::ft::init(&api).await?;
    let dao_program = utils_gclient::dao::init(
        &api,
        &ft_program,
        10000000,
        100000000,
        10000000,
        3,
        10000000,
    )
    .await?;

    utils_gclient::common::fund_applicants(&api).await?;

    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;

    for applicant in utils_gclient::common::APPLICANTS {
        api = api.with(applicant)?;

        let applicant_id = utils_gclient::common::get_current_actor_id(&api);

        utils_gclient::ft::mint(&api, &ft_program, 0, &applicant_id, token_tribute).await?;
        utils_gclient::ft::approve(&api, &ft_program, 1, &dao_program, token_tribute).await?;

        {
            api = api.with("//Alice")?;

            utils_gclient::dao::add_to_whitelist(&api, &dao_program, &applicant_id, false).await?;
            utils_gclient::dao::submit_membership_proposal(
                &api,
                &dao_program,
                &applicant_id,
                token_tribute,
                shares_requested,
                0,
                "",
                false,
            )
            .await?
            .expect("Unexpected empty proposal id.");
        }
    }

    Ok(())
}
