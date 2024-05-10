// Import necessary items from the near_sdk crate
use near_sdk::{env, near, AccountId, Timestamp};

// Add serialization formats for Borsh and JSON, and derive Clone trait for the struct
#[near(serializers=[borsh, json])]
#[derive(Clone)]
// Define the struct Event
pub struct Event {
    // Define fields of the Event struct
    id: i32,                                // Unique identifier for the event
    pub creator: AccountId,                 // Account ID of the creator of the event
    created_at: Timestamp,                  // Timestamp when the event was created
    title: String,                          // Title of the event
    estimated_budget: u128,                 // Estimated budget for the event
    pub total_votes: i64,                   // Total number of votes received for the event
    description: String,                    // Description of the event
    pub votes: Vec<String>,                 // List of account IDs that voted for the event
}

// Implement methods for the Event struct
impl Event {
    // Define a constructor method to create a new Event instance
    pub fn new(id: i32, title: String, estimated_budget: u128, description: String) -> Self {
        // Create a new Event instance with the provided parameters
        Event {
            // Assign values to the fields of the Event struct
            id,                                             // Assign provided id
            creator: env::signer_account_id(),             // Set creator as the signer's account ID
            created_at: env::block_timestamp(),           // Set creation timestamp to current block timestamp
            title,                                         // Assign provided title
            estimated_budget,                              // Assign provided estimated budget
            total_votes: 0,                                // Initialize total_votes to 0
            description,                                   // Assign provided description
            votes: vec![],                                 // Initialize votes vector as empty
        }
    }
}
