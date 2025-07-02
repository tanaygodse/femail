use clap::Parser;

mod gmail;
mod filesystem;
mod auth;

use gmail::GmailClient;
use filesystem::GmailFilesystem;
use auth::{save_token, StoredToken};

#[derive(Parser)]
#[clap(name = "femail")]
#[clap(about = "A Gmail filesystem browser with real Gmail API integration")]
struct Args {
    /// Command to run
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
enum Command {
    /// Authenticate with Gmail (run this first)
    Auth,
    /// List Gmail labels (directories)
    Labels,
    /// List messages in a label
    Messages { label: String },
    /// Show content of a message
    Read { label: String, message_id: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("Femail - Gmail Filesystem Browser");
    println!("Now with real Gmail API integration!");
    println!();
    
    match args.command {
        Some(Command::Auth) => {
            println!("Setting up Gmail authentication...");
            println!();
            println!("To use the Gmail API, you need an access token.");
            println!();
            println!("📋 Steps to get your access token:");
            println!("1. Go to https://developers.google.com/oauthplayground/");
            println!("2. In the left panel, find 'Gmail API v1'");
            println!("3. Select 'https://www.googleapis.com/auth/gmail.readonly'");
            println!("4. Click 'Authorize APIs' and sign in with your Google account");
            println!("5. Click 'Exchange authorization code for tokens'");
            println!("6. Copy the 'Access token' value");
            println!();
            println!("Enter your access token:");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let access_token = input.trim().to_string();
            
            if access_token.is_empty() {
                println!("❌ No token provided.");
                return Ok(());
            }
            
            let token = StoredToken { access_token };
            save_token(&token)?;
            
            println!("✅ Access token saved successfully!");
            println!("You can now use other commands like 'femail labels'");
            println!();
            println!("Note: This token will expire after 1 hour. You'll need to");
            println!("repeat the auth process when it expires.");
        }
        Some(Command::Labels) => {
            println!("Fetching Gmail labels...");
            let gmail_client = GmailClient::new()?;
            let filesystem = GmailFilesystem::new(gmail_client);
            
            match filesystem.list_labels() {
                Ok(labels) => {
                    println!("Available labels (directories):");
                    for label in labels {
                        println!("  📁 {} ({})", label.name, label.id);
                    }
                }
                Err(e) => {
                    println!("❌ Error fetching labels: {}", e);
                    println!("Try running 'femail auth' first if you haven't authenticated.");
                }
            }
        }
        Some(Command::Messages { label }) => {
            println!("Fetching messages from '{}'...", label);
            let gmail_client = GmailClient::new()?;
            let filesystem = GmailFilesystem::new(gmail_client);
            
            match filesystem.list_messages(&label) {
                Ok(messages) => {
                    if messages.is_empty() {
                        println!("No messages found in '{}'", label);
                    } else {
                        println!("Messages in '{}':", label);
                        for message in messages {
                            println!("  📧 {} (from: {}) - {}", 
                                message.subject, message.from, 
                                if message.snippet.len() > 50 {
                                    format!("{}...", &message.snippet[..50])
                                } else {
                                    message.snippet
                                });
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Error fetching messages: {}", e);
                    println!("Make sure the label '{}' exists and you're authenticated.", label);
                }
            }
        }
        Some(Command::Read { label, message_id }) => {
            println!("Fetching message '{}' from '{}'...", message_id, label);
            let gmail_client = GmailClient::new()?;
            let filesystem = GmailFilesystem::new(gmail_client);
            
            match filesystem.read_message(&message_id) {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    println!("❌ Error reading message: {}", e);
                    println!("Make sure the message ID '{}' is correct and accessible.", message_id);
                }
            }
        }
        None => {
            println!("Gmail Filesystem Browser - Real API Integration");
            println!();
            println!("Available commands:");
            println!("  femail auth              - Authenticate with Gmail (run this first)");
            println!("  femail labels            - List Gmail labels");
            println!("  femail messages <label>  - List messages in a label");
            println!("  femail read <label> <id> - Read a specific message");
            println!();
            println!("Example usage:");
            println!("  femail auth");
            println!("  femail labels");
            println!("  femail messages INBOX");
            println!("  femail read INBOX 123abc...");
        }
    }
    
    Ok(())
}