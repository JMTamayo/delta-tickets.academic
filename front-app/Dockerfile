# Use an official Node.js runtime as a parent image
FROM node:lts-alpine

# Set the working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json (or yarn.lock)
COPY package*.json ./

# Install dependencies
RUN npm install --force

# Copy the rest of your application code
COPY . .

# Expose the port that the app runs on
EXPOSE 3000

# Command to run the app
CMD ["npm", "start"]