use super::utils::refresh_user_token::refresh_user_token;
use crate::{
    commands::utils::{color::Color, get_user_input},
    rocal_api_client::{
        cancel_subscription::CancelSubscription, subscription_status::SubscriptionStatus,
        RocalAPIClient,
    },
};
use std::io::Write;

pub async fn unsubscribe() {
    refresh_user_token().await;

    if let Ok(status) = get_subscription_status().await {
        println!("Your plan is {}", status.get_plan());

        if *status.get_cancel_at_period_end() {
            println!(
                "Your subscription has been scheduled to cancel at the end of the current period."
            );
            println!("If you want to continue your subscription next period, please wait for the end so that you could subscribe again by `rocal subscribe` command.");
            return;
        }

        println!("Are you sure to unsubscribe? (yes/no)");

        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut proceed = String::new();

        std::io::stdin()
            .read_line(&mut proceed)
            .expect("Enter yes or no");

        let proceed = proceed.trim().to_lowercase();

        if proceed == "yes" {
            handle_unsubscribe().await;
        } else if proceed != "no" {
            println!("{}", Color::Red.text("Answer yes/no"));
        }
    } else {
        println!("You have not subscribed yet.");
    }
}

async fn handle_unsubscribe() {
    println!("Tell us a reason why you want to leave.");

    let reasons = CancelSubscription::get_reasons();

    for n in 1..(reasons.len() + 1) {
        let reason = reasons.get(&(n as u32)).unwrap();
        println!("{}. {}", n, reason);
    }

    let reason = get_user_input("a reason (1 to 8)");

    if let Ok(reason) = reason.parse::<u32>() {
        if 1 <= reason && reason <= 8 {
            let cancel_subscription = match CancelSubscription::new(reason) {
                Ok(sub) => sub,
                Err(err) => {
                    println!("{}", Color::Red.text(&err));
                    return;
                }
            };

            let client = RocalAPIClient::new();

            if let Err(err) = client.unsubscribe(cancel_subscription).await {
                println!("{}", Color::Red.text(&err));
            } else {
                println!(
                    "Done. Your subscription is scheduled to cancel at the end of the current period."
                )
            }
        } else {
            println!("{}", Color::Red.text("Answer 1 to 8"));
        }
    } else {
        println!("{}", Color::Red.text("Answer 1 to 8"));
    }
}

pub async fn get_subscription_status() -> Result<SubscriptionStatus, ()> {
    let client = RocalAPIClient::new();

    match client.get_subscription_status().await {
        Ok(status) => {
            if status.get_plan() == "N/A" {
                return Err(());
            }

            Ok(status)
        }
        Err(_) => Err(()),
    }
}
