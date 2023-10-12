use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionsArchitect;
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Prject Manager".to_string();

        let attributes = BasicAgent {
           objective: "Manage agents who are building an excellent website for the user".to_string(),
           position: position.clone(),
           state: AgentState::Discovery,
           memory: vec![]
        };

        
        let project_description: String = ai_task_request(
           usr_req,
           &position,
           get_function_string!(convert_user_input_to_goal), 
           convert_user_input_to_goal,
        ).await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes,
            factsheet,
            agents
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);  
    }

    fn create_agents(&mut self){
        self.add_agent(Box::new(AgentSolutionsArchitect::new()));
        // ! TODO: add more agents
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> = 
                agent.execute(&mut self.factsheet).await; 

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }
}


