// Dummy list of locations
const locations = ["Newfs", "CJs", "Olfs", "Barcade", "Corbys", "Linebacker"];

// Function to load locations with a check-in button
function loadLocations() {
    const locationList = document.getElementById("location-list");
    locations.forEach(location => {
        const li = document.createElement("li");
        li.textContent = location;
        
        const checkInButton = document.createElement("button");
        checkInButton.textContent = "+ Check In";
        checkInButton.onclick = function() {
            alert(`Checked in at ${location}`);
        };

        li.appendChild(checkInButton);
        locationList.appendChild(li);
    });
}

// Function to post a message
function postMessage() {
    const messageInput = document.getElementById("message-input");
    const messageList = document.getElementById("message-list");

    if (messageInput.value.trim() !== "") {
        const li = document.createElement("li");
        li.textContent = messageInput.value;
        messageList.appendChild(li);
        messageInput.value = "";
    } else {
        alert("Message cannot be empty");
    }
}

// Load locations on page load
window.onload = loadLocations;

