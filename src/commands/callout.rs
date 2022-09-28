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
        self.to_string() == **other
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
        _ctx: &mut worker::RouteContext<()>,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(options) = options {
            if let Some(option) = options.get(0) {
                if Arguments::Map == &option.name {
                    return Callout::handle_map(&option.value);
                }
            }
        }

        Callout::handle_default()
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
            choices: Some(vec![ApplicationCommandOptionChoice {
                name: "option 1".into(),
                value: "test".into(),
            }]),
        })
    }
}

impl Callout {
    fn handle_default() -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        Ok(InteractionApplicationCommandCallbackData {
            content: Some("Supply a map name to get callouts.".to_string()),
            choices: None,
            embeds: None,
        })
    }

    fn handle_map(
        map_name: &Option<String>,
    ) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        match map_name {
            Some(name) => Ok(InteractionApplicationCommandCallbackData {
                content: Some(format!("Callouts for {} map", name).to_string()),
                choices: None,
                embeds: None,
            }),
            _ => Self::handle_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let callout = Callout::handle_map(&Some("map".to_string()));
        assert_eq!(
            callout.unwrap().content.unwrap(),
            "Callouts for map map".to_string()
        );
    }
}
