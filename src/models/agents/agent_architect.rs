use crate::ai_functions::aifunc_architect::{ print_project_scope, print_site_urls };
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ ai_task_request_decoded, check_status_code };
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{ FactSheet, ProjectScope, SpecialFunctions };

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionsArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionsArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
           objective: "Gathers information and design solutions for website development".to_string(),
           position: "Solutions Architect".to_string(),
           state: AgentState::Discovery,
           memory: vec![]
        };

        Self { attributes }
    }

    // Retrieve project scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", factsheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
           msg_context,
           &self.attributes.position,
           get_function_string!(print_project_scope), 
           print_project_scope,
        ).await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        return ai_response;
    }

    // Retrieve site urls
    async fn call_determine_external_urls(&mut self, factsheet: &mut FactSheet, msg_context: String) {

        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
           msg_context,
           &self.attributes.position,
           get_function_string!(print_site_urls), 
           print_site_urls,
        ).await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionsArchitect {

    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {
        // warning be careful to infinite loops
       while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;    
                    // confirm if exteral urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(factsheet, factsheet.project_description.clone()).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }

                AgentState::UnitTesting => {
                    
                }

                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            } 
       } 

       Ok(())
    }
}
