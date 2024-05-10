// Import necessary items from the near_sdk crate
use models::Event;
use near_sdk::{env, near, near_bindgen, AccountId};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    owner: AccountId,       // Account ID of the contract owner
    events: Vec<Event>,     // Vector to store Event instances
}

// Implement the default trait for the Contract struct
impl Default for Contract {
    // Define the default method
    fn default() -> Self {
        // Create a new instance of Contract with default values
        Self {
            owner: env::current_account_id(),   // Set the owner as the current account ID
            events: Vec::new(),                 // Initialize events vector as empty
        }
    }
}

// Implement the Contract structure
#[near_bindgen]
impl Contract {
    // Initialize method for creating a new instance of the Contract
    #[init]
    pub fn new(owner: AccountId) -> Self {
        // Initialize events vector as empty
        let events: Vec<Event> = Vec::new();

        // Create and return a new instance of Contract with provided parameters
        Contract { owner, events }
    }

    // Method to add a new event to the contract
    pub fn add_event(&mut self, title: String, estimated_budget: u128, description: String) {
        // Generate a unique ID for the event
        let id = self.events.len() as i32;

        // Create a new Event instance and push it to the events vector
        self.events.push(Event::new(id, title, estimated_budget, description));

        // Log a message indicating that a new event has been added
        env::log_str("Added a new event!");
    }

    // Method to retrieve a list of all events
    pub fn list_events(&self) -> Vec<Event> {
        // Return a copy of the events vector
        self.events.to_vec()
    }

    // Method to get the count of events
    pub fn event_count(&mut self) -> usize {
        // Return the length of the events vector
        self.events.len()
    }

    // Method to add a vote for a specific event
    pub fn add_vote(&mut self, id: usize) {
        // Get a mutable reference to the event by its ID
        let event: &mut Event = self.events.get_mut(id).unwrap();

        // Get the account ID of the caller
        let voter = env::predecessor_account_id();

        // Increment the total votes for the event
        event.total_votes += 1;

        // Log a message indicating that the vote has been successfully submitted
        env::log_str("Vote submitted successfully for this event!");

        // Add the voter's account ID to the list of votes for the event
        event.votes.push(voter.to_string());
    }

    // Method to get the total number of votes for a specific event
    pub fn get_total_votes(&mut self, id: usize) -> u64 {
        // Get a mutable reference to the event by its ID
        let event: &mut Event = self.events.get_mut(id).unwrap();

        // Return the total number of votes for the event
        event.total_votes.try_into().unwrap()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    // Function to set up the testing context and unit test environment
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // Unit test to add a project
    #[test]
    fn add_project() {
        // Set the account ID for testing as Alice
        let alice: AccountId = "alice.testnet".parse().unwrap();

        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        // Create a new instance of the contract with Alice as the owner
        let mut contract = Contract::new(alice);

        // Add a new event to the contract
        contract.add_event(
            "New Contemporary Art Show".to_string(),
            200,
            "Amazing selection of international artists from all over the world".to_string(),
        );

        // Get the count of events after adding a new event
        let result = contract.event_count();

        // Assert that the count of events is equal to 1
        assert_eq!(result, 1);
    }

    // Unit test to add a voter
    #[test]
    fn add_voter() {
        // Set the account ID for testing as Alice
        let alice: AccountId = "alice.testnet".parse().unwrap();

        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        // Create a new instance of the contract with Alice as the owner
        let mut contract = Contract::new(alice);

        // Add a new event to the contract
        contract.add_event(
            "New Contemporary Art Show".to_string(),
            200,
            "Amazing selection of international artists from all over the world".to_string(),
        );

        // Add a vote for the first event
        contract.add_vote(0);

        // Get the total votes for the first event
        let result = contract.get_total_votes(0);

        // Assert that the total votes for the first event is equal to 1
        assert_eq!(result, 1);
    }
}
