// Tests for authentication module

use home_registry::auth::{create_token, validate_password, validate_username, verify_token};

#[test]
fn test_password_hashing() {
    let password = "test_password_123";
    let hash = home_registry::auth::hash_password_sync(password).expect("Failed to hash password");

    // Hash should not be empty
    assert!(!hash.is_empty());

    // Hash should not equal the password
    assert_ne!(hash, password);

    // Verify correct password
    assert!(home_registry::auth::verify_password_sync(password, &hash).unwrap());

    // Verify incorrect password
    assert!(!home_registry::auth::verify_password_sync("wrong_password", &hash).unwrap());
}

#[test]
fn test_password_validation() {
    // Valid passwords
    assert!(validate_password("password123").is_ok());
    assert!(validate_password("longer_password").is_ok());
    assert!(validate_password("P@ssw0rd!123").is_ok());

    // Invalid: too short
    assert!(validate_password("short").is_err());
    assert!(validate_password("1234567").is_err());

    // Invalid: empty
    assert!(validate_password("").is_err());

    // Invalid: too long
    assert!(validate_password(&"a".repeat(129)).is_err());
}

#[test]
fn test_username_validation() {
    // Valid usernames
    assert!(validate_username("user123").is_ok());
    assert!(validate_username("valid_user").is_ok());
    assert!(validate_username("user-name").is_ok());
    assert!(validate_username("User_Name-123").is_ok());

    // Invalid: too short
    assert!(validate_username("ab").is_err());
    assert!(validate_username("x").is_err());

    // Invalid: empty
    assert!(validate_username("").is_err());

    // Invalid: too long
    assert!(validate_username(&"a".repeat(51)).is_err());

    // Invalid: contains spaces
    assert!(validate_username("user name").is_err());

    // Invalid: contains invalid characters
    assert!(validate_username("user@name").is_err());
    assert!(validate_username("user.name").is_err());
    assert!(validate_username("user#name").is_err());
    assert!(validate_username("user$name").is_err());
}

#[test]
fn test_jwt_token_creation() {
    use uuid::Uuid;

    // Initialize JWT secret
    let _ = home_registry::auth::get_or_init_jwt_secret();

    let user_id = Uuid::new_v4();
    let username = "test_user";

    let token = create_token(&user_id, username).expect("Failed to create token");

    // Token should not be empty
    assert!(!token.is_empty());

    // Token should have 3 parts (header.payload.signature)
    assert_eq!(token.split('.').count(), 3);
}

#[test]
fn test_jwt_token_verification() {
    use uuid::Uuid;

    // Initialize JWT secret
    let _ = home_registry::auth::get_or_init_jwt_secret();

    let user_id = Uuid::new_v4();
    let username = "test_user";

    let token = create_token(&user_id, username).expect("Failed to create token");

    // Verify valid token
    let claims = verify_token(&token).expect("Failed to verify token");
    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.username, username);

    // Verify invalid token
    let invalid_token = "invalid.token.here";
    assert!(verify_token(invalid_token).is_err());

    // Verify tampered token (change last character)
    let mut tampered = token.clone();
    tampered.pop();
    tampered.push('X');
    assert!(verify_token(&tampered).is_err());
}

#[test]
fn test_jwt_secret_initialization() {
    let secret = home_registry::auth::get_or_init_jwt_secret();

    // Secret should not be empty
    assert!(!secret.is_empty());

    // Secret should be at least 32 characters for security
    assert!(secret.len() >= 32);

    // Calling again should return the same secret
    let secret2 = home_registry::auth::get_or_init_jwt_secret();
    assert_eq!(secret, secret2);
}

#[test]
fn test_password_hash_uniqueness() {
    let password = "test_password";

    let hash1 = home_registry::auth::hash_password_sync(password).expect("Failed to hash password");
    let hash2 = home_registry::auth::hash_password_sync(password).expect("Failed to hash password");

    // Hashes should be different due to different salts
    assert_ne!(hash1, hash2);

    // But both should verify against the same password
    assert!(home_registry::auth::verify_password_sync(password, &hash1).unwrap());
    assert!(home_registry::auth::verify_password_sync(password, &hash2).unwrap());
}
