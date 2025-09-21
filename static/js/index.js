
/// Function to make a POST request to the server to create a new project
const createProject = (name, github_url) => {
    // Send the content to the server via an HTTP POST request
    fetch('/create-project', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name, github_url }),
    })
    .then(response => {
        if (response.ok) {
            console.log("Project created")
        } else {
            console.log("Could not create the project")
        }
    })
    .catch(error => {
        console.error('Error:', error);
        alert('An error occurred while saving content to the server.');
    });
}