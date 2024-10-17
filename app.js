// Dummy list of locations
const locations = ["Newfs", "CJs", "Olfs", "Barcade", "Corbys", "Linebacker"];
const peopleAtLocations = {
    "Newfs": [],
    "CJs": [],
    "Olfs": [],
    "Barcade": [],
    "Corbys": [],
    "Linebacker": []
}
// Function to load locations with a check-in button
function loadLocations() {
    const locationList = document.getElementById("location-list");
    locations.forEach(location => {
        const li = document.createElement("li");
        li.textContent = location;
        
        const checkInButton = document.createElement("button");
        checkInButton.textContent = "+ Check In";
        checkInButton.onclick = function() {
            //alert(`Checked in at ${location}`); // fake for now
            //checkIn(location);
            fake_check_in(location);
        };

        li.appendChild(checkInButton);
        locationList.appendChild(li);
    });
}
// Function to handle the check-in process
function fake_check_in(location) {
    const inputValue = document.getElementById("name-input").value.trim();

    if (inputValue !== "") {
        // Create a timestamp for the check-in (only time)
        const timestamp = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: true }); // Format as 4:20 PM

        // Add person with timestamp to the corresponding location in peopleAtLocations
        if (!peopleAtLocations[location]) {
            peopleAtLocations[location] = [];
        }

        // Check if the person is already checked in at this location
        if (!peopleAtLocations[location].some(person => person.name === inputValue)) {
            peopleAtLocations[location].push({ name: inputValue, time: timestamp });
            updatePeopleAtLocation(); // Update the displayed people list
            //alert(`${inputValue} checked in at ${location} at ${timestamp}`);
        } else {
            alert(`${inputValue} is already checked in at ${location}`);
        }

        // Clear the input field after check-in
        document.getElementById("name-input").value = "";
    } else {
        alert("Name cannot be empty");
    }
}

// Function to update the displayed list of people at each location, sorted by the number of people and recent check-in time
function updatePeopleAtLocation() {
    const locationPeopleList = document.getElementById("location-people-list");
    locationPeopleList.innerHTML = ""; // Clear the previous list

    // Create an array of locations with their people counts
    const locationCounts = locations.map(loc => ({
        location: loc,
        count: peopleAtLocations[loc].length,
        recentTime: peopleAtLocations[loc].length > 0 ? peopleAtLocations[loc][peopleAtLocations[loc].length - 1].time : null
    }));

    // Sort locations by count of people, then by recent check-in time
    locationCounts.sort((a, b) => {
        if (b.count === a.count) {
            return b.recentTime ? b.recentTime.localeCompare(a.recentTime) : -1; // Sort by most recent time
        }
        return b.count - a.count; // Sort by count of people
    });

    // Create a list of sorted locations
    locationCounts.forEach(({ location, count }) => {
        const li = document.createElement("li");
        li.textContent = `${location} (${count})`;

        // Add event listener to toggle display of people
        li.style.cursor = "pointer";
        li.onclick = function() {
            togglePeopleList(location);
        };

        // Create a nested list for people at this location
        const peopleList = document.createElement("ul");
        peopleList.style.display = "none"; // Initially hidden

        // Populate the list with names, timestamps, and a "Check Out" button
        peopleAtLocations[location].forEach((person, index) => {
            const personLi = document.createElement("li");
            personLi.textContent = `${person.name} (at ${person.time})`;

            // Create a Check Out button for each person
            const checkOutButton = document.createElement("button");
            checkOutButton.textContent = "Check Out";
            checkOutButton.onclick = function() {
                checkOut(location, index); // Pass location and the index of the person to be checked out
            };

            personLi.appendChild(checkOutButton);
            peopleList.appendChild(personLi);
        });

        li.appendChild(peopleList);
        locationPeopleList.appendChild(li);
    });
}

// Function to check a person out of a location
function checkOut(location, personIndex) {
    peopleAtLocations[location].splice(personIndex, 1); // Remove the person from the location by index
    updatePeopleAtLocation(); // Update the displayed people list
}


// Function to toggle the display of the people list for a location
function togglePeopleList(location) {
    const locationPeopleListItems = document.querySelectorAll("#location-people-list li");
    locationPeopleListItems.forEach(item => {
        const subList = item.querySelector("ul");
        if (subList && item.firstChild.textContent.startsWith(location)) {
            subList.style.display = subList.style.display === "none" ? "block" : "none";
        } else if (subList) {
            subList.style.display = "none"; // Hide other lists
        }
    });
}


// Function to toggle the display of the people list for a location
function togglePeopleList(location) {
    const locationPeopleListItems = document.querySelectorAll("#location-people-list li");
    locationPeopleListItems.forEach(item => {
        const subList = item.querySelector("ul");
        if (subList && item.firstChild.textContent.startsWith(location)) {
            subList.style.display = subList.style.display === "none" ? "block" : "none";
        } else if (subList) {
            subList.style.display = "none"; // Hide other lists
        }
    });
}


// Load locations on page load
window.onload = loadLocations;
