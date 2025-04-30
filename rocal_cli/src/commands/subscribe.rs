use crate::{
    commands::{
        unsubscribe::get_subscription_status,
        utils::{color::Color, list::List, open_link::open_link},
    },
    rocal_api_client::{create_payment_link::CreatePaymentLink, RocalAPIClient},
};

use super::utils::{get_user_input, refresh_user_token::refresh_user_token};

pub async fn subscribe() -> Result<(), std::io::Error> {
    refresh_user_token().await;

    if let Ok(status) = get_subscription_status().await {
        println!("Your plan is {}", status.get_plan());

        if !status.is_free_plan() {
            show_plans()?;
            return Ok(());
        }
    }

    println!(
        "Choose your plan from these options ({} or {})",
        &Color::Green.text("basic"),
        &Color::Blue.text("developer")
    );

    show_plans()?;

    let plan = get_user_input("a plan (basic or developer)");
    let plan = plan.to_lowercase();

    if !(plan == "basic" || plan == "developer") {
        println!(
            "{}",
            &Color::Red.text("Enter a plan you want to subscribe from basic or developer")
        );
        return Ok(());
    }

    create_payment_link(&plan).await;

    Ok(())
}

async fn create_payment_link(plan: &str) {
    let client = RocalAPIClient::new();
    let create_payment_link = CreatePaymentLink::new(&plan);

    match client.create_payment_link(create_payment_link).await {
        Ok(link) => {
            println!(
                "{}",
                Color::Green.text(
                    "Here is your payment link. Open the link with your browser to subscribe."
                )
            );

            println!("{}", Color::Green.text(&link));

            if let Err(err) = open_link(&link) {
                println!("{}", err.to_string());
            }
        }
        Err(err) => {
            println!("{}", Color::Red.text(&err));
        }
    }
}

fn show_plans() -> Result<(), std::io::Error> {
    let mut list = List::new();

    // Basic
    let mut plan = List::new();
    plan.add_text(&Color::Green.text("Basic"));

    let mut plan_cap = List::new();
    plan_cap.add_text("Deploy your application + compression, basic hosting, and versioning");

    let mut plan_price = List::new();
    plan_price.add_text("$10/month");

    plan.add_list(plan_cap);
    plan.add_list(plan_price);

    list.add_list(plan);

    // Developer
    let mut plan = List::new();
    plan.add_text(&Color::Blue.text("Developer"));

    let mut plan_cap = List::new();
    plan_cap.add_text("Including all Basic plan's capabilities plus CDN and Sync server support");

    let mut plan_price = List::new();
    plan_price.add_text("$20/month");

    plan.add_list(plan_cap);
    plan.add_list(plan_price);

    list.add_list(plan);

    // Pro
    let mut plan = List::new();
    plan.add_text(&Color::Red.text("Pro (coming soon, stay tuned...)"));

    let mut plan_cap = List::new();
    plan_cap.add_text("Including all Developer plan's capabilities plus custom domain, team collboration, and customer supports");

    let mut plan_price = List::new();
    plan_price.add_text("$40/month");

    plan.add_list(plan_cap);
    plan.add_list(plan_price);

    list.add_list(plan);

    println!("{}", list);

    Ok(())
}
