--ACCOUNT CREATION 

CREATE TABLE IF NOT EXISTS users (
    userID INT AUTO_INCREMENT Primary Key,
    firstName VARCHAR(50) NOT NULL,
    lastName VARCHAR(50) NOT NULL,
    userName VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    dateOfBirth DATE NOT NULL,
    creation_date TIMESTAMP DEFAULT Current_TIMESTAMP,
    last_login TIMESTAMP NULL,
    status ENUM('active', 'pending', 'deactivated') DEFAULT 'pending',
    role ENUM('admin', 'user', 'guest') DEFAULT 'user',
    profile_picture_url VARCHAR(255) NULL,
    password_reset_token VARCHAR(255) NULL,
    email_verified BOOLEAN DEFAULT FALSE,
);

--PRAYERWALL POSTS

CREATE TABLE IF NOT EXISTS prayerwall {
    prayerID INT AUTO_INCREMENT Primary Key,
    userID INT NOT NULL,
    content TEXT NOT NULL, 
    image_url VARCHAR(255) NULL, 
    external_link VARCHAR(255), NULL,
    posted TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP NULL,
    visibility ENUM('public', 'private', 'restricted') DEFAULT 'public',
    FOREIGN KEY (userID) REFERENCES users(userID)
};

CREATE TABLE IF NOT EXISTS prayer_updated {
    updateID INT AUTO_INCREMENT PRIMARY KEY,
    prayerID INT NOT NULL,
    update_content TEXT NOT NULL,
    update_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (prayerID) REFERENCES prayerwall(prayerID)
};

CREATE TABLE IF NOT EXISTS prayer_subscriptions {
    subscriptionID INT AUTO_INCREMENT PRIMARY KEY,
    userID INT NOT NULL,
    prayerID INT NOT NULL,
    subscribed_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN Key (prayerID) REFERENCES prayerwall(prayerID) 
};

CREATE TABLE IF NOT EXISTS comments {
    commentID INT AUTO_INCREMENT PRIMARY KEY, 
    userID INT NOT NULL,
    prayerID INT NOT NULL,
    content TEXT NOT NULL,
    posted TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP NULL,
    FOREIGN KEY (userID) REFERENCES users(userId),
    FOREIGN KEY (prayerID) REFERENCES prayerwall(prayerID)
}

CREATE TABLE IF NOT EXISTS prayerwall_prays {
    prayID INT AUTO_INCREMENT PRIMARY KEY,
    userID INT NOT NULL,
    prayerID INT NOT NULL,
    prayer_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userID) REFERENCES users(userID),
    FOREIGN KEY (prayerID) REFERENCES prayerwall(prayerID)
};

SELECT * FROM prayerwall WHERE prayerID = [specific_prayer_id]; --Retrieve Original Prayer
SELECT * FROM prayer_updates WHERE prayerID = [specific_prayer_id] ORDER BY update_date; --Retrieving Prayer Updates


