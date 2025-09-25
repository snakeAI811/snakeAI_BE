const express = require("express");
const path = require("path");

const app = express();
const PORT = process.env.PORT || 3000;

// Serve static files from "dist" or "build" folder
app.use(express.static(path.join(__dirname, "build"))); // or "build" if React

// Fallback for React Router (SPA) express v5
app.get("*splat", (req, res) => {
  res.sendFile(path.join(__dirname, "build", "index.html"));
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});