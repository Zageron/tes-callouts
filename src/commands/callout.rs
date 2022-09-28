use crate::command::Command;
use crate::error::InteractionError;
use crate::interaction::{
    ApplicationCommandInteractionDataOption, ApplicationCommandOption,
    ApplicationCommandOptionChoice, ApplicationCommandOptionType,
    InteractionApplicationCommandCallbackData,
};
use std::fmt::{self, Debug};

use async_trait::async_trait;

pub(crate) struct Callout {}

#[derive(Debug, Clone, Copy)]
enum Arguments {
    Map,
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl PartialEq<&String> for Arguments {
    fn eq(&self, other: &&std::string::String) -> bool {
        self.to_string().to_lowercase() == **other
    }

    fn ne(&self, other: &&String) -> bool {
        !self.eq(other)
    }
}

#[async_trait(?Send)]
impl Command for Callout {
    async fn respond(
        &self,
        options: &Option<Vec<ApplicationCommandInteractionDataOption>>,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(options) = options {
            if let Some(option) = options.first() {
                if Arguments::Map == &option.name {
                    return Callout::handle_map(&option.value, ctx).await;
                }
            }
        }

        Callout::handle_error(&format!("No options supplied.")).await
    }

    fn name(&self) -> String {
        "callout".into()
    }

    fn description(&self) -> String {
        "Grab out custom callouts".into()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![ApplicationCommandOption {
            name: "map".into(),
            autocomplete: Some(true),
            description: "The name of the map to get callouts image for.".into(),
            required: Some(true),
            ty: ApplicationCommandOptionType::String,
            choices: None,
        }])
    }

    async fn autocomplete(
        &self,
        _options: &Option<Vec<ApplicationCommandInteractionDataOption>>,
        _ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: None,
            embeds: None,
            choices: Some(vec![
                ApplicationCommandOptionChoice {
                    name: "Wahoo World".into(),
                    value: "wahoo".into(),
                },
                ApplicationCommandOptionChoice {
                    name: "Undertow Spillway".into(),
                    value: "undertow".into(),
                },
            ]),
        })
    }
}

impl Callout {
    async fn handle_error(
        error: &String,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(error.to_string()),
            choices: None,
            embeds: None,
        })
    }

    async fn handle_map(
        map_name: &Option<String>,
        ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Ok(kv) = ctx.kv("CALLOUTS") {
            match map_name {
                Some(name) => {
                    if let Ok(Some(value)) = kv.get(name.as_str()).text().await {
                        Ok(InteractionApplicationCommandCallbackData {
                            content: Some(value),
                            choices: None,
                            embeds: None,
                        })
                    } else {
                        Self::handle_error(&format!("No image found for {:?}", map_name)).await
                    }
                }
                _ => Self::handle_error(&format!("No map option sent with request.")).await,
            }
        } else {
            Self::handle_error(&format!(
                "Could not find CALLOUTS KV namespace, make sure it is bound in wrangler."
            ))
            .await
        }
    }
}
